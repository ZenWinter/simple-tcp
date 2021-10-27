//将std::io::prelude引入作用域来获取读写流所需的特定trait
use std::io::prelude::*;
//使用标准库的std::net处理流内容
use std::net::TcpStream;
//使用标准库的std::net监听TCP连接
use std::net::TcpListener;

fn main() {
    //在127.0.0.1:7878地址开始监听
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //获取传入的流，incoming 方法返回一个迭代器，代表一系列的流
    for stream in listener.incoming() {
        //获取一个stream实例
        let stream = stream.unwrap();

        //流内容读取
        handle_connection(stream);
    }
}

//流内容读取函数
fn handle_connection(mut stream: TcpStream) {
    //声明一个buffer存放读取到的数据，1024字节
    let mut buffer = [0; 1024];

    //读取流内容到buffer
    stream.read(&mut buffer).unwrap();

    //String::from_utf8_lossy函数获取一个 &[u8] 并产生一个 String，并处理无效序列
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}