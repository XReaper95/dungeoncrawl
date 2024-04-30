use crate::prelude::*;

pub fn movement_system(
    mut commands: Commands,
    movables_query: Query<(Entity, &WantsToMove)>,
    player_entity_query: Query<Entity, With<Player>>,
    map: Res<Map>,
    mut camera: ResMut<Camera>,
) {
    for (entity, want_move) in &movables_query {
        let mut entity_cmds = commands.entity(entity);

        if map.can_enter_tile(want_move.destination) {
            entity_cmds.insert(Position(want_move.destination));

            if entity == player_entity_query.single() {
                camera.on_player_move(want_move.destination)
            }
        }

        entity_cmds.remove::<WantsToMove>();
    }
}
