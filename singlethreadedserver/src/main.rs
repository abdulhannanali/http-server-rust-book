mod html;
mod config;

use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::prelude::*,
    io::BufReader,
};

use html::*;
use config::Config;


fn main() {
    let config = Config::new();
    let address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&address).unwrap();
    println!("listneing on {}", &address);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (file_name, status, reason) = if http_request.contains(&String::from("GET / HTTP/1.1")) {
        ("index.html", 200, String::from("OK"))
    } else {
        ("404.html", 404, String::from("Not Found"))
    };
    
    let contents = fs::read_to_string(file_name).unwrap();
    let response = create_html_response(status, reason, contents);
    stream.write_all(response.as_bytes()).unwrap();
}

