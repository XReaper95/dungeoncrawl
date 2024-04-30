use bevy_ecs::event::event_update_system;

use crate::prelude::*;

mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

pub fn build_input_scheduler() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        (
            event_update_system::<KeyEvent>,
            player_input::player_input_system,
        )
            .chain(),
        map_render::map_render_system,
        entity_render::entity_render_system,
        hud::hud_system,
        tooltips::tooltips_system,
    ));

    schedule
}

pub fn build_player_scheduler() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        (combat::combat_system, movement::movement_system).chain(),
        map_render::map_render_system,
        entity_render::entity_render_system,
        hud::hud_system,
        end_turn::end_turn_system,
    ));

    schedule
}

pub fn build_monster_scheduler() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        (
            (random_move::random_move_system, chasing::chasing_system),
            combat::combat_system,
            movement::movement_system,
        )
            .chain(),
        map_render::map_render_system,
        entity_render::entity_render_system,
        hud::hud_system,
        end_turn::end_turn_system,
    ));

    schedule
}
