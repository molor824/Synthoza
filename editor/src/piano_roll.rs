use eframe::egui::*;
use std::{iter, sync::LazyLock};
use std::num::NonZeroUsize;

const WHITE_KEY: Rgba = Rgba::from_rgb(0.9, 0.9, 0.9);
const WHITE_KEY_CLICK: Rgba = Rgba::from_rgb(0.6, 0.6, 0.6);

const BLACK_KEY: Rgba = Rgba::from_rgb(0.05, 0.05, 0.05);
const BLACK_KEY_CLICK: Rgba = Rgba::from_rgb(0.15, 0.15, 0.15);

const KEY_WIDTH: f32 = 10.0;
const KEY_HEIGHT: f32 = 60.0;
const BLACK_KEY_HEIGHT: f32 = 40.0;

const PIANO_SIZE: Vec2 = Vec2::new(KEY_HEIGHT, KEY_WIDTH * 12.0);
const STROKE_WIDTH: f32 = 1.0;
const STROKE_COLOR: Rgba = Rgba::from_rgb(0.0, 0.0, 0.0);

static WHITE_RECTS: LazyLock<[Rect; 7]> = LazyLock::new(|| {
    let sizes = [1.5, 2.0, 2.0, 1.5, 1.5, 2.0, 1.5].map(|y| Vec2::new(KEY_HEIGHT, KEY_WIDTH * y));
    let mut pos = Pos2::ZERO;

    std::array::from_fn(|idx| {
        let size = sizes[idx];
        let rect = Rect::from_min_size(pos, size);
        pos.y += size.y;
        rect
    })
});
static BLACK_RECTS: LazyLock<[Rect; 5]> = LazyLock::new(|| {
    let positions = [1.0, 3.0, 5.0, 8.0, 10.0].map(|y| Pos2::new(0.0, KEY_WIDTH * y));

    std::array::from_fn(|idx| {
        let pos = positions[idx];
        Rect::from_min_size(pos, Vec2::new(BLACK_KEY_HEIGHT, KEY_WIDTH))
    })
});
const BLACK_INDICES: [usize; 5] = [10, 8, 6, 3, 1];
const WHITE_INDICES: [usize; 7] = [11, 9, 7, 5, 4, 2, 0];

const OCTAVES: usize = 10;

fn offset_rect(rect: Rect, offset: Vec2) -> Rect {
    Rect {
        min: rect.min + offset,
        max: rect.max + offset,
    }
}

const KEYBOARD_KEYS: [(Key, usize); 17 * 2] = [
    (Key::Z, 0),
    (Key::S, 1),
    (Key::X, 2),
    (Key::D, 3),
    (Key::C, 4),
    (Key::V, 5),
    (Key::G, 6),
    (Key::B, 7),
    (Key::H, 8),
    (Key::N, 9),
    (Key::J, 10),
    (Key::M, 11),
    (Key::Comma, 12),
    (Key::L, 13),
    (Key::Period, 14),
    (Key::Semicolon, 15),
    (Key::Slash, 16),
    (Key::Q, 12),
    (Key::Num2, 13),
    (Key::W, 14),
    (Key::Num3, 15),
    (Key::E, 16),
    (Key::R, 17),
    (Key::Num5, 18),
    (Key::T, 19),
    (Key::Num6, 20),
    (Key::Y, 21),
    (Key::Num7, 22),
    (Key::U, 23),
    (Key::I, 24),
    (Key::Num9, 25),
    (Key::O, 26),
    (Key::Num0, 27),
    (Key::P, 28),
];
const KEYBOARD_MAX_OFFSET: usize = KEYBOARD_KEYS[KEYBOARD_KEYS.len() - 1].1;

pub struct Piano {
    kbd_octave: usize,
    mouse_key: Option<usize>, // starting at C0 - 0, D0 - 1 etc...
    kbd_keys: [bool; KEYBOARD_KEYS.len()], // Keyboard keys ordered, and represents its equivalent indexed notes.
}
impl Default for Piano {
    fn default() -> Self {
        Self {
            kbd_octave: 4,
            mouse_key: None,
            kbd_keys: [false; _],
        }
    }
}
impl Piano {
    pub fn show(&mut self, ui: &mut Ui) {
        let full_rect = PIANO_SIZE * Vec2::new(1.0, OCTAVES as f32);
        let (response, painter) = ui.allocate_painter(full_rect, Sense::click_and_drag());
        let render_rect = response.rect;
        let clicking = response.is_pointer_button_down_on()
            && ui.input(|input| input.pointer.button_down(PointerButton::Primary));
        let pointer_at = ui.input(|i| i.pointer.latest_pos());

        ui.ctx().input(|input| {
            for event in input.events.iter() {
                match event {
                    Event::Key {
                        physical_key: Some(key),
                        pressed,
                        ..
                    } if let Some(i) = KEYBOARD_KEYS.iter().find(|k| k.0 == *key).map(|k| k.1) => {
                        self.kbd_keys[i] = *pressed;
                    }
                    _ => {}
                }
            }
        });

        self.mouse_key = None;
        if clicking && let Some(pointer) = pointer_at {
            for octave in 0..OCTAVES {
                if self.mouse_key.is_some() {
                    break;
                }

                let start = render_rect.min.to_vec2() + self.octave_start(octave);

                self.mouse_key = BLACK_RECTS
                    .into_iter()
                    .zip(BLACK_INDICES)
                    .chain(WHITE_RECTS.into_iter().zip(WHITE_INDICES))
                    .find(|(rect, _)| offset_rect(*rect, start).contains(pointer))
                    .map(|(_, idx)| octave * 12 + idx);
            }
        }

        for octave in 0..OCTAVES {
            let start = render_rect.min.to_vec2() + self.octave_start(octave);

            for ((rect, idx), (color, click_color)) in WHITE_RECTS
                .into_iter()
                .zip(WHITE_INDICES)
                .zip(iter::repeat((WHITE_KEY, WHITE_KEY_CLICK)))
                .chain(
                    BLACK_RECTS
                        .into_iter()
                        .zip(BLACK_INDICES)
                        .zip(iter::repeat((BLACK_KEY, BLACK_KEY_CLICK))),
                )
            {
                let rect = offset_rect(rect, start);
                painter.rect_filled(
                    rect,
                    0.0,
                    if self.pressed(octave * 12 + idx) {
                        click_color
                    } else {
                        color
                    },
                );
                painter.rect_stroke(rect, 0.0, (STROKE_WIDTH, STROKE_COLOR), StrokeKind::Inside);
            }

            painter.text(
                start.to_pos2() + PIANO_SIZE + Vec2::splat(-2.0),
                Align2::RIGHT_BOTTOM,
                octave.to_string(),
                FontId::proportional(10.0),
                Color32::BLACK,
            );
        }
    }
    fn octave_start(&self, octave: usize) -> Vec2 {
        Vec2::new(0.0, PIANO_SIZE.y * (OCTAVES - octave - 1) as f32)
    }
    pub fn pressed(&self, key: usize) -> bool {
        let kbd_start = self.kbd_octave * 12;
        let kbd_end = kbd_start + KEYBOARD_MAX_OFFSET;
        self.mouse_key == Some(key)
            || (key >= kbd_start
                && key <= kbd_end
                && self
                    .kbd_keys
                    .into_iter()
                    .zip(KEYBOARD_KEYS)
                    .any(|(pressed, (_, offset))| offset + self.kbd_octave * 12 == key && pressed))
    }
}

#[derive(Default)]
pub struct NoteEditor {
    notes: Vec<Note>, // Must be ordered by Note::begin!!!
}

impl NoteEditor {
    pub fn show(&mut self, ui: &mut Ui) {
        let (response, painter) = ui.allocate_painter(todo!(), Sense::click_and_drag());
    }
}

pub struct Note {
    pub begin: f32, // measured in seconds
    pub end: f32,
    pub key: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimeSignature {
    pub value: NonZeroUsize,
    pub signature: NonZeroUsize,
}
impl Default for TimeSignature {
    fn default() -> Self {
        Self {
            value: NonZeroUsize::new(4).unwrap(),
            signature: NonZeroUsize::new(4).unwrap(),
        }
    }
}

#[derive(Default)]
pub struct PianoRoll {
    piano: Piano,
    editor: NoteEditor,
}

impl PianoRoll {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.piano.show(ui);
            self.editor.show(ui);
        });
    }
}
