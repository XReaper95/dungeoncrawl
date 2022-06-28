use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map : Map,
    pub rooms : Vec<Rect>,
    pub player_start : Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms : Vec::new(),
            player_start : Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile : TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng : &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                                       rng.range(1, SCREEN_WIDTH - 10),
                                       rng.range(1, SCREEN_HEIGHT - 10),
                                       rng.range(2, 10),
                                       rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0
                        && p.y < SCREEN_HEIGHT
                    {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }

    fn apply_tunnel(&mut self, a1:i32, a2:i32, b:i32, vertical: bool) {
        use std::cmp::{min, max};
        for y in min(a1,a2) ..= max(a1,a2) {
            let new_point = if vertical {
                Point::new(b, y)
            } else {
                Point::new(y, b)
            };

            if let Some(idx) = self.map.try_idx(new_point) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = room.center();

            if rng.range(0,2) == 1 {
                self.apply_tunnel(prev.x, new.x, prev.y, false);
                self.apply_tunnel(prev.y, new.y, new.x, true);
            } else {
                self.apply_tunnel(prev.y, new.y, prev.x, true);
                self.apply_tunnel(prev.x, new.x, new.y, false);
            }
        }
    }
}