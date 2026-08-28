use iced::{
    Element, Length,
    widget::{Canvas, scrollable},
};

use crate::piano::{Piano, PianoMsg};

mod piano;

struct State {
    keys: Vec<bool>,
    key_height: f32,
}

#[derive(Clone)]
enum Msg {
    PianoMsg(PianoMsg),
}

impl State {
    fn new() -> Self {
        Self {
            keys: vec![false; 12 * 10],
            key_height: 15.0,
        }
    }
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::PianoMsg(msg) => match msg {
                PianoMsg::KeyAdd(key) => self.keys[key] = true,
                PianoMsg::KeyRemove(key) => self.keys[key] = false,
                PianoMsg::KeyChange(from, to) => {
                    self.keys[from] = false;
                    self.keys[to] = true;
                }
            },
        }
    }
    fn view(&self) -> impl Into<Element<'_, Msg>> {
        let piano = Canvas::new(Piano {
            keys: &self.keys,
            keyboard_octave: 0,
        })
        .height(self.keys.len() as f32 * self.key_height)
        .width(Length::Fill);

        Element::map(
            scrollable(piano)
                .anchor_bottom()
                .height(Length::Fill)
                .width(100.0)
                .into(),
            Msg::PianoMsg,
        )
    }
}

fn main() {
    iced::application(State::new, State::update, State::view)
        .antialiasing(true)
        .resizable(true)
        .run()
        .unwrap();
}
