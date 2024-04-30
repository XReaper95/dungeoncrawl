mod map;
mod components;
mod map_builder;
mod camera;
mod spawner;
mod systems;
mod turn_state;
mod utils;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use bevy_ecs::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::KeyEvent;
}

use prelude::*;

struct State {
    ecs : World,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        ecs.insert_resource(map_builder.map);
        ecs.insert_resource(Camera::new(map_builder.player_start));
        ecs.insert_resource(TurnState::AwaitingInput);
        ecs.insert_resource(Events::<KeyEvent>::default());

        Self {
            ecs,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler()
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK,
                                 "Slain by a monster, your hero's journey has come to a \
            premature end.");
        ctx.print_color_centered(5, WHITE, BLACK,
                                 "The Amulet of Yal remains unclaimed, and your home town \
            is not saved.");
        ctx.print_color_centered(8, YELLOW, BLACK,
                                 "Don't worry, you can always try again with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK,
                                 "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.ecs.clear_entities();
            self.ecs.clear_resources();
            self.ecs.clear_trackers();
            let mut rng = RandomNumberGenerator::new();
            let map_builder = MapBuilder::new(&mut rng);
            spawn_player(&mut self.ecs, map_builder.player_start);
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
            map_builder.rooms
                .iter()
                .skip(1)
                .map(|r| r.center())
                .for_each(|pos| spawn_monster(&mut self.ecs, &mut rng, pos));
            self.ecs.insert_resource(map_builder.map);
            self.ecs.insert_resource(Camera::new(map_builder.player_start));
            self.ecs.insert_resource(TurnState::AwaitingInput);
            self.ecs.insert_resource(Events::<KeyEvent>::default());
        }
    }
}

#[derive(Event)]
pub struct KeyEvent {
    pub key_code: VirtualKeyCode
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear screen
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        if let Some(key_code) = ctx.key {
            self.ecs.send_event(KeyEvent { key_code });
        }

        ctx.set_active_console(0);
        self.ecs.insert_resource(MousePosition(Point::from_tuple(ctx.mouse_pos())));

        let current_state = *self.ecs.get_resource::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self.input_systems.run(&mut self.ecs),
            TurnState::PlayerTurn => self.player_systems.run(&mut self.ecs),
            TurnState::MonsterTurn => self.monster_systems.run(&mut self.ecs),
            TurnState::GameOver => self.game_over(ctx)
        }

        render_draw_buffer(ctx).expect("Failed to render");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(
            DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png"
        )
        .with_simple_console_no_bg(
            DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")

        .with_simple_console_no_bg(
            SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png"
        )
        .build()?;

    main_loop(context, State::new())
}
