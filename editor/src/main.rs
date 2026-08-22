use iced::{
    Element, Length,
    widget::{Canvas, scrollable},
};

use crate::piano::{Note, Piano};

mod piano;

struct State {
    notes: Vec<bool>,
    note_height: f32,
}

#[derive(Clone)]
enum Msg {
    NoteUpdate(Note, bool),
}

impl State {
    fn new() -> Self {
        Self {
            notes: vec![false; 12 * 10],
            note_height: 15.0,
        }
    }
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::NoteUpdate(note, state) => self.notes[note.0 as usize] = state,
        }
    }
    fn view(&self) -> impl Into<Element<'_, Msg>> {
        let piano = Canvas::new(Piano {
            notes: &self.notes,
            updater: Msg::NoteUpdate,
        })
        .height(self.notes.len() as f32 * self.note_height)
        .width(Length::Fill);

        scrollable(piano)
            .anchor_bottom()
            .height(Length::Fill)
            .width(100.0)
    }
}

fn main() {
    iced::application(State::new, State::update, State::view)
        .antialiasing(true)
        .resizable(true)
        .run()
        .unwrap();
}
