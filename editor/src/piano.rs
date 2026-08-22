use std::mem::transmute;

use iced::{
    Color, Point, Size,
    widget::canvas::{self, Frame, Stroke},
};

const BLACK_KEY_COL: f32 = 0.15;
const ACTIVE_BLACK_KEY_COL: f32 = 0.2;
const WHITE_KEY_COL: f32 = 0.85;
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

pub struct Piano<'a, Msg, Upd>
where
    Upd: FnOnce(Note, bool) -> Msg,
{
    pub notes: &'a [bool], // Note maps. This also determines the piano length
    pub updater: Upd,      // Updater function that updates piano according to message
}

impl<'a, Msg, Upd> canvas::Program<Msg> for Piano<'a, Msg, Upd>
where
    Upd: FnOnce(Note, bool) -> Msg,
{
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());
        let note_count = self.notes.len();
        let key_height = bounds.height / note_count as f32;

        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::BLACK);

        for i in (0..note_count).step_by(12) {
            for n in 0..12 {
                let key = KEY_RENDER_ORDER[n];
                let abs_key = key + i;
                let Some(&active) = self.notes.get(abs_key) else {
                    continue;
                };

                let start = Point::new(
                    0.0,
                    ((note_count - abs_key - 1) as f32 + KEY_OFFSETS[key]) * key_height,
                );
                let size = Size::new(
                    bounds.width * KEY_WIDTH_SCALES[key],
                    key_height * KEY_HEIGHT_SCALES[key],
                );
                let intensity = if active {
                    KEY_COLS[key]
                } else {
                    ACTIVE_KEY_COLS[key]
                };
                frame.fill_rectangle(
                    start,
                    size,
                    Color::from_rgb(intensity, intensity, intensity),
                );
                frame.stroke_rectangle(start, size, Stroke::default().with_width(2.0));
            }
        }

        vec![frame.into_geometry()]
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NoteKind {
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
impl TryFrom<u8> for NoteKind {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 12 {
            return Err(format!("value {value} cannot be converted to note kind"));
        }
        Ok(unsafe { transmute(value) })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Note(pub u32);
impl Note {
    pub const fn new(kind: NoteKind, octave: u32) -> Self {
        Self(octave * 12 + kind as u8 as u32)
    }
    pub fn kind(self) -> NoteKind {
        NoteKind::try_from((self.0 % 12) as u8).unwrap()
    }
    pub fn octave(self) -> u32 {
        self.0 / 12
    }
    pub fn pitch_c0_freq(self, c0_freq: f32) -> f32 {
        2.0_f32.powf(self.0 as f32 / 12.0) * c0_freq
    }
    pub fn pitch_a4_freq(self, a4_freq: f32) -> f32 {
        2.0_f32.powf((self.0 - Self::new(NoteKind::A, 4).0) as f32 / 12.0) * a4_freq
    }
    pub fn pitch_standard(self) -> f32 {
        self.pitch_a4_freq(440.0)
    }
}
