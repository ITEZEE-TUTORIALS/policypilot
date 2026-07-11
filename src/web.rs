use std::fmt::Write as _;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use crate::app;
use crate::answer::AnswerReport;
use crate::ingest;

const DEFAULT_ADDR: &str = "127.0.0.1:7878";
const INDEX_HTML: &str = include_str!("../assets/ui/index.html");
const STYLES_CSS: &str = include_str!("../assets/ui/styles.css");
const APP_JS: &str = include_str!("../assets/ui/app.js");

pub fn run(addr: Option<&str>) -> io::Result<()> {
    let bind_addr = addr.unwrap_or(DEFAULT_ADDR);
    let listener = TcpListener::bind(bind_addr)?;

    println!("PolicyPilot web UI running at http://{bind_addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_connection(stream) {
                    eprintln!("request error: {err}");
                }
            }
            Err(err) => eprintln!("connection error: {err}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut request_line = String::new();
    {
        let mut reader = BufReader::new(&mut stream);
        reader.read_line(&mut request_line)?;
        if request_line.trim().is_empty() {
            return Ok(());
        }

        let mut header_line = String::new();
        loop {
            header_line.clear();
            let bytes_read = reader.read_line(&mut header_line)?;
            if bytes_read == 0 || header_line == "\r\n" || header_line == "\n" {
                break;
            }
        }
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    let path = target.split_once('?').map(|(path, _)| path).unwrap_or(target);

    let response = match (method, path) {
        ("GET", "/") => http_response(200, "text/html; charset=utf-8", INDEX_HTML),
        ("GET", "/styles.css") => http_response(200, "text/css; charset=utf-8", STYLES_CSS),
        ("GET", "/app.js") => http_response(200, "application/javascript; charset=utf-8", APP_JS),
        ("GET", "/health") => http_response(200, "text/plain; charset=utf-8", "ok"),
        ("GET", "/api/policies") => {
            let body = render_policies_json();
            http_response(200, "application/json; charset=utf-8", &body)
        }
        ("GET", "/api/answer") => {
            let question = query_param(target, "question")
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| app::SAMPLE_QUESTIONS[0].to_string());
            let report = app::answer_question(&question);
            let body = render_report_json(&report);
            http_response(200, "application/json; charset=utf-8", &body)
        }
        _ => http_response(404, "text/plain; charset=utf-8", "Not found"),
    };

    stream.write_all(response.as_bytes())?;
    stream.flush()
}

fn http_response(status: u16, content_type: &str, body: &str) -> String {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "OK",
    };

    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn render_report_json(report: &AnswerReport) -> String {
    let mut json = String::new();
    json.push('{');
    push_json_field(&mut json, "question", &report.question);
    json.push(',');
    push_json_field(&mut json, "answer", &report.answer);
    json.push(',');
    push_json_field(&mut json, "relevant_excerpt", &report.relevant_excerpt);
    json.push(',');
    json.push_str("\"top_match_title\":");
    match &report.top_match_title {
        Some(title) => push_json_string(&mut json, title),
        None => json.push_str("null"),
    }
    json.push(',');
    json.push_str("\"sources\":[");

    for (index, source) in report.sources.iter().enumerate() {
        if index > 0 {
            json.push(',');
        }

        json.push('{');
        push_json_field(&mut json, "document_id", &source.document_id);
        json.push(',');
        push_json_field(&mut json, "title", &source.title);
        json.push(',');
        json.push_str("\"section\":");
        match &source.section {
            Some(section) => push_json_string(&mut json, section),
            None => json.push_str("null"),
        }
        json.push(',');
        json.push_str("\"text\":");
        push_json_string(&mut json, &source.text);
        json.push(',');
        json.push_str("\"score\":");
        let _ = write!(json, "{:.6}", source.score);
        json.push('}');
    }

    json.push(']');
    json.push('}');
    json
}

fn render_policies_json() -> String {
    let documents = ingest::load_demo_documents();
    let mut json = String::from("[");

    for (index, document) in documents.iter().enumerate() {
        if index > 0 {
            json.push(',');
        }

        json.push('{');
        push_json_field(&mut json, "id", document.id);
        json.push(',');
        push_json_field(&mut json, "title", document.title);
        json.push(',');
        push_json_field(&mut json, "body", document.body);
        json.push('}');
    }

    json.push(']');
    json
}

fn push_json_field(json: &mut String, key: &str, value: &str) {
    push_json_string(json, key);
    json.push(':');
    push_json_string(json, value);
}

fn push_json_string(json: &mut String, value: &str) {
    json.push('"');

    for ch in value.chars() {
        match ch {
            '"' => json.push_str("\\\""),
            '\\' => json.push_str("\\\\"),
            '\n' => json.push_str("\\n"),
            '\r' => json.push_str("\\r"),
            '\t' => json.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(json, "\\u{:04x}", c as u32);
            }
            c => json.push(c),
        }
    }

    json.push('"');
}

fn query_param(target: &str, key: &str) -> Option<String> {
    let query = target.split_once('?')?.1;

    for pair in query.split('&') {
        let (raw_key, raw_value) = pair.split_once('=').unwrap_or((pair, ""));
        if raw_key == key {
            return Some(url_decode(raw_value));
        }
    }

    None
}

fn url_decode(value: &str) -> String {
    let mut output = String::new();
    let bytes = value.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                output.push(' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let hex = &value[index + 1..index + 3];
                if let Ok(parsed) = u8::from_str_radix(hex, 16) {
                    output.push(parsed as char);
                    index += 3;
                } else {
                    output.push('%');
                    index += 1;
                }
            }
            byte => {
                output.push(byte as char);
                index += 1;
            }
        }
    }

    output
}
