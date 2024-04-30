use crate::prelude::*;

pub fn player_input_system(
    mut commands: Commands,
    mut event_reader: EventReader<KeyEvent>,
    mut turn_state: ResMut<TurnState>,
    mut player_query: Query<(Entity, &Position, &mut Health), With<Player>>,
    enemies_query: Query<(Entity, &Position), With<Enemy>>,
) {
    for key_event in event_reader.read() {
        let delta = match key_event.key_code {
            VirtualKeyCode::H => Point::new(-1, 0),
            VirtualKeyCode::L => Point::new(1, 0),
            VirtualKeyCode::K => Point::new(0, -1),
            VirtualKeyCode::J => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (player, player_position, mut player_health) = player_query.single_mut();
        let mut player_cmd = commands.entity(player);

        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            let player_destination = player_position.0 + delta;
            let mut hit_something = false;

            for (enemy, enemy_position) in &enemies_query {
                if player_destination == enemy_position.0 {
                    hit_something = true;
                    did_something = true;

                    player_cmd.insert(WantsToAttack { victim: enemy });
                    break;
                }
            }

            if !hit_something {
                did_something = true;
                player_cmd.insert(WantsToMove { destination: player_destination });
            }
        }

        if !did_something {
            player_health.current = i32::min(player_health.max, player_health.current + 1);
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

