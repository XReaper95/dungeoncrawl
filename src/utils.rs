use crate::prelude::*;

pub fn get_entity_at_destination(positions_query: &Query<(Entity, &Position), With<Health>>, destination: Point) -> Option<Entity> {
    positions_query.iter().filter_map(
        |(entt, pos)| if pos.0 == destination { Some(entt) } else { None }
    ).next()
}
