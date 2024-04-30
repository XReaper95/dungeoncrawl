use crate::prelude::*;

pub fn tooltips_system(
    mouse_pos: Res<MousePosition>,
    camera: Res<Camera>,
    entity_query: Query<(&Position, &Name, Option<&Health>)>
) {
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = mouse_pos.0 + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    
    for (_, name, health_opt) in entity_query.iter().filter(|(pos, _, _)| pos.0 == map_pos) {
        let screen_pos = mouse_pos.0 * 4;
        
        let display = if let Some(health) = health_opt {
            format!("{} : {} hp", &name.0, health.current)
        } else {
            name.0.clone()
        };
  
        draw_batch.print(screen_pos, &display);
    }

    draw_batch.submit(10100).expect("Batch error");
}