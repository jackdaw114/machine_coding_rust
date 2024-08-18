use std::io::{self, BufRead, Write,Read};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:80")?;
    let mut write_buf = String::new();
    let mut buffer = [0;512];
    

    loop {
        // Read input from stdin

        print!("Enter message: ");
        io::stdout().flush()?;
        write_buf.clear();
        io::stdin().read_line(&mut write_buf).unwrap();

        stream.write_all(write_buf.as_bytes())?;
        
       
        let bytes_read = stream.read(&mut buffer)?;
       
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        // Print the response
        print!("{}", response);

        if bytes_read == 0 {
            println!("Connection dropped");
            break;
        }
    }
    
    Ok(())
}
