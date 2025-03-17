use std::{
    fs,
    io::{prelude::*,BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_slow_response(stream);
    }
}

// Commit 3 related
fn handle_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
 
    if request_line == "GET / HTTP/1.1" {
        send_response("HTTP/1.1 200 OK", "hello.html", &mut stream);
    } else {
        send_response("HTTP/1.1 404 NOT FOUND", "404.html", &mut stream);
    }
}

// Commit 4 related
fn handle_slow_response(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    send_response(status_line, file_name, &mut stream);
}

fn send_response(status_line: &str, file_path: &str, stream: &mut TcpStream) {
    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}