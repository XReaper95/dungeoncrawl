use crate::prelude::*;

pub fn combat_system(
    mut commands: Commands,
    attackers_query: Query<(Entity, &WantsToAttack)>,
    player_query: Query<Entity, With<Player>>,
    mut possible_victims_query: Query<&mut Health>,
) {
    let player = player_query.single();

    for (attacker, attack) in &attackers_query {
        if let Ok(mut health) = possible_victims_query.get_mut(attack.victim) {
            health.current -= 1;

            if health.current < 1 && attack.victim != player {
                commands.entity(attack.victim).despawn();
            }
        }

        commands.entity(attacker).remove::<WantsToAttack>();
    }
}
