use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::io::{self, Read, Write};

// 处理流
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // 这里做一个 Buffer，用于 read 客户端的消息
    let mut buf = [0; 512];
    for _ in 0..1000 {
        // 读取客户端消息
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(())
        } 
        // 往回传数据里写入读取的数据
        stream.write(&buf[..bytes_read])?;
        // 当前线程 sleep 1s ，防止消息错乱
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // 绑定本地服务
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // 开一个线程数组来记录子线程
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // 遍历 listener 连接客户端
    for stream in listener.incoming() {
        // 用 match 匹配 stream  状态
        match stream {
            Ok(stream) => {
                // 子线程处理
                let handle = thread::spawn(move || {
                    // 用 match 匹配 handle_client 结果
                    match handle_client(stream) {
                        Ok(_) => println!(""),
                        Err(_) => println!("Error"),
                    }
                });
                // 将句柄加入线程管理数组
                thread_vec.push(handle);
            },
            Err(_) => println!("bad stream")
        }
    }

    // 等待每个线程结束
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
