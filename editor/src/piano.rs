use std::collections::HashMap;

use iced::{
    Color, Element, Event, Length, Pixels, Point, Rectangle, Renderer, Size, Theme,
    alignment::Vertical,
    keyboard::{self, Key},
    mouse::{self, Cursor},
    widget::{
        Canvas,
        canvas::{Action, Frame, Geometry, Program, Stroke, Text},
        text::Alignment,
    },
};

const BLACK_KEY_COL: f32 = 0.1;
const ACTIVE_BLACK_KEY_COL: f32 = 0.25;
const WHITE_KEY_COL: f32 = 0.95;
const ACTIVE_WHITE_KEY_COL: f32 = 0.8;
const BLACK_KEY_WIDTH: f32 = 0.6;

const KEY_COLS: [f32; 12] = [
    WHITE_KEY_COL,
    BLACK_KEY_COL,
    WHITE_KEY_COL,
    BLACK_KEY_COL,
    WHITE_KEY_COL,
    WHITE_KEY_COL,
    BLACK_KEY_COL,
    WHITE_KEY_COL,
    BLACK_KEY_COL,
    WHITE_KEY_COL,
    BLACK_KEY_COL,
    WHITE_KEY_COL,
];
const ACTIVE_KEY_COLS: [f32; 12] = [
    ACTIVE_WHITE_KEY_COL,
    ACTIVE_BLACK_KEY_COL,
    ACTIVE_WHITE_KEY_COL,
    ACTIVE_BLACK_KEY_COL,
    ACTIVE_WHITE_KEY_COL,
    ACTIVE_WHITE_KEY_COL,
    ACTIVE_BLACK_KEY_COL,
    ACTIVE_WHITE_KEY_COL,
    ACTIVE_BLACK_KEY_COL,
    ACTIVE_WHITE_KEY_COL,
    ACTIVE_BLACK_KEY_COL,
    ACTIVE_WHITE_KEY_COL,
];
const KEY_OFFSETS: [f32; 12] = [
    -0.5, 0.0, -0.5, 0.0, 0.0, -0.5, 0.0, -0.5, 0.0, -0.5, 0.0, 0.0,
];
const KEY_HEIGHT_SCALES: [f32; 12] = [1.5, 1.0, 2.0, 1.0, 1.5, 1.5, 1.0, 2.0, 1.0, 2.0, 1.0, 1.5];
const KEY_WIDTH_SCALES: [f32; 12] = [
    1.0,
    BLACK_KEY_WIDTH,
    1.0,
    BLACK_KEY_WIDTH,
    1.0,
    1.0,
    BLACK_KEY_WIDTH,
    1.0,
    BLACK_KEY_WIDTH,
    1.0,
    BLACK_KEY_WIDTH,
    1.0,
];
const KEY_RENDER_ORDER: [usize; 12] = [0, 2, 4, 5, 7, 9, 11, 1, 3, 6, 8, 10];

thread_local! {
    static KEYBARD_KEY: HashMap<Key, usize> = [
        "z", "s", "x", "d", "c", "v", "g", "b", "h", "n", "j", "m",
        "q", "2", "w", "3", "e", "r", "5", "t", "6", "y", "7", "u", "i", "9", "o", "0", "p",
    ].into_iter().enumerate().map(|(i, k)| (Key::Character(k.into()), i))
        .chain(
            [",", "l", ".", ";", "/"]
                .into_iter().enumerate()
                .map(|(i, k)| (Key::Character(k.into()), i + 12))
        ).collect();
}

pub enum Msg {
    KeyAdd(usize),
    KeyRemove(usize),
    KeyChange(usize, usize),
}

pub struct Widget<'a>(&'a Piano);

#[derive(Default, Debug)]
pub struct State {
    clicked_key: Option<usize>,
    mouse_clicked: bool,
}

pub struct Piano {
    pub keys: Vec<bool>,
    pub keyboard_octave: usize,
    pub key_height: f32,
}
impl Default for Piano {
    fn default() -> Self {
        Self {
            keys: vec![false; 12 * 10],
            keyboard_octave: 0,
            key_height: 15.0,
        }
    }
}
impl Piano {
    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::KeyAdd(key) => self.keys[key] = true,
            Msg::KeyRemove(key) => self.keys[key] = false,
            Msg::KeyChange(from, to) => {
                self.keys[from] = false;
                self.keys[to] = true;
            }
        }
    }
    pub fn view<'a>(&'a self) -> impl Into<Element<'a, Msg>> {
        Canvas::new(Widget(self))
            .height(self.keys.len() as f32 * self.key_height)
            .width(100.0)
    }
}

impl<'a> Widget<'a> {
    fn note_rect(&self, key_size: Size, index: usize) -> Rectangle {
        let key = index % 12;
        Rectangle::new(
            Point::new(
                0.0,
                ((self.0.keys.len() - index - 1) as f32 + KEY_OFFSETS[key]) * key_size.height,
            ),
            Size::new(
                key_size.width * KEY_WIDTH_SCALES[key],
                key_size.height * KEY_HEIGHT_SCALES[key],
            ),
        )
    }
    fn key_size(&self, bounds: Rectangle) -> Size {
        Size::new(bounds.width, bounds.height / self.0.keys.len() as f32)
    }
}

impl<'a> Program<Msg> for Widget<'a> {
    type State = State;
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());
        let key_size = self.key_size(bounds);

        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::BLACK);

        for i in (0..self.0.keys.len()).step_by(12) {
            for key in KEY_RENDER_ORDER {
                let abs_key = key + i;
                let Some(&active) = self.0.keys.get(abs_key) else {
                    continue;
                };

                let rect = self.note_rect(key_size, abs_key);
                let intensity = if !active {
                    KEY_COLS[key]
                } else {
                    ACTIVE_KEY_COLS[key]
                };
                frame.fill_rectangle(
                    rect.position(),
                    rect.size(),
                    Color::from_rgb(intensity, intensity, intensity),
                );
                frame.stroke_rectangle(
                    rect.position(),
                    rect.size(),
                    Stroke::default().with_width(2.0),
                );
                if key == 0 {
                    frame.fill_text(Text {
                        content: format!("{}", i / 12),
                        align_x: Alignment::Right,
                        align_y: Vertical::Bottom,
                        position: Point::new(rect.x + rect.width - 2.0, rect.y + rect.height),
                        color: Color::BLACK,
                        size: Pixels::from(rect.height * 0.7),
                        ..Default::default()
                    });
                }
            }
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<Msg>> {
        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left)
                        if let Cursor::Available(cursor_pos) = cursor
                            && bounds.contains(cursor_pos) =>
                    {
                        state.mouse_clicked = true
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        state.mouse_clicked = false
                    }
                    _ => {}
                }

                let mut clicked_key: Option<usize> = None;
                let key_size = self.key_size(bounds);

                if state.mouse_clicked
                    && let Cursor::Available(cursor_pos) = cursor
                {
                    'outer: for i in (0..self.0.keys.len()).step_by(12) {
                        for key in KEY_RENDER_ORDER.into_iter().rev() {
                            let abs_key = key + i;
                            let note_rect = self.note_rect(key_size, abs_key);

                            if note_rect.contains(cursor_pos) {
                                clicked_key = Some(abs_key);
                                break 'outer;
                            }
                        }
                    }
                }

                let msg = match clicked_key {
                    Some(key) => match state.clicked_key {
                        Some(state_key) if state_key == key => None,
                        Some(state_key) => Some(Msg::KeyChange(state_key, key)),
                        None => Some(Msg::KeyAdd(key)),
                    },
                    None => match state.clicked_key {
                        Some(state_key) => Some(Msg::KeyRemove(state_key)),
                        None => None,
                    },
                };

                state.clicked_key = clicked_key;

                return msg.map(Action::publish);
            }
            Event::Keyboard(kbd_event) => match kbd_event {
                keyboard::Event::KeyPressed { key, .. }
                | keyboard::Event::KeyReleased { key, .. } => {
                    let piano_key = KEYBARD_KEY
                        .with(|map| map.get(&key).copied())
                        .map(|k| k + self.0.keyboard_octave * 12);

                    if let Some(key) = piano_key
                        && let Some(&pressed) = self.0.keys.get(key)
                    {
                        return match kbd_event {
                            keyboard::Event::KeyPressed { .. } if !pressed => {
                                Some(Msg::KeyAdd(key))
                            }
                            keyboard::Event::KeyReleased { .. } if pressed => {
                                Some(Msg::KeyRemove(key))
                            }
                            _ => None,
                        }
                        .map(Action::publish);
                    }
                }
                _ => {}
            },
            _ => {}
        }
        None
    }
}
