use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------",
    12,
    11,
);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let dijstra_map = build_dijkstra(&[mb.map.point2d_to_index(mb.player_start)], &mb.map);
    let mut placement = None;
    let mut attempts = 0;

    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );

        let mut can_place = false;
        dimensions.for_each(|point| {
            let idx = mb.map.point2d_to_index(point);
            let distance = dijstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != point {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|point| !points.contains(point));
        }
        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .0
            .chars()
            .filter(|character| *character != '\r' && *character != '\n')
            .collect();

        let mut char_idx = 0;

        for ty in placement.y..placement.y + FORTRESS.2 {
            for tx in placement.x..placement.x + FORTRESS.1 {
                let tile_idx = map_idx(tx, ty);
                let character = string_vec[char_idx];

                match character {
                    'M' => {
                        mb.map.tiles[tile_idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    '-' => mb.map.tiles[tile_idx] = TileType::Floor,
                    '#' => mb.map.tiles[tile_idx] = TileType::Wall,
                    _ => println!("No idea what to do with [{}]", character),
                }

                char_idx += 1;
            }
        }
    }
}
