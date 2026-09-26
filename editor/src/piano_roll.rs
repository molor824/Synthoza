use eframe::egui::*;
use std::iter;
use std::num::NonZeroUsize;

const WHITE_KEY: Rgba = Rgba::from_rgb(0.9, 0.9, 0.9);
const WHITE_KEY_CLICK: Rgba = Rgba::from_rgb(0.6, 0.6, 0.6);

const BLACK_KEY: Rgba = Rgba::from_rgb(0.05, 0.05, 0.05);
const BLACK_KEY_CLICK: Rgba = Rgba::from_rgb(0.15, 0.15, 0.15);

const KEY_WIDTH: f32 = 60.0;
const BLACK_KEY_WIDTH: f32 = 40.0;

const KEY_STROKE_WIDTH: f32 = 1.0;
const KEY_STROKE_COLOR: Rgba = Rgba::from_rgb(0.0, 0.0, 0.0);

const BLACK_INDICES: [usize; 5] = [10, 8, 6, 3, 1];
const WHITE_INDICES: [usize; 7] = [11, 9, 7, 5, 4, 2, 0];

const OCTAVES: usize = 10;

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
    key_height: f32,
}
impl Default for Piano {
    fn default() -> Self {
        Self {
            kbd_octave: 4,
            mouse_key: None,
            kbd_keys: [false; _],
            key_height: 16.0,
        }
    }
}
impl Piano {
    pub fn show(&mut self, ui: &mut Ui) {
        let full_size = Vec2::new(KEY_WIDTH, self.key_height * 12.0 * OCTAVES as f32);
        let (response, painter) = ui.allocate_painter(full_size, Sense::click_and_drag());
        let render_rect = response.rect;
        let clicking = response.is_pointer_button_down_on()
            && ui.input(|input| input.pointer.button_down(PointerButton::Primary));
        let pointer_at = ui.input(|i| i.pointer.latest_pos());
        let white_rects = self.white_rects();
        let black_rects = self.black_rects();

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

                let start = self.octave_start(octave) + render_rect.min.to_vec2();

                self.mouse_key = black_rects
                    .into_iter()
                    .zip(BLACK_INDICES)
                    .chain(white_rects.into_iter().zip(WHITE_INDICES))
                    .find(|(rect, _)| rect.translate(start).contains(pointer))
                    .map(|(_, idx)| octave * 12 + idx);
            }
        }

        for octave in 0..OCTAVES {
            let start = self.octave_start(octave) + render_rect.min.to_vec2();

            for ((rect, idx), (color, click_color)) in white_rects
                .into_iter()
                .zip(WHITE_INDICES)
                .zip(iter::repeat((WHITE_KEY, WHITE_KEY_CLICK)))
                .chain(
                    black_rects
                        .into_iter()
                        .zip(BLACK_INDICES)
                        .zip(iter::repeat((BLACK_KEY, BLACK_KEY_CLICK))),
                )
            {
                let rect = rect.translate(start);
                painter.rect_filled(
                    rect,
                    0.0,
                    if self.pressed(octave * 12 + idx) {
                        click_color
                    } else {
                        color
                    },
                );
                painter.rect_stroke(
                    rect,
                    0.0,
                    (KEY_STROKE_WIDTH, KEY_STROKE_COLOR),
                    StrokeKind::Inside,
                );
            }

            painter.text(
                (start + Vec2::new(KEY_WIDTH, self.key_height * 12.0) + Vec2::splat(-2.0))
                    .to_pos2(),
                Align2::RIGHT_BOTTOM,
                octave.to_string(),
                FontId::proportional(10.0),
                Color32::BLACK,
            );
        }
    }

    fn white_rects(&self) -> [Rect; 7] {
        let heights = [1.5, 2.0, 2.0, 1.5, 1.5, 2.0, 1.5];
        let mut pos = Pos2::ZERO;

        std::array::from_fn(|idx| {
            let size = Vec2::new(KEY_WIDTH, heights[idx] * self.key_height);
            let rect = Rect::from_min_size(pos, size);
            pos.y += size.y;
            rect
        })
    }
    fn black_rects(&self) -> [Rect; 5] {
        let positions = [1.0, 3.0, 5.0, 8.0, 10.0];

        std::array::from_fn(|idx| {
            Rect::from_min_size(
                Pos2::new(0.0, self.key_height * positions[idx]),
                Vec2::new(BLACK_KEY_WIDTH, self.key_height),
            )
        })
    }
    fn octave_start(&self, octave: usize) -> Vec2 {
        Vec2::new(0.0, self.key_height * 12.0 * (OCTAVES - octave - 1) as f32)
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

pub struct NoteEditor {
    notes: Vec<Note>, // Must be ordered by Note::begin!!!
    signature: TimeSignature,
    bars: usize,
    note_width: f32, // Measured in whole note
    division: NonZeroUsize,
    snapping: bool,
}
impl Default for NoteEditor {
    fn default() -> Self {
        Self {
            notes: vec![],
            signature: TimeSignature::default(),
            bars: 4,
            note_width: 200.0,
            snapping: false,
            division: NonZeroUsize::new(1).unwrap(),
        }
    }
}

const OCTAVE_STROKE_WIDTH: f32 = 1.0;
const OCTAVE_STROKE_GRAY: f32 = BLACK_HIGHLIGHT_GRAY;

const MAJOR_STROKE_WIDTH: f32 = 2.0;
const MAJOR_STROKE_ALPHA: f32 = 0.5;

const MINOR_STROKE_WIDTH: f32 = 1.5;
const MINOR_STROKE_ALPHA: f32 = 0.25;

const SNAP_STROKE_WIDTH: f32 = 1.0;
const SNAP_STROKE_ALPHA: f32 = 0.1;

const WHITE_HIGHLIGHT_GRAY: f32 = 0.05;
const BLACK_HIGHLIGHT_GRAY: f32 = 0.03;

impl NoteEditor {
    pub fn show(&mut self, ui: &mut Ui, piano: &Piano) {
        let value_width = self.note_width / self.signature.value.get() as f32;
        let bar_width = value_width * self.signature.measure.get() as f32;
        let full_size = Vec2::new(
            bar_width * self.bars as f32,
            piano.key_height * 12.0 * OCTAVES as f32,
        );
        let (response, painter) = ui.allocate_painter(full_size, Sense::click_and_drag());
        let render_rect = response.rect;

        let octave_stroke = Stroke::new(OCTAVE_STROKE_WIDTH, Rgba::from_gray(OCTAVE_STROKE_GRAY));
        let major_stroke = Stroke::new(
            MAJOR_STROKE_WIDTH,
            Rgba::from_white_alpha(MAJOR_STROKE_ALPHA),
        );
        let minor_stroke = Stroke::new(
            MINOR_STROKE_WIDTH,
            Rgba::from_white_alpha(MINOR_STROKE_ALPHA),
        );
        let snap_stroke = Stroke::new(SNAP_STROKE_WIDTH, Rgba::from_white_alpha(SNAP_STROKE_ALPHA));

        let white_highlight = Rgba::from_gray(WHITE_HIGHLIGHT_GRAY);
        let black_highlight = Rgba::from_gray(BLACK_HIGHLIGHT_GRAY);

        for octave in 0..=OCTAVES {
            let base = render_rect.min + Vec2::new(0.0, (octave * 12) as f32 * piano.key_height);

            if octave != OCTAVES {
                for (idx, color) in WHITE_INDICES
                    .into_iter()
                    .zip(iter::repeat(white_highlight))
                    .chain(BLACK_INDICES.into_iter().zip(iter::repeat(black_highlight)))
                {
                    let rect = Rect::from_min_size(
                        base + Vec2::new(0.0, (11 - idx) as f32 * piano.key_height),
                        Vec2::new(render_rect.width(), piano.key_height),
                    );
                    painter.rect_filled(rect, 0.0, color);
                }
            }

            painter.hline(render_rect.x_range(), base.y, octave_stroke);
            painter.hline(
                render_rect.x_range(),
                base.y + 7.0 * piano.key_height,
                octave_stroke,
            );
        }

        for b in 0..=self.bars {
            let x = render_rect.min.x + bar_width * b as f32;
            painter.vline(x, render_rect.y_range(), major_stroke);

            if b == self.bars {
                break;
            }

            for i in 0..self.signature.measure.get() {
                let x = x + value_width * i as f32;
                if i > 0 {
                    painter.vline(x, render_rect.y_range(), minor_stroke);
                }

                if self.snapping {
                    let snap_width = value_width / self.division.get() as f32;
                    for i in 1..self.division.get() {
                        let x = x + snap_width * i as f32;
                        painter.vline(x, render_rect.y_range(), snap_stroke);
                    }
                }
            }
        }
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
    pub measure: NonZeroUsize,
}
impl Default for TimeSignature {
    fn default() -> Self {
        Self {
            value: NonZeroUsize::new(4).unwrap(),
            measure: NonZeroUsize::new(4).unwrap(),
        }
    }
}

const COMMON_DIVS: [(usize, &str); 9] = [
    (0, "No snap"),
    (1, "1x"),
    (2, "1/2x"),
    (3, "1/3x"),
    (4, "1/4x"),
    (6, "1/6x"),
    (8, "1/8x"),
    (12, "1/12x"),
    (16, "1/16x"),
];

#[derive(Default)]
pub struct PianoRoll {
    piano: Piano,
    editor: NoteEditor,
}

impl PianoRoll {
    pub fn show(&mut self, ui: &mut Ui) {
        Window::new("Piano roll")
            .default_size(ui.available_size())
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    Grid::new("piano_roll_grid").show(ui, |ui| {
                        ui.label("Measure");
                        ui.add(
                            DragValue::new(&mut self.editor.signature.measure).range(1..=256),
                        );
                        ui.end_row();
                        ui.label("Value");
                        ui.add(
                            DragValue::new(&mut self.editor.signature.value).range(1..=256),
                        );
                    });
                    ui.vertical(|ui| {
                        ui.label("Keyboard octave");
                        ui.add(DragValue::new(&mut self.piano.kbd_octave).range(0..=(OCTAVES - 1)));
                    });
                    ui.vertical(|ui| {
                        ui.checkbox(&mut self.editor.snapping, "Snap");
                        ui.add_enabled(
                            self.editor.snapping,
                            DragValue::new(&mut self.editor.division).range(1..=64),
                        );
                    });

                    let mut division = if self.editor.snapping {
                        self.editor.division.get()
                    } else {
                        0
                    };
                    let common = COMMON_DIVS
                        .into_iter()
                        .find(|(d, _)| *d == division)
                        .map(|(_, str)| str);
                    ComboBox::from_id_salt("common_snaps")
                        .selected_text(common.unwrap_or("Custom"))
                        .show_ui(ui, |ui| {
                            for (div, str) in COMMON_DIVS {
                                ui.selectable_value(&mut division, div, str);
                            }
                        });

                    self.editor.snapping = division != 0;
                    if let Some(div) = NonZeroUsize::new(division) {
                        self.editor.division = div;
                    }
                });
                ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        self.piano.show(ui);
                        ScrollArea::horizontal().show(ui, |ui| self.editor.show(ui, &self.piano));
                    })
                });
            });
    }
}
