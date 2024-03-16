use std::{env, fs};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use colored::Colorize;

fn main() {
    let port = env::args().nth(1).expect("port switch required!");
    let filename = env::args().nth(2).expect("html file required!");
    let end_point: String = "127.0.0.1".to_owned() + ":" + port.as_str();
    let listener = TcpListener::bind(end_point.clone()).unwrap();
    println!("{}", "Server is up!".green());
    println!("{}\n", end_point.blue());
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, filename.to_owned());
    }
}

fn handle_connection(mut stream: TcpStream, filename: String) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        println!("Request Status: {}", status_line.purple());
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap()
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("./src/404.html").unwrap();
        let length = contents.len();
        println!("Request Status: {}", status_line.red());
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
