use crate::prelude::*;

pub fn random_move_system(
    mut commands: Commands,
    random_movers_query: Query<(Entity, &Point), (With<MovingRandomly>, Without<ChasingPlayer>)>,
    player_query: Query<Entity, With<Player>>,
    positions_query: Query<(Entity, &Point), With<Health>>
) {
    for (random_mover, current_position) in &random_movers_query {
        let mut rng = RandomNumberGenerator::new();
        let mut entity_cmd = commands.entity(random_mover);

        let destination: Point = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *current_position;
        
        // locations can only have a single entity
        let at_destination = positions_query.iter().filter_map(
            |(entt, pos)| if *pos == destination { Some(entt) } else { None }
        ).next();
        if let Some(entity) = at_destination {
            let player = player_query.single();
            if entity == player {
                entity_cmd.insert(WantsToAttack { victim: player });
            }
            // if entity is not the player, do nothing
        } else {
            entity_cmd.insert(WantsToMove { destination });
        }
    }
}
