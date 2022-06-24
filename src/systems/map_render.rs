use crate::camera::Camera;
use crate::map::{in_bounds, map_idx, Map, TileType};
use bracket_lib::prelude::{to_cp437, ColorPair, DrawBatch, Point, BLACK, WHITE};
use legion::system;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let point = Point::new(x, y);

            if in_bounds(point) {
                let offset = Point::new(camera.left_x, camera.top_y);
                let idx = map_idx(point);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                draw_batch.set(point - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error!");
}
