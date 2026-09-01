use iced::{
    Color, Element, Event, Point, Rectangle, Renderer, Theme, mouse,
    widget::{
        Action, Canvas,
        canvas::{Frame, Geometry, Program},
    },
};

use crate::piano::Piano;

pub struct NoteField {
    notes_clips: Vec<Vec<Clip>>, // NOTE: Inner vec<T> assumes ordered Clip!, outer vec assumes the note indices starting from C0
    bars: usize,
    signature: TimeSignature,
    quarter_note_len: f32,
}
impl Default for NoteField {
    fn default() -> Self {
        Self {
            notes_clips: vec![],
            bars: 1,
            signature: Default::default(),
            quarter_note_len: 100.0,
        }
    }
}
impl NoteField {
    fn bar_len(&self) -> f32 {
        self.quarter_note_len * 4.0 / self.signature.0 as f32 * self.signature.1 as f32
    }
    pub fn update(&mut self, msg: Msg) {}
    pub fn view<'a>(&'a self, piano: &Piano) -> impl Into<Element<'a, Msg>> {
        Canvas::new(Widget {
            note_field: self,
            keys_len: piano.keys.len(),
            key_height: piano.key_height,
        })
        .width(self.bars as f32 * self.bar_len())
        .height(piano.keys.len() as f32 * piano.key_height)
    }
}

pub struct Widget<'a> {
    note_field: &'a NoteField,
    keys_len: usize,
    key_height: f32,
}

const BG_COLOR: Color = Color::from_rgb(0.12, 0.12, 0.12);

impl<'a> Program<Msg> for Widget<'a> {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        frame.fill_rectangle(Point::ORIGIN, bounds.size(), BG_COLOR);

        vec![frame.into_geometry()]
    }
    fn update(
        &self,
        _state: &mut Self::State,
        _event: &Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<Action<Msg>> {
        None
    }
}

pub enum Msg {}

#[derive(Clone, Copy)]
pub struct Clip {
    pub start: f32,
    pub duration: f32,
}

#[derive(Clone, Copy)]
pub struct TimeSignature(usize, usize);
impl Default for TimeSignature {
    fn default() -> Self {
        TimeSignature(4, 4)
    }
}
