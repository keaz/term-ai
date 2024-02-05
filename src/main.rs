use tokio::runtime::Builder;

mod ai;
mod terminal;
mod util;

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let api_key = std::env::var("OPEN_AI_API_KEY").unwrap();
    let assistant_id = std::env::var("OPEN_AI_ASSISTANT_ID").unwrap();
    let open_ai = runtime.block_on(ai::OpenAI::from(api_key, assistant_id));

    let open_ai = match open_ai {
        Ok(open_ai) => open_ai,
        Err(e) => {
            eprintln!("Failed to create OpenAI instance: {}", e);
            return;
        }
    };
    let mut terminal = terminal::Terminal::new(open_ai, runtime).unwrap();
    let _ = terminal.run();
}
