use std::mem::transmute;

use iced::{
    Color, Event, Point, Rectangle, Renderer, Size, Theme,
    mouse::{self, Cursor},
    widget::canvas::{Action, Frame, Geometry, Program, Stroke},
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

#[derive(Clone)]
pub enum PianoMsg {
    KeyAdd(usize),
    KeyRemove(usize),
    KeyChange(usize, usize),
}

pub struct Piano<'a> {
    pub keys: &'a [bool], // Key maps. This also determines the piano length
}

#[derive(Default, Debug)]
pub struct PianoState {
    clicked_key: Option<usize>,
    mouse_clicked: bool,
}

impl<'a> Piano<'a> {
    fn note_rect(&self, key_size: Size, index: usize) -> Rectangle {
        let key = index % 12;
        Rectangle::new(
            Point::new(
                0.0,
                ((self.keys.len() - index - 1) as f32 + KEY_OFFSETS[key]) * key_size.height,
            ),
            Size::new(
                key_size.width * KEY_WIDTH_SCALES[key],
                key_size.height * KEY_HEIGHT_SCALES[key],
            ),
        )
    }
    fn key_size(&self, bounds: Rectangle) -> Size {
        Size::new(bounds.width, bounds.height / self.keys.len() as f32)
    }
}

impl<'a> Program<PianoMsg> for Piano<'a> {
    type State = PianoState;
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

        for i in (0..self.keys.len()).step_by(12) {
            for key in KEY_RENDER_ORDER {
                let abs_key = key + i;
                let Some(&active) = self.keys.get(abs_key) else {
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
    ) -> Option<Action<PianoMsg>> {
        let Event::Mouse(mouse_event) = event else {
            return None;
        };

        match mouse_event {
            mouse::Event::ButtonPressed(mouse::Button::Left) => state.mouse_clicked = true,
            mouse::Event::ButtonReleased(mouse::Button::Left) => state.mouse_clicked = false,
            _ => {}
        }

        let mut clicked_key: Option<usize> = None;
        let key_size = self.key_size(bounds);

        if state.mouse_clicked
            && let Cursor::Available(cursor_pos) = cursor
        {
            'outer: for i in (0..self.keys.len()).step_by(12) {
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
                Some(state_key) => Some(PianoMsg::KeyChange(state_key, key)),
                None => Some(PianoMsg::KeyAdd(key)),
            },
            None => match state.clicked_key {
                Some(state_key) => Some(PianoMsg::KeyRemove(state_key)),
                None => None,
            },
        };

        state.clicked_key = clicked_key;

        msg.map(|msg| Action::publish(msg))
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}
impl TryFrom<u8> for Note {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 12 {
            return Err(format!("value {value} cannot be converted to note"));
        }
        Ok(unsafe { transmute(value) })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key(pub u32);
impl Key {
    pub const fn new(note: Note, octave: u32) -> Self {
        Self(octave * 12 + note as u8 as u32)
    }
    pub fn note(self) -> Note {
        Note::try_from((self.0 % 12) as u8).unwrap()
    }
    pub fn octave(self) -> u32 {
        self.0 / 12
    }
    pub fn pitch_c0_freq(self, c0_freq: f32) -> f32 {
        2.0_f32.powf(self.0 as f32 / 12.0) * c0_freq
    }
    pub fn pitch_a4_freq(self, a4_freq: f32) -> f32 {
        2.0_f32.powf((self.0 - Self::new(Note::A, 4).0) as f32 / 12.0) * a4_freq
    }
    pub fn pitch_standard(self) -> f32 {
        self.pitch_a4_freq(440.0)
    }
}
