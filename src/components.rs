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

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct ChasingPlayer;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct AmuletOfYala;
