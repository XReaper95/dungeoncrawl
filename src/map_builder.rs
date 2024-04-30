use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        mb._fill(TileType::Wall);
        mb._build_random_rooms(rng);
        mb._build_corridors(rng);
        mb.player_start = mb.rooms[0].center();

        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[mb.map.point2d_to_index(mb.player_start)],
            &mb.map,
            1024.0,
        );
        const UNREACHABLE: &f32 = &f32::MAX;
        mb.amulet_start = mb.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        );

        mb
    }

    fn _fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn _build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
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
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }

    fn _apply_tunnel(&mut self, a1: i32, a2: i32, b: i32, vertical: bool) {
        use std::cmp::{max, min};
        for y in min(a1, a2)..=max(a1, a2) {
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

    fn _build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self._apply_tunnel(prev.x, new.x, prev.y, false);
                self._apply_tunnel(prev.y, new.y, new.x, true);
            } else {
                self._apply_tunnel(prev.y, new.y, prev.x, true);
                self._apply_tunnel(prev.x, new.x, new.y, false);
            }
        }
    }
}
