use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread::{self};
use std::time::Duration;
use webserver::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 创建 bytes 数组
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // starts_with 可以判断字符串的起始值
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // 倒计时
        // Duration 是时间间隔。
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    // as_bytes 实现字符串转 bytes 数组
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // 监听端口
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    // 创建线程池
    let pool = ThreadPool::new(4);

    // incoming 会返回 TcpStream，TcpStream 用于和 Client 交换数据。
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        // handle_connection(stream);

        pool.execute(|| handle_connection(stream))

        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
    }
}
