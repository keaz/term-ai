use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;

use crossterm::cursor::{self, position, MoveToColumn};
use crossterm::event::{
    poll, read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture,
    EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event, KeyCode,
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
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
        Ok(())
    }

    fn write_output(&self, output: String) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::CurrentLine))?; // Clear current line
        print!("{}", output);
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
                            println!("\nCompletions: {:?}", completions);
                        }
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter => {
                        println!("\nExe39jcuting: {}", current_input);
                        current_input.clear();
                        let pos = position()?;
                        println!("Current position: {:?}\r", pos);
                    }
                    KeyCode::Backspace => {
                        current_input.pop();
                        self.write_output(current_input.clone())?;
                    }
                    _ => {}
                },
                Event::Mouse(mouse_event) => {
                    println!("Mouse event: {:?}\r", mouse_event);
                }
                Event::Resize(x, y) => {
                    let (original_size, new_size) = flush_resize_events((x, y));
                    println!("Resize from: {:?}, to: {:?}\r", original_size, new_size);
                }
                Event::Paste(paste_event) => {
                    println!("Paste event: {:?}\r", paste_event);
                }
                _ => {
                    println!("Event: {:?}\r", event);
                }
            }
        }

        Ok(())
    }

    fn suggest_completions(&mut self, input: &str) -> Vec<String> {
        let _message = self
            .runtime
            .block_on(self.open_ai.send_message(input.to_string()))
            .unwrap();

        let mut open_ai = self.open_ai.clone();
        self.runtime.block_on(open_ai.run_assistant()).unwrap();

        let mut completed = false;
        while !completed {
            let run = self
                .runtime
                .block_on(open_ai.get_assistant_status())
                .unwrap();
            completed = run.status == "completed";
        }

        let assistant_responses = self
            .runtime
            .block_on(open_ai.get_assistant_messages())
            .unwrap();

        self.open_ai = open_ai;

        if assistant_responses.is_empty() {
            return vec![];
        }

        assistant_responses
            .iter()
            .map(|response| extract_code_block(response.content[0].text.value.as_str()).unwrap())
            .collect()
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
