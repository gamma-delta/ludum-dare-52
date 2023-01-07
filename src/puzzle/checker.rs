use hex2d::{Angle, Coordinate, Direction};

use super::{Board, Puzzle};

impl Board {
    pub fn is_solved(&self, puzzle: &Puzzle) -> Result<(), FailureReason> {
        if self.find_euler_path().is_none() {
            return Err(FailureReason::NotSinglePass);
        }

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
                println!(
                    "checking {:?} from {:?} ({})",
                    dir, anchor, centered_idx
                );

                let mut scanner = 0;
                let mut popcnt = 0;
                // Scan across ...
                'across: for j in
                    0..=(puzzle.radius * 2 - centered_idx.unsigned_abs())
                {
                    println!("> {j}");
                    let coord = anchor + Coordinate::from(dir).scale(j as i32);
                    let junction_count = self.get_junction_count(coord);
                    if junction_count == 0 {
                        // empty cells are freebies
                        continue 'across;
                    }

                    popcnt += 1;

                    if j as usize >= markset.len() {
                        // there's more junctions here than the plan called for
                        println!(
                            "failed at {:?} idx {},{}, ran out",
                            dir, i, j
                        );
                        return Err(FailureReason::CountFailed);
                    }

                    let wanted = markset[scanner as usize].get();
                    if junction_count != wanted {
                        // this junction doesn't match
                        println!(
                            "failed at {:?} idx {},{}, wanted {} found {} (slot {})",
                            dir, i, j, wanted, junction_count, scanner
                        );
                        return Err(FailureReason::CountFailed);
                    }
                    // then we've found the next step in the plan
                    scanner += 1;
                }

                if markset.len() != popcnt {
                    // then we're missing a junction
                    println!(
                        "failed at {:?} idx {}, wanted {} junctions, found {}",
                        dir,
                        i,
                        markset.len(),
                        popcnt,
                    );
                    return Err(FailureReason::CountFailed);
                }
            }
        }

        // made it through it all!
        Ok(())
    }

    pub fn find_euler_path(&self) -> Option<Vec<Coordinate>> {
        // TODO
        Some(Vec::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureReason {
    NotSinglePass,
    CountFailed,
}
