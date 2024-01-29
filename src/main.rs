use std::error::Error;
use std::{io, thread, time};

use crossterm::execute;
use crossterm::{terminal::{enable_raw_mode, Clear, ClearType}};

mod ai;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let input = input();

    let thread = ai::create_thread().await?;
    let thread_id = thread.as_str();
    let _message = ai::send_message(thread_id, String::from("Hello")).await?;
    let run_response = ai::run_assistant(thread_id).await?;
    let run_id = run_response.id.as_str();
    let mut not_completed = true;

    while not_completed {
        let status_response = ai::get_assistant_status(thread_id, run_id).await?;

        if status_response.status == "in_progress" {
            not_completed = true;
            thread::sleep(time::Duration::from_secs(1));
        } else if status_response.status == "completed" {
            not_completed = false;
        } else {
            println!("{}", status_response.status);
            return Ok(());
        }
    }

    let response = ai::get_assistant_messages(thread_id).await?;
    response
        .iter()
        .filter(|message| message.role == "assistant")
        .for_each(|message| {
            let code = util::extract_code_block(message.content[0].text.value.as_str());
            println!("{:?}", code);
        });

    let delete_response = ai::delete_thread(thread_id).await?;
    println!("{}", delete_response.text().await?);
    Ok(())
}
