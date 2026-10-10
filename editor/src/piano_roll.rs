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
    pub kbd_octave: usize,
    pub mouse_key: Option<usize>, // starting at C0 - 0, D0 - 1 etc...
    pub kbd_keys: [bool; KEYBOARD_KEYS.len()], // Keyboard keys ordered, and represents its equivalent indexed notes.
    pub key_height: f32,
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

pub enum NoteDrag {
    New { base: f32, end: f32, key: usize },
}

pub struct NoteEditor {
    pub notes: Vec<Note>, // Must be ordered by Note::begin!!!
    pub signature: TimeSignature,
    pub bars: usize,
    pub whole_note_width: f32, // Measured in whole note
    pub division: NonZeroUsize,
    pub snapping: bool,
    pub current_note_duration: f32,
    pub note_drag: Option<NoteDrag>,
    pub prev_click_state: bool,
}
impl Default for NoteEditor {
    fn default() -> Self {
        Self {
            notes: vec![],
            signature: TimeSignature::default(),
            bars: 4,
            whole_note_width: 200.0,
            snapping: true,
            division: NonZeroUsize::new(4).unwrap(),
            current_note_duration: 0.25, // quarter note by default (most common note probably)
            note_drag: None,
            prev_click_state: false,
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

const GHOST_NOTE_COLOR: Color32 = Color32::from_rgba_unmultiplied_const(255, 255, 255, 100);

const NOTE_MIN_DURATION: f32 = 0.001;

impl NoteEditor {
    fn show_grid(&mut self, ui: &mut Ui, piano: &Piano) -> (Response, Painter) {
        let value_width = self.whole_note_width / self.signature.value.get() as f32;
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

        (response, painter)
    }
    pub fn show(&mut self, ui: &mut Ui, piano: &Piano) {
        let (response, painter) = self.show_grid(ui, piano);
        let clicking = response.is_pointer_button_down_on()
            && response
                .ctx
                .input(|i| i.pointer.button_down(PointerButton::Primary));
        let snap = (self.signature.value.get() * self.division.get()) as f32;
        let just_clicked = !self.prev_click_state && clicking;
        let just_released = self.prev_click_state && !clicking;
        self.prev_click_state = clicking;

        let notes_len = self.notes.len();
        for (i, note) in self.notes.iter().enumerate() {
            let mut note1 = *note;
            note1.paint(
                &mut false,
                &mut false,
                &response,
                &painter,
                vec2(self.whole_note_width, piano.key_height),
                Rgba::from_white_alpha((i + 1) as f32 / notes_len as f32).into(),
            );
        }

        if let Some(pos) = response.hover_pos() {
            let relative_pos = pos - response.rect.min;
            let note_pos = relative_pos.x / self.whole_note_width;
            let start = if self.snapping {
                (note_pos * snap).round() / snap
            } else {
                note_pos
            };
            let note = Note {
                start,
                duration: self.current_note_duration,
                key: (relative_pos.y / piano.key_height) as usize,
            };

            if just_released {
                let drag = self.note_drag.take();
                self.add_note(match drag {
                    Some(NoteDrag::New { base, end, key })
                        if (base - end).abs() > NOTE_MIN_DURATION =>
                    {
                        Note::from_points(base, end, key)
                    }
                    _ => note,
                });
            } else {
                let mut note = note;
                if just_clicked {
                    self.note_drag = Some(NoteDrag::New {
                        base: start,
                        end: start,
                        key: note.key,
                    });
                }

                if let Some(drag) = self.note_drag.as_mut() {
                    match drag {
                        NoteDrag::New { end, base, key } => {
                            *end = if self.snapping {
                                (note_pos * snap).round() / snap
                            } else {
                                note_pos
                            };
                            if (*base - *end).abs() > NOTE_MIN_DURATION {
                                note = Note::from_points(*base, *end, *key);
                            }
                            note.key = *key;
                        }
                    }
                }

                note.paint(
                    &mut false,
                    &mut false,
                    &response,
                    &painter,
                    vec2(self.whole_note_width, piano.key_height),
                    GHOST_NOTE_COLOR,
                );
            }
        }
    }
    fn add_note(&mut self, new_note: Note) {
        if new_note.duration <= NOTE_MIN_DURATION {
            return;
        }
        let mut idx = 0;
        while idx < self.notes.len() {
            let note = self.notes[idx];
            if note.end() < new_note.start {
                idx += 1;
                continue;
            } else if note.start > new_note.end() { // insert
                self.notes.insert(idx, new_note);
                return; // not worth going any further
            }
            let (left, right) = note.overlap_other(&new_note);

            if let Some(r) = right {
                // everything past is already past the new note, return from this point on
                if let Some(l) = left {
                    // everything exists, replace all at once
                    self.notes.splice(idx..=idx, [l, new_note, r]);
                } else {
                    self.notes.splice(idx..=idx, [new_note, r]);
                }
                return; // anything past is not worth computing
            }
            if let Some(l) = left { // no right, but left
                self.notes[idx] = l;
                idx += 1;
            } else { // nothing, remove the note entirely
                self.notes.remove(idx);
            }
        }
        // if you reached this, then it was always left and no right overlapping note occured
        // thus making new_note the last one
        self.notes.push(new_note);
    }
}

const NOTE_DRAG_DISTANCE: f32 = 5.0;

#[derive(Debug, Copy, Clone)]
pub struct Note {
    pub start: f32, // measured in notes (1.0 - one whole note)
    pub duration: f32,
    pub key: usize,
}

impl Note {
    pub const fn from_end(start: f32, end: f32, key: usize) -> Self {
        assert!(start <= end, "start cannot be more than the end");
        Self {
            start,
            key,
            duration: end - start,
        }
    }
    pub const fn from_points(a: f32, b: f32, key: usize) -> Self {
        Self::from_end(a.min(b), a.max(b), key)
    }
    pub fn overlap_other(&self, other: &Note) -> (Option<Note>, Option<Note>) {
        (
            if self.start < other.start {
                Some(Note::from_end(
                    self.start,
                    self.end().min(other.start),
                    self.key,
                ))
            } else {
                None
            },
            if self.end() > other.end() {
                Some(Note::from_end(
                    self.start.max(other.end()),
                    self.end(),
                    self.key,
                ))
            } else {
                None
            },
        )
    }
    pub const fn end(&self) -> f32 {
        self.start + self.duration
    }
    pub fn paint(
        &mut self,
        begin_drag: &mut bool,
        end_drag: &mut bool,
        response: &Response,
        painter: &Painter,
        whole_note_size: Vec2,
        color: Color32,
    ) {
        let draw_rect = Rect::from_min_size(
            response.rect.min + vec2(self.start, self.key as f32) * whole_note_size,
            vec2(self.duration, 1.0) * whole_note_size,
        );

        if response.dragged_by(PointerButton::Primary)
            && let Some(pos) = response.interact_pointer_pos()
        {
            let drag_bounding_size = Vec2::splat(NOTE_DRAG_DISTANCE);
            if !*end_drag && !*begin_drag {
                if Rect::from_min_max(
                    pos2(draw_rect.max.x, draw_rect.min.y) - drag_bounding_size,
                    draw_rect.max + drag_bounding_size,
                )
                .contains(pos)
                {
                    *end_drag = true;
                } else if Rect::from_min_max(
                    draw_rect.min - drag_bounding_size,
                    pos2(draw_rect.min.x, draw_rect.max.y) + drag_bounding_size,
                )
                .contains(pos)
                {
                    *begin_drag = true;
                }
            }
            let relative_pos = pos - response.rect.min;
            if *end_drag {
                self.duration = (relative_pos.x / whole_note_size.x - self.start).max(0.0);
            } else if *begin_drag {
                let end = self.end();
                self.start = (relative_pos.x / whole_note_size.x).min(end);
                self.duration = end - self.start;
            }
        } else {
            *begin_drag = false;
            *end_drag = false;
        }

        painter.rect_filled(draw_rect, 5, color);
    }
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
    pub piano: Piano,
    pub editor: NoteEditor,
}

impl PianoRoll {
    pub fn show(&mut self, ui: &mut Ui) {
        Window::new("Piano roll")
            .default_size(ui.available_size())
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    Grid::new("piano_roll_grid").show(ui, |ui| {
                        ui.label("Measure");
                        ui.add(DragValue::new(&mut self.editor.signature.measure).range(1..=256));
                        ui.end_row();
                        ui.label("Value");
                        ui.add(DragValue::new(&mut self.editor.signature.value).range(1..=256));
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
