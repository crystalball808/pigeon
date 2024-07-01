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
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};

mod app;
mod ui;

use app::{App, Message};
use ui::ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (in_tx, in_rx) = mpsc::channel::<Message>(32);
    let (out_tx, mut out_rx) = mpsc::channel::<Message>(32);
    tokio::spawn(async move {
        let mut stream = TcpStream::connect("localhost:8080").await.unwrap();

        let (reader, mut writer) = stream.split();

        let mut buf_reader = BufReader::new(reader);
        let mut buffer = String::new();

        loop {
            tokio::select! {
                message = out_rx.recv() => {
                    if let Some(message) = message {
                        if let Err(error) = writer.write_all(message.content.as_bytes()).await { println!("{error}")};
                    }
                }
                // print received messageprint
                result = buf_reader.read_line(&mut buffer) => {
                    let bytes_read = result.unwrap();
                    if bytes_read == 0 {
                        continue
                    }
                    in_tx.send(Message { author_name: "Anon".to_owned(), content: buffer.clone()}).await.unwrap();
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

    let _res = run_app(&mut terminal, &mut app, in_rx, out_tx).await;

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
    sender: Sender<Message>,
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
                    KeyCode::Enter => {
                        if app.input_value.is_empty() == false {
                            app.input_value.push_str("\n");
                            let message = Message {
                                author_name: "Client".to_owned(),
                                content: app.input_value.clone(),
                            };
                            sender.send(message).await.unwrap();
                            app.input_value.clear();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
