use md5;

pub fn create_html_response(status: u16, reason: String, contents: String) -> String {
    let headers = convert_headers_to_text(
        vec![
            Header { name: String::from("Content-Length"), value: contents.len().to_string()},
            Header { name: String::from("Server"), value: String::from("Basic Rust Server by github.com/abdulhannanali") },
            Header { name: String::from("E-Tag"), value: format!("{:?}", md5::compute(contents.as_bytes()))}
        ]
    );

    
    vec![
        format!("HTTP/1.1 {status} {reason}"),
        headers,
        contents
    ]
    .into_iter()
    .map(|line| format!("{line}\r\n"))
    .collect::<String>()
}

struct Header {
    name: String,
    value: String,
}



fn convert_headers_to_text(headers: Vec<Header>) -> String {
    headers
        .into_iter()
        .map(|Header {name, value}| format!("{name}: {value}\r\n"))
        .collect::<String>()
}