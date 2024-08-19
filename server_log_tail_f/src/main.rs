use std::{collections::VecDeque, fmt::format, fs::{read, File, OpenOptions}, io::{self, BufRead, BufReader, Error, Read, Seek, SeekFrom, Write}, net::{TcpListener, TcpStream}, path::Path, sync::{Arc, Condvar, Mutex}, thread, time::Duration};



fn main()-> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:80")?;
   
    let mut update_count = Arc::new((Mutex::new(0),Condvar::new())); 
    for stream in listener.incoming(){
        match stream{

            Ok(n) =>  {  
                let counter = Arc::clone(&update_count);
                thread::spawn(move || handle_client(n,counter));},
            Err(e) => println!("failed to accept request {}",e),
        }
    }


    Ok(())
}

fn discard_buffer_data(n:&mut BufReader<&TcpStream>) -> io::Result<()>{
    let mut buffer = String::new();
    while n.read_line(&mut buffer)? > 0{
        buffer.clear();
    }

    Ok(())

}

fn handle_client(mut n:TcpStream, update_count: Arc<(Mutex<i32>,Condvar)>){
    let mut reader = BufReader::new(&n);
    loop {
        let mut buf= String::new();
        reader.read_line(&mut buf);

        let mut iter = buf.split_whitespace();
        
        loop{
            match iter.next(){
                Some(n) => {
                    if n == "GET"{
                        match iter.next(){
                            Some(n) => {

                                println!("{}",n); // branch here 
                                match n{
                                    "/write_file" =>{
                                        println!("server at write_file");
                                        match reader.get_mut().write_all(b"recieved"){
                                    
                                        Ok(_) => {println!("ok");},
                                        Err(e) => {println!("err:{}",e);}
                                        }
                                        reader.get_mut().flush();       
                                          
                                        let test = reader.get_mut(); 
                                        write_file(test,&update_count);
                                    },
                                    "/stream_file" =>{
                                        println!("server at read file");

    let response_headers = 
        format!("HTTP/1.1 200 OK\r\n\
        Content-Type: text/event-stream\r\n\
        Cache-Control: no-cache\r\n\
        Connection: keep-alive\r\n\
        \r\nhello");
                                        match reader.get_mut().write_all(response_headers.as_bytes()){
                                    
                                        Ok(_) => {println!("ok");},
                                        Err(e) => {println!("err:{}",e);}
                                        }
                                        reader.get_mut().flush();       
                                          
                                        let test = reader.get_mut(); 
                                        stream_file(test,&update_count);
                                    }
                                    x =>{
                                        match reader.get_mut().write_all(format!("path for {} does not exist",x).as_bytes()){
                                    
                                        Ok(_) =>{ 
                                            println!("path for {} does not exist",x);
                                            
                                        },

                                        Err(e) => {println!("err:{}",e);}
                                        }
                                        break;
                                    }
                                }
                            }
                            None => {break;}
                        }
                    }
                },
                None => {break;},
            }

        }

        
        if buf.is_empty() {
            println!("stream end");
            break;
        }
    }

    

}

fn write_file(stream:&mut &TcpStream,update_count: &Arc<(Mutex<i32>,Condvar)>) -> std::io::Result<()>{
    let path = Path::new("./test.txt");
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;


    loop{

        let mut buffer = [0;512];
        let mut bytes_read=0;
        match stream.read(&mut buffer){
            Ok(n) => {
                bytes_read=n;
                let (temp_count,cvar) = &**update_count;
                let mut temp_count = temp_count.lock().unwrap();
                *temp_count+=1;
                cvar.notify_all();
            },
            Err(e) => {
                println!("failed to read {}",e);
            }
        }
        if bytes_read == 0{
            println!("file write end");
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        stream.write_all(b"file appended")?;
        println!("file written")
    }
    Ok(())
}

fn stream_file(stream:&mut &TcpStream,update_count: &Arc<(Mutex<i32>,Condvar)>) -> std::io::Result<()>{
    let response_headers = 
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/event-stream\r\n\
        Cache-Control: no-cache\r\n\
        Connection: keep-alive\r\n\
        \r\n
        ";
   

    stream.write_all(response_headers.as_bytes())?;
    stream.flush();
    let path = Path::new("./test.txt");
    let file = File::open(path)?;

    let to_write = read_n_lines(&file,10)?; 
    stream.write_all(to_write.as_bytes())?;
    stream.flush()?;

    loop{
        let (lock,cvar) = &**update_count;
        let mut mutex_guard = lock.lock().unwrap();

        while *mutex_guard ==0{
            mutex_guard = cvar.wait_timeout(mutex_guard, Duration::from_secs(1)).unwrap().0;
        }
        let to_write = read_n_lines(&file,1)?; 
        *mutex_guard=0;
        let message = format!("\n{}",&to_write);

        stream.write_all(message.as_bytes())?;
        stream.flush()?;
    }
}

fn read_n_lines(file:&File,lines:usize) -> std::io::Result<String>{
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(0));
    println!("{}",lines);
    let mut position = reader.stream_position()?; 
    let mut output = VecDeque::new();

    println!("{}",position);




    while output.len() < lines+1 && position > 0{
        position = position.saturating_sub(1);

        reader.seek(io::SeekFrom::Start(position))?;
        let mut line = String::new();

        let mut char_buffer= [0;1];
        reader.read(&mut char_buffer)?;
        
        let char_curr = String::from_utf8_lossy(&char_buffer);
        

        if position > 0 && char_curr == "\n" {
            position = position.saturating_add(1);
            reader.seek(SeekFrom::Start(position))?;
            reader.read_line(&mut line)?;
            output.push_front(line.trim().to_string());
            position = position.saturating_sub(2);
            continue;
        }
    }
    let str_out:String = output.into_iter().map(|n| n+"\n").collect();
    let str_out = str_out.trim().to_string();
    println!("output {}",str_out);
    Ok(str_out) 
}
