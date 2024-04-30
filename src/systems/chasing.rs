use crate::prelude::*;
use crate::utils;

pub fn chasing_system(
    mut command: Commands,
    map: Res<Map>,
    chasers_query: Query<(Entity, &Position), (With<ChasingPlayer>, Without<MovingRandomly>)>,
    player_query: Query<(Entity, &Position), With<Player>>,
    positions_query: Query<(Entity, &Position), With<Health>>,
) {
    let (player, player_pos) = player_query.single();
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map =
        DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, &*map, 1024.0);

    chasers_query.iter().for_each(|(chaser, chaser_pos)| {
        let idx = map_idx(chaser_pos.x, chaser_pos.y);
        if let Some(exit_position) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, &*map) {
            let distance = DistanceAlg::Pythagoras.distance2d(chaser_pos.0, player_pos.0);
            let destination = if distance > 1.2 {
                map.index_to_point2d(exit_position)
            } else {
                player_pos.0
            };

            let mut chaser_cmd = command.entity(chaser);

            if let Some(entity) = utils::get_entity_at_destination(&positions_query, destination) {
                if entity == player {
                    chaser_cmd.insert(WantsToAttack { victim: player });
                }
                // if entity is not the player, do nothing
            } else {
                chaser_cmd.insert(WantsToMove { destination });
            }
        }
    });
}
