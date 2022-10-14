use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::prelude::*,
    io::BufReader,
    time::Duration,
    thread,
};

use multithreadedserver::html;
use multithreadedserver::config::Config;
use multithreadedserver::multithreading;

fn main() {
    let config = Config::new();
    let address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&address).unwrap();
    let pool = multithreading::ThreadPool::new(4);
    
   
   
    println!("listneing on {}", &address);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down");

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let default_value = String::from("");
    
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();


    let first_line = http_request
        .get(0)
        .unwrap_or(&default_value)
        .as_str();

    let (file_name, status, reason) = match first_line {
        "GET / HTTP/1.1" => ("index.html", 200, String::from("OK")),
        
        "GET /sleep HTTP/1.1" => {
            ("index.html", 200, String::from("OK"))
        },

        _ => ("404.html", 404, String::from("Not Found")),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let response = html::create_html_response(status, reason, contents);
    stream.write_all(response.as_bytes()).unwrap();
}
