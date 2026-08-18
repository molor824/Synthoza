use std::{iter, sync::LazyLock};

use eframe::egui::*;

const WHITE_KEY: Rgba = Rgba::from_rgb(0.9, 0.9, 0.9);
const WHITE_KEY_CLICK: Rgba = Rgba::from_rgb(0.7, 0.7, 0.7);

const BLACK_KEY: Rgba = Rgba::from_rgb(0.1, 0.1, 0.1);
const BLACK_KEY_CLICK: Rgba = Rgba::from_rgb(0.3, 0.3, 0.3);

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

fn offset_rect(rect: Rect, offset: Vec2) -> Rect {
    Rect {
        min: rect.min + offset,
        max: rect.max + offset,
    }
}

pub struct PianoRoll {
    max_octave: usize,
    hit_key: Option<usize>, // starting at C0 - 0, D0 - 1 etc...
}

impl Default for PianoRoll {
    fn default() -> Self {
        Self {
            max_octave: 10,
            hit_key: None,
        }
    }
}
impl PianoRoll {
    pub fn show(&mut self, ui: &mut Ui) {
        self.hit_key = None;

        ui.horizontal(|ui| {
            let full_rect = PIANO_SIZE * Vec2::new(1.0, (self.max_octave + 1) as f32);
            let (response, painter) = ui.allocate_painter(full_rect, Sense::DRAG | Sense::CLICK);
            let render_rect = response.rect;
            let clicking = response.is_pointer_button_down_on();
            let pointer_at = ui.input(|i| i.pointer.latest_pos());

            for octave in 0..=self.max_octave {
                let start = render_rect.min.to_vec2()
                    + Vec2::new(0.0, PIANO_SIZE.y * (self.max_octave - octave) as f32);

                if self.hit_key.is_none()
                    && clicking
                    && let Some(pointer) = pointer_at
                {
                    self.hit_key = BLACK_RECTS
                        .into_iter()
                        .zip(BLACK_INDICES)
                        .chain(WHITE_RECTS.into_iter().zip(WHITE_INDICES))
                        .find(|(rect, _)| offset_rect(*rect, start).contains(pointer))
                        .map(|(_, idx)| octave * 12 + idx);
                }

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
                        if Some(octave * 12 + idx) == self.hit_key {
                            click_color
                        } else {
                            color
                        },
                    );
                    painter.rect_stroke(
                        rect,
                        0.0,
                        (STROKE_WIDTH, STROKE_COLOR),
                        StrokeKind::Inside,
                    );
                }
            }
        });
    }
}
