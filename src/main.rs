use std::error::Error;

mod ai;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let api_key = "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"; // Replace with your API key

    //    let thread = client
    //        .post("https://api.openai.com/v1/threads")
    //        .header("Content-Type", "application/json")
    //        .header("Authorization", format!("Bearer {}", api_key))
    //        .header("OpenAI-Beta", "assistants=v1")
    //        .send()
    //        .await?;
    //    println!("{}", thread.text().await?);
    //"thread_OYRcF3JXKrQdVdBNDoD54zHE"
    //
    //    let message = OpenAIMessage::new(
    //        "user".to_string(),
    //        "Generate rust code to read user inpunt from terminal".to_string(),
    //    );
    //    let message_request = client
    //        .post("https://api.openai.com/v1/threads/thread_OYRcF3JXKrQdVdBNDoD54zHE/messages")
    //        .header("Content-Type", "application/json")
    //        .header("Authorization", format!("Bearer {}", api_key))
    //        .header("OpenAI-Beta", "assistants=v1")
    //        .json(&message)
    //        .send()
    //        .await?;
    //
    //    println!("{}", message_request.text().await?);
    //asst_hqAXpyDJ4f9f9i3IFv0zne7d
    //
    //let run_message = OpenAIRun::new("asst_hqAXpyDJ4f9f9i3IFv0zne7d".to_string(), None);
    //    let run_assistant = client
    //        .post("https://api.openai.com/v1/threads/thread_OYRcF3JXKrQdVdBNDoD54zHE/runs")
    //        .header("Content-Type", "application/json")
    //        .header("Authorization", format!("Bearer {}", api_key))
    //        .header("OpenAI-Beta", "assistants=v1")
    //        .json(&run_message)
    //        .send()
    //        .await?;
    //
    //    println!("{}", run_assistant.text().await?);
    //    let status_request = client.get("https://api.openai.com/v1/threads/thread_OYRcF3JXKrQdVdBNDoD54zHE/runs/run_6LkoDN57Pz5ithVfR6I2PvfK")
    //        .header("Content-Type", "application/json")
    //        .header("Authorization", format!("Bearer {}", api_key))
    //        .header("OpenAI-Beta", "assistants=v1")
    //        .send().await?;
    //
    //    println!("{}", status_request.text().await?);
    //    let response = client
    //        .get("https://api.openai.com/v1/threads/thread_OYRcF3JXKrQdVdBNDoD54zHE/messages")
    //        .header("Content-Type", "application/json")
    //        .header("Authorization", format!("Bearer {}", api_key))
    //        .header("OpenAI-Beta", "assistants=v1")
    //        .send()
    //        .await?;
    //
    //    let response = response.json::<ai::OpenAIMessagesResponse>().await.unwrap();
    //    println!("{:?}", response.data[0].content[0].text.value);
    let text = "Sure! Here's an example of Rust code that reads user input from the terminal:\n\n```rust\nuse std::io;\n\nfn main() {\n    // Create a mutable string variable to store the user input\n let mut input = String::new();\n\n    // Read user input from the terminal\n    match io::stdin().read_line(&mut input) {\n        Ok(_) => {\n // If reading was successful, print the user input\n            println!(\"You entered: {}\", input);\n        }\n        Err(error) => {\n            // If an error occurred, print the error message\n            eprintln!(\"Error reading input: {}\", error);\n        }\n    }\n}\n```\n\nThis code uses the `std::io` module to read the user input from the terminal. It creates a mutable `String` variable `input` to store the user input. The `read_line` function is used to read the input into the `input` variable. If the reading is successful, it prints the user input. If an error occurs, it prints the error message.";
    let code = util::extract_code_block(text);
    println!("{:?}", code);

    Ok(())
}
