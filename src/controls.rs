use crate::game::GameState;
use crossterm::event::{self, Event, KeyCode};
use tokio;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;

pub enum Controls {
    Continue,
    Quit,
}

pub enum InputMode {
    Editing,
    Normal,
}

pub fn send_key_codes(keys_tx: Sender<KeyCode>) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            match event::read() {
                Ok(Event::Key(key)) => {
                    if keys_tx.send(key.code).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("event::read() err: {e}");
                    break;
                }
                _ => {}
            }
        }
    })
}

pub fn controls(state: &mut GameState) -> anyhow::Result<Controls> {
    if let Ok(code) = state.keys_rx.try_recv() {
        match state.input_mode {
            InputMode::Normal => match code {
                KeyCode::Char('i') => {
                    state.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    return Ok(Controls::Quit);
                }
                _ => {}
            },
            InputMode::Editing => match code {
                KeyCode::Enter => {
                    let _msg: String = state.input.drain(..).collect();
                }
                KeyCode::Char(c) => {
                    state.input.push(c);
                }
                KeyCode::Backspace => {
                    state.input.pop();
                }
                KeyCode::Esc => {
                    state.input_mode = InputMode::Normal;
                }
                _ => {}
            },
        }
    }
    Ok(Controls::Continue)
}
