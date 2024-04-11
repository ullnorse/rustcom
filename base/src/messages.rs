
use std::sync::Mutex;
use std::cell::OnceCell;
use flume::{unbounded, Sender, Receiver};

static CHANNEL: Mutex<OnceCell<(Sender<Message>, Receiver<Message>)>> = Mutex::new(OnceCell::new());

#[derive(Clone, Debug)]
pub enum Message {
    // Connection
    Connect,
    Disconnect,

    // Shortcuts
    Cut,
    Copy,
    Paste,

    // File recording
    StartRecording,
    StopRecording,
    RecordingData(String),

    // Logging
    ClearLogText,
    // Log(Entry),

    // Menu
    ShowAbout,
    CloseAbout,
    SetDefaultUi,
    CloseApplication,

    // Main terminal
    DataReceived(String),
    DataForTransmit(String),
    ClearTerminalText,
}

pub fn init() {
    CHANNEL.lock().unwrap().get_or_init(unbounded);
}

pub fn send(message: Message) {
    CHANNEL.lock().unwrap().get().unwrap().0.send(message).unwrap();
}

pub fn try_receive() -> Option<Message> {
    CHANNEL.lock().unwrap().get().unwrap().1.try_recv().ok()
}