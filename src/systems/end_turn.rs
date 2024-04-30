use crate::prelude::*;

pub fn end_turn_system(
    mut turn_state: ResMut<TurnState>,
    player_hp_query: Query<&Health, With<Player>>,
) {
    let player_hp = player_hp_query.single();
    let current_state = *turn_state;
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }

    *turn_state = new_state;
}
