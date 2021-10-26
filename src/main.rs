use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};

use std::thread;
use std::time;

fn main() {
    // server thread
    let handle_server = thread::spawn(move || {
        // print log
        println!("[Server]: start ...");
        // call worker_server fn
        worker_server();
        // print log
        println!("[Server]: close ...");
    });
    // client thread
    let handle_client = thread::spawn(move || {
        // delay
        thread::sleep(time::Duration::from_millis(200));
        println!("[Client]: start ...");
        worker_client();
        println!("[Client]: close ...");
    });
    // join waiting for all threads to finish
    handle_server.join().unwrap();
    handle_client.join().unwrap();
}
// server process
fn worker_server() {
    // server bind ip:port
    let listener = TcpListener::bind("127.0.0.1:8080");
    // listener is ok or not
    if !listener.is_ok() {
        println!("[Server]: Bind ip and port fail ...");
        return;
    }
    let listener = listener.unwrap();
    println!("[Server]: Waiting for net message ...");
    // get stream from listener.incoming()
    for stream in listener.incoming() {
        // stream is ok
        if !stream.is_ok() {
            println!("[Server]: Get error message ...");
            continue;
        }
        // use unwrap resolve error
        let stream = stream.unwrap();
        // call process_stream fn
        if false == process_stream(stream) {
            println!("[Server]: Process error message ...");
            continue;
        }
        // print log
        println!("[Server]: Waiting for net message ...");
    }
}
// server stream process
fn process_stream(mut stream: TcpStream) -> bool {
    // mutable buffer for store the message from client
    let mut buffer = [0; 512];
    // if stream err then return false
    if !stream.read(&mut buffer).is_ok() {
        return false;
    }
    // print message receive from client
    println!(
        "[Server][process_stream] Get request info: \"{}\"",
        String::from_utf8_lossy(&buffer[..])
    );
    // echo messge to client
    if !stream
        .write(b"Server has received your request ...")
        .is_ok()
    {
        return false;
    }
    return true;
}
// client process
fn worker_client() {
    // client connet ip:port
    let stream = TcpStream::connect("127.0.0.1:8080");
    // stream is ok or not
    if !stream.is_ok() {
        println!("[Client]: Bind fail ...");
        return;
    }
    let mut stream = stream.unwrap();
    // send message to server
    let status = stream.write(b"client send info to server!");
    if !status.is_ok() {
        println!("[Client]: Send info fail ...");
        return;
    }
    // mutable buffer
    let mut buffer = [0; 512];
    // read message to buffer
    let status = stream.read(&mut buffer);
    if !status.is_ok() {
        println!("[Client]: Receive info fail ...");
        return;
    }
    println!(
        "[Client]: Get msg from server \"{}\"",
        String::from_utf8_lossy(&buffer[..])
    );
    // shutdown stream
    match stream.shutdown(Shutdown::Both) {
        Ok(is_ok) => {
            return is_ok;
        }
        Err(is_err) => {
            println!("stream shutdown {}", is_err);
        }
    }
}
