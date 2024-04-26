use crate::prelude::*;

pub fn random_move_system(
    mut commands: Commands,
    movers_query: Query<(Entity, &Point), (With<MovingRandomly>, Without<Player>, Without<ChasingPlayer>)>,
    player_query: Query<(Entity, &Point), With<Player>>,
) {
    let (player_entity, player_position) = player_query.single();

    for (entity, position) in &movers_query {
        let mut rng = RandomNumberGenerator::new();

        let destination: Point = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *position;

        let mut entity_cmd = commands.entity(entity);

        if destination == (*player_position).into() {
            entity_cmd.insert(WantsToAttack { victim: player_entity });
        } else {
            entity_cmd.insert(WantsToMove { destination });
        }
    }
}
