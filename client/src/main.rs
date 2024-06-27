use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
    sync::mpsc::{self, Receiver},
};

mod app;
mod ui;

use app::{App, Message};
use ui::ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(async move {
        let mut stream = TcpStream::connect("localhost:8080").await.unwrap();
        let (reader, mut writer) = stream.split();

        let (reader, mut writer) = stream.split();

        let mut buf_reader = BufReader::new(reader);
        let mut buffer = String::new();

        loop {
            tokio::select! {
                // send message
                // result = input_reader.read_line(&mut input_buffer) => {
                //     let bytes_read = result?;
                //     if bytes_read == 0 {
                //         continue
                //     }
                //     writer.write_all(input_buffer.as_bytes()).await?;
                //     input_buffer.clear();
                // }
                // print received messageprint
                result = buf_reader.read_line(&mut buffer) => {
                    let bytes_read = result.unwrap();
                    if bytes_read == 0 {
                        continue
                    }
                    tx.send(Message { author_name: "Anon".to_owned(), content: buffer.clone()}).await.unwrap();
                    buffer.clear();
                }
            }
        }
    });

    // terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // create backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run in
    let mut app = App::with_mock();

    let _res = run_app(&mut terminal, &mut app, rx).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut receiver: Receiver<Message>,
) -> io::Result<()> {
    loop {
        if let Ok(message) = receiver.try_recv() {
            app.messages.push(message)
        }
        // draw ui
        terminal.draw(|frame| ui(frame, app))?;

        if let Ok(true) = event::poll(Duration::ZERO) {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Char(c) => {
                        app.input_value.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input_value.pop();
                    }
                    _ => {}
                }
            }
        }
    }
}

// let mut stream = TcpStream::connect("localhost:8080").await?;
//
// let (reader, mut writer) = stream.split();
//
// let mut buf_reader = BufReader::new(reader);
// let mut buffer = String::new();
//
// let mut input_reader = BufReader::new(io::stdin());
// let mut input_buffer = String::new();
//
// loop {
//     tokio::select! {
//         // send message
//         result = input_reader.read_line(&mut input_buffer) => {
//             let bytes_read = result?;
//             if bytes_read == 0 {
//                 continue
//             }
//             writer.write_all(input_buffer.as_bytes()).await?;
//             input_buffer.clear();
//         }
//         // print received message
//         result = buf_reader.read_line(&mut buffer) => {
//             let bytes_read = result?;
//             if bytes_read == 0 {
//                 continue
//             }
//             println!("message - {buffer}");
//             buffer.clear();
//         }
//     }
// }
