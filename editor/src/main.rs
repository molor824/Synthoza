mod piano_roll;

use crate::piano_roll::PianoRoll;
use eframe::egui::{Align, Layout, ScrollArea, Ui};

fn main() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Synthoza",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap();
}

struct App {
    piano_roll: PianoRoll,
    frame_count: u64,
}

impl App {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            piano_roll: PianoRoll::default(),
            frame_count: 0,
        }
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.frame_count += 1;

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.label(format!("Frame: {}", self.frame_count));
        });
        ScrollArea::vertical()
            .max_height(ui.available_size().y - 20.0)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                self.piano_roll.show(ui);
            });
    }
}
