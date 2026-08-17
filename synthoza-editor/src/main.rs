use eframe::egui::*;

use crate::piano_roll::PianoRoll;

mod piano_roll;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Synthoza Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap();
}

struct App {
    frame_count: u64,
    piano_roll: PianoRoll,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            frame_count: 0,
            piano_roll: PianoRoll::default(),
        }
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ui, |ui| {
            self.frame_count += 1;

            ui.label(format!("Frame {}", self.frame_count));

            ScrollArea::both()
                .stick_to_bottom(true)
                .show(ui, |ui| self.piano_roll.show(ui));
        });
    }
}
