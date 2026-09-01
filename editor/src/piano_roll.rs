use iced::{
    Element, Length,
    widget::{row, scrollable},
};

use crate::{
    note_field::{self, NoteField},
    piano::{self, Piano},
};

#[derive(Default)]
pub struct PianoRoll {
    pub piano: Piano,
    pub note_field: NoteField,
}

impl PianoRoll {
    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Piano(msg) => self.piano.update(msg),
            Msg::NoteField(msg) => self.note_field.update(msg),
        }
    }
    pub fn view<'a>(&'a self) -> impl Into<Element<'a, Msg>> {
        scrollable(row([
            self.piano.view().into().map(Msg::Piano),
            scrollable(self.note_field.view(&self.piano).into().map(Msg::NoteField))
                .horizontal()
                .auto_scroll(true)
                .into(),
        ]))
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::hidden(),
        ))
        .anchor_bottom()
        .height(Length::Fill)
        .width(Length::Fill)
        .auto_scroll(true)
    }
}

pub enum Msg {
    Piano(piano::Msg),
    NoteField(note_field::Msg),
}
