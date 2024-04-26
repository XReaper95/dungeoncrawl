pub use crate::prelude::*;

#[derive(Component)]
pub struct Render {
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MovingRandomly;

#[derive(Component)]
pub struct WantsToMove {
    pub destination : Point
}

#[derive(Component)]
pub struct WantsToAttack {
    pub victim : Entity
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

impl Health {
    pub fn new(initial_value: i32) -> Self {
        Self {
            current: initial_value,
            max: initial_value
        }
    }
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct ChasingPlayer;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct AmuletOfYala;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub name: Name,
    pub position: Point,
    pub render_data: Render,
    pub health: Health,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub marker: Enemy,
    pub name: Name,
    pub position: Point,
    pub render_data: Render,
    pub health: Health,
    pub chases_player: ChasingPlayer,
}

#[derive(Bundle)]
pub struct AmuletBundle {
    pub marker: AmuletOfYala,
    pub item_marker: Item,
    pub name: Name,
    pub position: Point,
    pub render_data: Render,
}