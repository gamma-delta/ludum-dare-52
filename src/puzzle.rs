use std::{collections::hash_map, num::NonZeroU8};

use ahash::{AHashMap, AHashSet};
use hex2d::Coordinate;
use serde::Deserialize;

use crate::geom::{EdgePos, EdgeSet};

#[derive(Debug, Deserialize)]
pub struct Puzzle {
    radius: u32,
    marks: [Vec<Vec<NonZeroU8>>; 3],
    #[serde(default)]
    water: AHashSet<Coordinate>,
}

impl Puzzle {
    pub fn radius(&self) -> u32 {
        self.radius
    }

    pub fn marks(&self) -> [&Vec<Vec<NonZeroU8>>; 3] {
        self.marks.each_ref()
    }

    pub fn has_water(&self, coord: Coordinate) -> bool {
        self.water.contains(&coord)
    }

    /// can the alien go over there
    pub fn is_valid(&self, coord: Coordinate) -> bool {
        coord.distance(Coordinate::new(0, 0)) <= self.radius as i32
            && !self.water.contains(&coord)
    }
}

#[derive(Deserialize)]
pub struct Level {
    pub puzzle: Puzzle,
    pub title: String,
}

pub struct Board {
    paths: AHashMap<Coordinate, EdgeSet>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            paths: AHashMap::new(),
        }
    }

    pub fn can_twiddle_path(&self, puzzle: &Puzzle, edge: EdgePos) -> bool {
        let [a, b] = edge.spans();
        puzzle.is_valid(a) && puzzle.is_valid(b)
    }

    /// Get whether the path is there, missing, or invalid
    pub fn get_path(&self, edge: EdgePos) -> Option<bool> {
        Some(self.paths.get(&edge.coord)?.contains(edge.edge))
    }

    /// Return the old value of the path
    pub fn set_path(
        &mut self,
        puzzle: &Puzzle,
        edge: EdgePos,
        newval: bool,
    ) -> Option<bool> {
        if !self.can_twiddle_path(puzzle, edge) {
            None
        } else if newval {
            let here = self.paths.entry(edge.coord).or_default();
            let prev = here.contains(edge.edge);
            here.insert(edge.edge);
            Some(prev)
        } else if let hash_map::Entry::Occupied(mut entry) =
            self.paths.entry(edge.coord)
        {
            let here = entry.get_mut();
            let prev = here.contains(edge.edge);
            here.remove(edge.edge);
            if here.is_empty() {
                entry.remove_entry();
            }
            Some(prev)
        } else {
            // as an optimization don't bother adding and then immediately
            // removing when removing. there was nothing here.
            Some(false)
        }
    }

    pub fn get_paths(&self, coord: Coordinate) -> EdgeSet {
        self.paths.get(&coord).copied().unwrap_or_default()
    }
}
