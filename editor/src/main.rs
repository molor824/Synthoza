use iced::Element;

use crate::piano_roll::PianoRoll;

mod note_field;
mod piano;
mod piano_roll;

#[derive(Default)]
struct Model {
    piano_roll: PianoRoll,
}

enum Msg {
    PianoRoll(piano_roll::Msg),
}

impl Model {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::PianoRoll(msg) => self.piano_roll.update(msg),
        }
    }
    fn view(&self) -> impl Into<Element<'_, Msg>> {
        self.piano_roll.view().into().map(Msg::PianoRoll)
    }
}

fn main() {
    iced::application(Model::default, Model::update, Model::view)
        .antialiasing(true)
        .resizable(true)
        .run()
        .unwrap();
}
