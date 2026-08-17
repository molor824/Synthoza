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
        rect.expand(EXPAND_SIZE)
    })
});
static BLACK_RECTS: LazyLock<[Rect; 5]> = LazyLock::new(|| {
    let positions = [1.0, 3.0, 5.0, 8.0, 10.0].map(|y| Pos2::new(0.0, KEY_WIDTH * y));

    std::array::from_fn(|idx| {
        let pos = positions[idx];
        Rect::from_min_size(pos, Vec2::new(BLACK_KEY_HEIGHT, KEY_WIDTH)).expand(EXPAND_SIZE)
    })
});

fn show_piano(ui: &mut Ui) -> Option<usize> {
    let mut key_hit = None;
    let (response, painter) = ui.allocate_painter(PIANO_SIZE, Sense::hover() | Sense::click());

    for rect in WHITE_RECTS.into_iter() {
        painter.rect_filled(rect, 0.0, WHITE_KEY);
    }
    for rect in BLACK_RECTS.into_iter() {
        painter.rect_filled(rect, 0.0, BLACK_KEY);
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
            for _ in self.min_octave..=self.max_octave {
                show_piano(ui);
            }
        });
    }
}
