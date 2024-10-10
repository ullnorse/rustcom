
#[derive(Clone, Debug)]
pub enum Message {
    // Connection
    TryConnect,
    Connected,
    TryDisconnect,
    Disconnected,

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