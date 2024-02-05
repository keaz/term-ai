use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;

use crossterm::cursor::{self, position, MoveDown, MoveToColumn};
use crossterm::event::{
    poll, read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture,
    EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event, KeyCode,
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::style::{Attribute, Color, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, queue, style};
use tokio::runtime::Runtime;

use crate::ai::OpenAI;
use crate::util::extract_code_block;

pub struct Terminal {
    open_ai: OpenAI,
    runtime: Runtime,
}

impl Terminal {
    pub fn new(open_ai: OpenAI, runtime: Runtime) -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All))?;
        let supports_keyboard_enhancement = matches!(
            crossterm::terminal::supports_keyboard_enhancement(),
            Ok(true)
        );
        if supports_keyboard_enhancement {
            queue!(
                stdout,
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                )
            )?;
        }
        execute!(
            stdout,
            EnableBracketedPaste,
            EnableFocusChange,
            EnableMouseCapture,
        )?;

        Ok(Self { open_ai, runtime })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All))?;

        let supports_keyboard_enhancement = matches!(
            crossterm::terminal::supports_keyboard_enhancement(),
            Ok(true)
        );

        if supports_keyboard_enhancement {
            queue!(
                stdout,
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                )
            )?;
        }

        execute!(
            stdout,
            EnableBracketedPaste,
            EnableFocusChange,
            EnableMouseCapture,
        )?;

        if let Err(e) = self.print_events() {
            println!("Error: {:?}\r", e);
        }

        if supports_keyboard_enhancement {
            queue!(stdout, PopKeyboardEnhancementFlags)?;
        }

        execute!(
            stdout,
            DisableBracketedPaste,
            PopKeyboardEnhancementFlags,
            DisableFocusChange,
            DisableMouseCapture
        )?;

        let _ = disable_raw_mode();
        let open_ai = self.open_ai.clone();
        self.runtime.block_on(open_ai.delete_thread())?;
        Ok(())
    }

    fn write_output(&self, output: String) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::CurrentLine))?; // Clear current line
        execute!(stdout, Print(output))?;
        stdout.flush()?;
        Ok(())
    }

    fn print_events(&mut self) -> io::Result<()> {
        let mut current_input = String::new();
        loop {
            let event = read()?;

            match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char(char) => {
                        current_input.push(char);
                        self.write_output(current_input.clone())?;
                        if char == ' ' {
                            let completions = self.suggest_completions(&current_input);
                            self.display_completions(completions);
                        }
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter => {
                        println!("\nExecuting: {}", current_input);
                        current_input.clear();
                        self.draw_hr_bar()?;
                    }
                    KeyCode::Backspace => {
                        current_input.pop();
                        self.write_output(current_input.clone())?;
                    }
                    _ => {}
                },
                Event::Mouse(mouse_event) => {
                    //                println!("Mouse event: {:?}\r", mouse_event);
                }
                Event::Resize(x, y) => {
                    //                 let (original_size, new_size) = flush_resize_events((x, y));
                    //                  println!("Resize from: {:?}, to: {:?}\r", original_size, new_size);
                }
                Event::Paste(paste_event) => {
                    //                   println!("Paste event: {:?}\r", paste_event);
                }
                _ => {
                    //                   println!("Event: {:?}\r", event);
                }
            }
        }

        Ok(())
    }

    fn display_completions(&self, completions: Result<Vec<String>, String>) -> io::Result<()> {
        let mut stdout = io::stdout();
        match completions {
            Ok(completions) => {
                if completions.is_empty() {
                    return Ok(());
                }
                execute!(stdout, MoveDown(1))?;
                execute!(stdout, MoveToColumn(0))?;
                execute!(stdout, SetAttribute(Attribute::Dim))?;
                for completion in completions {
                    execute!(stdout, Print(format!("{} \t", completion)),)?;
                }
                execute!(stdout, SetAttribute(Attribute::Reset))?; // Reset the text attributes
                execute!(stdout, MoveToColumn(0))?;
                stdout.flush()?;
                self.draw_hr_bar()?;
            }
            Err(e) => {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Red),
                    Print("Command failed to execute:\n"),
                    Print(format!("{}\n", e)),
                    SetForegroundColor(Color::Reset) // Reset the color
                )?;
            }
        }
        Ok(())
    }

    fn suggest_completions(&mut self, input: &str) -> Result<Vec<String>, String> {
        let message = self
            .runtime
            .block_on(self.open_ai.send_message(input.to_string()));
        if message.is_err() {
            return Err("Failed to send message to OpenAI".to_string());
        }

        let mut open_ai = self.open_ai.clone();
        let assistant_run = self.runtime.block_on(open_ai.run_assistant());
        match assistant_run {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("Failed to run assistant: {:?}", e));
            }
        }

        let mut completed = false;
        while !completed {
            let run = self.runtime.block_on(open_ai.get_assistant_status());

            match run {
                Ok(run) => {
                    completed = run.status == "completed";
                }
                Err(e) => {
                    return Err(format!("Failed to get assistant status: {:?}", e));
                }
            }
        }

        let assistant_responses = self.runtime.block_on(open_ai.get_assistant_messages());

        let assistant_responses = match assistant_responses {
            Ok(assistant_responses) => assistant_responses,
            Err(e) => return Err(format!("Failed to get assistant messages: {:?}", e)),
        };

        self.open_ai = open_ai;

        if assistant_responses.is_empty() {
            return Ok(vec![]);
        }

        let responses = assistant_responses
            .iter()
            .map(|response| extract_code_block(response.content[0].text.value.as_str()))
            .filter(|response| response.is_ok())
            .filter_map(|response| response.unwrap())
            .collect();
        Ok(responses)
    }

    fn draw_hr_bar(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::CurrentLine))?;
        let (width, _) = terminal::size()?;
        execute!(stdout, style::Print("-".repeat(width as usize)))?;
        stdout.flush()?;
        println!();
        execute!(stdout, MoveToColumn(0))?;
        stdout.flush()?;
        Ok(())
    }
}

fn flush_resize_events(first_resize: (u16, u16)) -> ((u16, u16), (u16, u16)) {
    let mut last_resize = first_resize;
    while let Ok(true) = poll(Duration::from_millis(50)) {
        if let Ok(Event::Resize(x, y)) = read() {
            last_resize = (x, y);
        }
    }

    (first_resize, last_resize)
}
