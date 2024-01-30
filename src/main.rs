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

    let open_ai = runtime
        .block_on(ai::OpenAI::from(
            "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".to_string(),
            "asst_T9pmKDQnnONxBqtI9Yud2Onc".to_string(),
        ))
        .unwrap();
    let mut terminal = terminal::Terminal::new(open_ai, runtime).unwrap();
    terminal.run();
}
