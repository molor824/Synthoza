use std::sync::LazyLock;

use eframe::egui::{Pos2, Rect, Rgba, Sense, Ui, Vec2};

const WHITE_KEY: Rgba = Rgba::from_rgb(0.9, 0.9, 0.9);
const WHITE_KEY_HOVER: Rgba = Rgba::from_rgb(0.7, 0.7, 0.7);

const BLACK_KEY: Rgba = Rgba::from_rgb(0.1, 0.1, 0.1);
const BLACK_KEY_HOVER: Rgba = Rgba::from_rgb(0.3, 0.3, 0.3);

const KEY_WIDTH: f32 = 20.0;
const KEY_HEIGHT: f32 = 50.0;
const BLACK_KEY_HEIGHT: f32 = 30.0;

const PIANO_SIZE: Vec2 = Vec2::new(KEY_HEIGHT, KEY_WIDTH * 12.0);
const EXPAND_SIZE: f32 = -1.0;

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

fn offset_rect(rect: Rect, offset: Vec2) -> Rect {
    Rect {
        min: rect.min + offset,
        max: rect.max + offset,
    }
}

fn show_piano(ui: &mut Ui) -> Option<usize> {
    let mut key_hit = None;
    let (response, painter) = ui.allocate_painter(PIANO_SIZE, Sense::empty());
    let render_rect = response.rect;
    let pointer_pos = ui.input(|i| i.pointer.hover_pos());
    let pointer_down = ui.input(|i| i.pointer.primary_down());

    if pointer_down && let Some(pos) = pointer_pos {
        for (rect, idx) in BLACK_RECTS
            .into_iter()
            .chain(*WHITE_RECTS)
            .zip(BLACK_INDICES.into_iter().chain(WHITE_INDICES))
        {
            let screen_rect = offset_rect(rect, render_rect.min.to_vec2());
            if screen_rect.contains(pos) {
                key_hit = Some(idx);
                break;
            }
        }
    }

    for (rect, idx) in WHITE_RECTS.into_iter().zip(WHITE_INDICES) {
        painter.rect_filled(
            offset_rect(rect, render_rect.min.to_vec2()).expand(EXPAND_SIZE),
            0.0,
            if key_hit == Some(idx) {
                WHITE_KEY_HOVER
            } else {
                WHITE_KEY
            },
        );
    }
    for (rect, idx) in BLACK_RECTS.into_iter().zip(BLACK_INDICES) {
        painter.rect_filled(
            offset_rect(rect, render_rect.min.to_vec2()).expand(EXPAND_SIZE),
            0.0,
            if key_hit == Some(idx) {
                BLACK_KEY_HOVER
            } else {
                BLACK_KEY
            },
        );
    }

    key_hit
}

pub struct PianoRoll {
    min_octave: usize,
    max_octave: usize,
}

impl Default for PianoRoll {
    fn default() -> Self {
        Self {
            min_octave: 0,
            max_octave: 10,
        }
    }
}
impl PianoRoll {
    pub fn show(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            for _ in self.min_octave..=self.max_octave {
                show_piano(ui);
            }
        });
    }
}
