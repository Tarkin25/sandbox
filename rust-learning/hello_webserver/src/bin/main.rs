use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use hello_webserver::ThreadPool;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let ok = ("HTTP/1.1 200 OK", "hello.html");

    let (status_line, filename) = if buffer.starts_with(get) {
        ok
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ok
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    let content = fs::read_to_string(format!("resources/{}", filename)).unwrap();

    let headers = format!("Content-Type: text/html\r\nContent-Length: {}", content.len());

    let response = format!("{}\r\n{}\r\n\r\n{}", status_line, headers, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
