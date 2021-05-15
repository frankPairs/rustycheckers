#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl GamePiece {
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,
        }
    }
    pub fn crowned(p: GamePiece) -> GamePiece {
        GamePiece {
            color: p.color,
            crowned: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn on_board(self) -> bool {
        let Coordinate(x, y) = self;

        x <= 7 && y <= 7
    }

    /// Produce an iterator of Coordinates of all possible jumps from
    /// the given coordinate
    pub fn jump_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut jumps: Vec<Coordinate> = vec![];
        let Coordinate(x, y) = *self;

        if y >= 2 {
            jumps.push(Coordinate(x + 2, y - 2));
        }
        jumps.push(Coordinate(x + 2, y + 2));

        if x >= 2 && y >= 2 {
            jumps.push(Coordinate(x - 2, y - 2));
        }

        if x >= 2 {
            jumps.push(Coordinate(x - 2, y + 2));
        }

        jumps.into_iter()
    }

    /// Produce an iterator of Coordinates of all possible moves from
    /// the given coordinate
    pub fn move_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut moves: Vec<Coordinate> = vec![];
        let Coordinate(x, y) = *self;

        if x >= 1 {
            moves.push(Coordinate(x - 1, y + 1));
        }
        moves.push(Coordinate(x + 1, y + 1));

        if y >= 1 && x >= 1 {
            moves.push(Coordinate(x - 1, y - 1));
        }

        if y >= 1 {
            moves.push(Coordinate(x + 1, y - 1));
        }

        moves.into_iter()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Move {
        Move {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}
