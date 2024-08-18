use std::{io::Read, net::TcpStream};
use mysql::*;
use mysql::prelude::*;

fn main() -> std::io::Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:3306")? ;
    let mut buf = [0;512];
    while stream.read(&mut buf)? > 0{
        let data = String::from_utf8_lossy(&buf);
        println!("{}",data);
    }
    let pool = Pool::new();
    println!("Hello, world!");
    Ok(())
}
