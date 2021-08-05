use crate::prelude::*;

pub fn build_dijkstra(starts: &[usize], map: &Map) -> DijkstraMap {
    DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, starts, map, 1024.0)
}
