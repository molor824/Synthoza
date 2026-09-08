mod piano_roll;

use crate::piano_roll::PianoRoll;
use eframe::egui::{ScrollArea, Ui};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Synthoza",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap();
}

struct App {
    piano_roll: PianoRoll,
}

impl App {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            piano_roll: PianoRoll::default(),
        }
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        ScrollArea::vertical().show(ui, |ui| {
            self.piano_roll.show(ui);
        });
    }
}
