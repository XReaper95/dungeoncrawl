use crate::prelude::*;

pub fn entity_render_system(renderables_query: Query<(&Position, &Render)>, camera: Res<Camera>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    for (position, render) in &renderables_query {
        draw_batch.set(position.0 - offset, render.color, render.glyph);
    }

    draw_batch.submit(5000).expect("Batch error");
}
