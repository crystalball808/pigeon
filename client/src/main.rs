use std::error::Error;

use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:8080").await?;
    let (reader, mut writer) = stream.split();

    let mut buf_reader = BufReader::new(reader);
    let mut buffer = String::new();

    let mut input_reader = BufReader::new(io::stdin());
    let mut input_buffer = String::new();

    loop {
        tokio::select! {
            // send message
            result = input_reader.read_line(&mut input_buffer) => {
                let bytes_read = result?;
                if bytes_read == 0 {
                    continue
                }
                writer.write_all(input_buffer.as_bytes()).await?;
                input_buffer.clear();
            }
            // print received message
            result = buf_reader.read_line(&mut buffer) => {
                let bytes_read = result?;
                if bytes_read == 0 {
                    continue
                }
                println!("message - {buffer}");
                buffer.clear();
            }
        }
    }
}
