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

    writer.write_all(b"fuck you").await?;

    loop {
        // send message
        input_reader.read_line(&mut input_buffer).await?;
        writer.write_all(input_buffer.as_bytes()).await?;
        input_buffer.clear();

        // receive message
        buf_reader.read_line(&mut buffer).await?;
        println!("message - {buffer}");
        buffer.clear();
    }
}
