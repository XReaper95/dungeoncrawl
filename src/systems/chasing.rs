use crate::prelude::*;

pub fn chasing_system(
    mut command: Commands,
    map: Res<Map>,
    movers_query: Query<(Entity, &Point), (With<ChasingPlayer>, Without<MovingRandomly>)>,
    player_query: Query<(Entity, &Point), With<Player>>,
) {
    let (player, player_pos) = player_query.single();
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        &*map,
        1024.0
    );
    
    for (mover, movers_pos) in &movers_query {
        let idx = map_idx(movers_pos.x, movers_pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(
            &dijkstra_map, idx, &*map
        ){
            let mut mover_cmd = command.entity(mover);
            
            let distance = DistanceAlg::Pythagoras.distance2d(*movers_pos, *player_pos);
            if distance > 1.2 {
                mover_cmd.insert(WantsToMove { destination: map.index_to_point2d(destination) });
            } else {
                mover_cmd.insert(WantsToAttack { victim: player });
            };
        }
    }
}
