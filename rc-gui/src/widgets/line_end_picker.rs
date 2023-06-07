use eframe::egui::{self, ComboBox, Response, Widget};

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub enum LineEnd {
    #[default]
    LF,
    CR,
    CrLf,
    None,
}

impl std::fmt::Display for LineEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineEnd::LF => write!(f, "LF"),
            LineEnd::CR => write!(f, "CR"),
            LineEnd::CrLf => write!(f, "CR + LF"),
            LineEnd::None => write!(f, "None"),
        }
    }
}

impl From<LineEnd> for &'static str {
    fn from(val: LineEnd) -> Self {
        match val {
            LineEnd::LF => "\n",
            LineEnd::CR => "\r",
            LineEnd::CrLf => "\r\n",
            LineEnd::None => "",
        }
    }
}

pub struct LineEndPicker<'a> {
    width: f32,
    line_end: &'a mut LineEnd,
}

impl<'a> LineEndPicker<'a> {
    const LINE_ENDS: [LineEnd; 4] = [LineEnd::LF, LineEnd::CR, LineEnd::CrLf, LineEnd::None];
    const ID: &'static str = "line_end";

    pub fn new(width: f32, line_end: &'a mut LineEnd) -> Self {
        Self {
            width,
            line_end,
        }
    }
}

impl<'a> Widget for LineEndPicker<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ComboBox::from_id_source(Self::ID)
            .selected_text(self.line_end.to_string())
            .width(self.width)
            .show_ui(ui, |ui| {
                for line_end in Self::LINE_ENDS {
                    ui.selectable_value(self.line_end, line_end, line_end.to_string());
                }
            })
            .response
    }
}
