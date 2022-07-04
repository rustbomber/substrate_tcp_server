//导入tcp相关的对象
use std::{
    io::{Read, Write},
    net::TcpListener,
};

//tcp server启动方法
pub fn start_server() {
    //在8080端口开启监听，接收client的连接
    let listener = TcpListener::bind("127.0.0.1:8080");
    //listener返回的是Result，因此这里用match进行匹配，进行错误处理
    match listener {
        Ok(server) => {
            println!("wait connection for client.");
            //处理客户的连接
            for incoming in server.incoming() {
                match incoming {
                    Ok(stream) => {
                        //处理客户端的数据
                        handle_client(stream);
                    }
                    Err(e) => 
                    //发错错误时，将错误打印的标准错误
                    eprintln!("{:?}", e),
                }
            }
        }
        Err(_) => {
            eprintln!("error")
        }
    }
}

//具体处理客户端来的请求
fn handle_client(mut stream: std::net::TcpStream) {
    //使用一个缓存来接收数据
    let mut data = [0; 1024];
    //不断读取数据流，直到结束
    while match stream.read(&mut data[..]) {
        Ok(i) => {
            //将数据反序列化，以便后续处理
            let content = String::from_utf8(data.to_vec());
            //返回的给客户的数据提示
            let mut msg = String::from("echo from server: ");
            //将上提示与客户发来的数据进行拼接
            msg.push_str(content.unwrap().as_str());
            println!("The message from client: {}", msg);
            //数据回发
            stream.write(msg.as_bytes());
            true
        }
        Err(ref e) => {
            println!("{}", e);
            false
        }
    } {}
}
