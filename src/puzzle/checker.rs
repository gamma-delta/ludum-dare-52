use ahash::AHashMap;
use hex2d::{Angle, Coordinate, Direction};
use itertools::Itertools;

use crate::geom::EdgePos;

use super::{Board, Puzzle};

impl Board {
    pub fn is_solved(&self, puzzle: &Puzzle) -> Result<(), FailureReason> {
        let Some(euler) = self.find_euler_path() else {
            return Err(FailureReason::NotSinglePass)
        };
        // println!("{:?}", euler);

        // for each flank of the level
        for (marks, dir) in puzzle.marks.iter().zip([
            Direction::XY,
            Direction::YZ,
            Direction::ZX,
        ]) {
            // scan the flank
            'side: for (i, markset) in marks.iter().enumerate() {
                if markset.is_empty() {
                    // nothing marked = anything goes
                    continue 'side;
                }
                // ... -2, -1, 0, 1, 2 ...
                let centered_idx = i as i32 - puzzle.radius as i32;
                let side_center = Coordinate::new(0, 0)
                    - (Coordinate::from(dir).scale(puzzle.radius as i32));
                let offset = Coordinate::from(
                    dir + if centered_idx > 0 {
                        Angle::Right
                    } else {
                        Angle::Left
                    },
                )
                .scale(centered_idx.abs());
                let anchor = side_center + offset;
                //         println!(
                //             "checking {:?} from {:?} ({})",
                //             dir, anchor, centered_idx
                //         );

                let mut scanner = 0;
                let mut popcnt = 0;
                // Scan across ...
                'across: for j in
                    0..=(puzzle.radius * 2 - centered_idx.unsigned_abs())
                {
                    //             println!("> {j}");
                    let coord = anchor + Coordinate::from(dir).scale(j as i32);
                    let junction_count = self.get_junction_count(coord);
                    if junction_count == 0 {
                        // empty cells are freebies
                        continue 'across;
                    }

                    popcnt += 1;

                    if j as usize >= markset.len() {
                        // there's more junctions here than the plan called for
                        //                 println!(
                        //                     "failed at {:?} idx {},{}, ran out",
                        //                     dir, i, j
                        //                 );
                        return Err(FailureReason::CountFailed);
                    }

                    let wanted = markset[scanner as usize].get();
                    if junction_count != wanted {
                        // this junction doesn't match
                        //                 println!(
                        //                     "failed at {:?} idx {},{}, wanted {} found {} (slot {})",
                        //                     dir, i, j, wanted, junction_count, scanner
                        //                 );
                        return Err(FailureReason::CountFailed);
                    }
                    // then we've found the next step in the plan
                    scanner += 1;
                }

                if markset.len() != popcnt {
                    // then we're missing a junction
                    //             println!(
                    //                 "failed at {:?} idx {}, wanted {} junctions, found {}",
                    //                 dir,
                    //                 i,
                    //                 markset.len(),
                    //                 popcnt,
                    //             );
                    return Err(FailureReason::CountFailed);
                }
            }
        }

        // made it through it all!
        Ok(())
    }

    /// https://github.com/gamma-delta/HexMod/blob/main/Common/src/main/java/at/petrak/hexcasting/api/spell/math/EulerPathFinder.kt
    pub fn find_euler_path(&self) -> Option<Vec<Coordinate>> {
        let mut graph = make_graph(self);
        if graph.is_empty() {
            return None;
        }

        let odd_nodes = graph
            .iter()
            .filter_map(|(coord, edges)| {
                if edges.count_ones() % 2 == 1 {
                    Some(*coord)
                } else {
                    None
                }
            })
            .collect_vec();
        let mut current = match odd_nodes.len() {
            0 => *graph.keys().next().unwrap(),
            2 => odd_nodes[0],
            _ => return None,
        };

        let mut stack = Vec::new();
        let mut out = Vec::new();
        // hacking a do-while loop, sorry
        while {
            //     println!("at {:?}", current);
            let edges = graph.get_mut(&current).unwrap();
            //     println!("edges {:06b}", *edges);
            if *edges == 0 {
                out.push(current);
                current = stack.pop().unwrap();
            } else {
                stack.push(current);

                let mut a_dir = None;
                'dirs: for &dir in Direction::all() {
                    let mask = 1 << dir as u8;
                    if (*edges & mask) != 0 {
                        a_dir = Some(dir);
                        break 'dirs;
                    }
                }
                let burn_dir = a_dir.unwrap();

                *edges &= !(1 << burn_dir as u8);
                if let Some(facing) = graph.get_mut(&(current + burn_dir)) {
                    *facing &= !(1 << (burn_dir + Angle::Back) as u8);
                }
                current = current + burn_dir;
            }

            let graph_ok = match graph.get(&current) {
                Some(it) => *it != 0,
                None => false,
            };
            graph_ok || !stack.is_empty()
        } {}
        out.push(current);

        Some(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureReason {
    NotSinglePass,
    CountFailed,
}

fn make_graph(board: &Board) -> AHashMap<Coordinate, u8> {
    let mut out = AHashMap::new();
    for &coord in board.paths.keys() {
        for &dir in Direction::all() {
            if board.get_path(EdgePos::new(coord, dir)) == Some(true) {
                *out.entry(coord).or_default() |= 1 << dir as u8;
                *out.entry(coord + dir).or_default() |=
                    1 << (dir + Angle::Back) as u8;
            }
        }
    }

    out
}
