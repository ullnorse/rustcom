#[derive(Clone, Debug)]
pub enum Message {
    // Connection
    TryConnect,
    Disconnect,

    // Shortcuts
    Cut,
    Copy,
    Paste,
    ClearReceiveText,

    // File recording
    StartRecording,
    StopRecording,
    RecordingData(String),

    // Logging
    ClearLogText,

    // Menu
    ShowAbout,
    CloseAbout,
    ShowLog,
    SetDefaultUi,
    CloseApplication,

    // Macros
    MacroClicked(String),

    // Main terminal
    DataReceived(String),
    DataForTransmit,

    Quit,
}
