use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7788").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // 将数据读入缓存
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let line = "HTTP/1.1 200 OK\r\n\r\n";
        let contents = r#"{usename: "chenxing", password: 123}"#;
        let response = format!(
            "{}Content-Length: {}\r\n\r\n{}",
            line,
            contents.len(),
            contents
            
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }


}
