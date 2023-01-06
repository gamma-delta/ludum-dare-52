use enumflags2::{bitflags, BitFlags};
use hex2d::{Coordinate, Direction};

pub type EdgeSet = BitFlags<HexEdge>;

/// Hex direction but only for the 3 directions we track on the coord
#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HexEdge {
    /// Right
    XY,
    /// Down-right
    ZY,
    /// Down-left
    ZX,
}

impl HexEdge {
    pub fn to_hex2d(&self) -> Direction {
        match self {
            HexEdge::XY => Direction::XY,
            HexEdge::ZY => Direction::ZY,
            HexEdge::ZX => Direction::ZX,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgePos {
    pub coord: Coordinate,
    pub edge: HexEdge,
}

impl EdgePos {
    pub fn new(coord: Coordinate, edge: HexEdge) -> Self {
        Self { coord, edge }
    }

    pub fn spans(&self) -> [Coordinate; 2] {
        [self.coord, self.coord + self.edge.to_hex2d()]
    }
}

/// Turn an unrestricted direction into the restricted direction on the coordinate
pub fn canonicalize(
    coord: Coordinate,
    dir: Direction,
) -> (Coordinate, HexEdge) {
    match dir {
        Direction::XY => (coord, HexEdge::XY),
        Direction::ZY => (coord, HexEdge::ZY),
        Direction::ZX => (coord, HexEdge::ZX),
        // These three we need to offset
        // Offset by the direction and flip
        Direction::YX => (coord + Direction::YX, HexEdge::XY),
        Direction::YZ => (coord + Direction::YZ, HexEdge::ZY),
        Direction::XZ => (coord + Direction::XZ, HexEdge::ZX),
    }
}
