mod answer;
mod app;
mod chunk;
mod embed;
mod ingest;
mod retrieve;
mod rig;
mod store;
mod web;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(|value| value.as_str()) {
        None | Some("--serve") => {
            let addr = args
                .get(1)
                .map(|value| value.as_str())
                .unwrap_or("127.0.0.1:7878");

            if let Err(err) = web::run(Some(addr)) {
                eprintln!("failed to start web UI: {err}");
                std::process::exit(1);
            }
        }
        Some("--cli") => {
            let question = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
            let question = if question.trim().is_empty() {
                "Can I expense a hotel minibar?".to_string()
            } else {
                question
            };

            let report = app::answer_question(&question);
            println!("{}", report.plain_text());
        }
        Some(_) => {
            let question = args.join(" ");
            let report = app::answer_question(&question);
            println!("{}", report.plain_text());
        }
    }
}
