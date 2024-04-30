use crate::prelude::*;
use crate::utils;

pub fn random_move_system(
    mut commands: Commands,
    random_movers_query: Query<(Entity, &Position), (With<MovingRandomly>, Without<ChasingPlayer>)>,
    player_query: Query<Entity, With<Player>>,
    positions_query: Query<(Entity, &Position), With<Health>>,
) {
    for (random_mover, current_position) in &random_movers_query {
        let mut rng = RandomNumberGenerator::new();
        let mut entity_cmd = commands.entity(random_mover);

        let destination: Point = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + current_position.0;

        // locations can only have a single entity
        if let Some(entity) = utils::get_entity_at_destination(&positions_query, destination) {
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
