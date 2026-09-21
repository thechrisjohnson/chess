//! A library for playing a chess game
#![deny(missing_docs)]

use std::{collections::HashMap, error::Error, fmt, ops::Index, rc::Rc};

const BOARD: [(Square, Piece); 32] = [
    (Square{file: File::A, rank: Rank::One}, Piece{color: Color::White, piece_type: PieceType::Rook, has_moved: false}),
    (Square{file: File::B, rank: Rank::One}, Piece{color: Color::White, piece_type: PieceType::Knight, has_moved: false}),
    (Square{file: File::C, rank: Rank::One}, Piece{color: Color::White, piece_type: PieceType::Bishop, has_moved: false}),
    (Square{file: File::D, rank: Rank::One}, Piece{color: Color::White, piece_type: PieceType::Queen, has_moved: false}),
    (Square{file: File::E, rank: Rank::One}, Piece{color: Color::White, piece_type: PieceType::King, has_moved: false}),
    (Square{file: File::F, rank: Rank::One}, Piece{color: Color::White, piece_type: PieceType::Bishop, has_moved: false}),
    (Square{file: File::G, rank: Rank::One}, Piece{color: Color::White, piece_type: PieceType::Knight, has_moved: false}),
    (Square{file: File::H, rank: Rank::One}, Piece{color: Color::White, piece_type: PieceType::Rook, has_moved: false}),
    (Square{file: File::A, rank: Rank::Two}, Piece{color: Color::White, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::B, rank: Rank::Two}, Piece{color: Color::White, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::C, rank: Rank::Two}, Piece{color: Color::White, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::D, rank: Rank::Two}, Piece{color: Color::White, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::E, rank: Rank::Two}, Piece{color: Color::White, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::F, rank: Rank::Two}, Piece{color: Color::White, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::G, rank: Rank::Two}, Piece{color: Color::White, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::H, rank: Rank::Two}, Piece{color: Color::White, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::A, rank: Rank::Eight}, Piece{color: Color::Black, piece_type: PieceType::Rook, has_moved: false}),
    (Square{file: File::B, rank: Rank::Eight}, Piece{color: Color::Black, piece_type: PieceType::Knight, has_moved: false}),
    (Square{file: File::C, rank: Rank::Eight}, Piece{color: Color::Black, piece_type: PieceType::Bishop, has_moved: false}),
    (Square{file: File::D, rank: Rank::Eight}, Piece{color: Color::Black, piece_type: PieceType::Queen, has_moved: false}),
    (Square{file: File::E, rank: Rank::Eight}, Piece{color: Color::Black, piece_type: PieceType::King, has_moved: false}),
    (Square{file: File::F, rank: Rank::Eight}, Piece{color: Color::Black, piece_type: PieceType::Bishop, has_moved: false}),
    (Square{file: File::G, rank: Rank::Eight}, Piece{color: Color::Black, piece_type: PieceType::Knight, has_moved: false}),
    (Square{file: File::H, rank: Rank::Eight}, Piece{color: Color::Black, piece_type: PieceType::Rook, has_moved: false}),
    (Square{file: File::A, rank: Rank::Seven}, Piece{color: Color::Black, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::B, rank: Rank::Seven}, Piece{color: Color::Black, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::C, rank: Rank::Seven}, Piece{color: Color::Black, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::D, rank: Rank::Seven}, Piece{color: Color::Black, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::E, rank: Rank::Seven}, Piece{color: Color::Black, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::F, rank: Rank::Seven}, Piece{color: Color::Black, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::G, rank: Rank::Seven}, Piece{color: Color::Black, piece_type: PieceType::Pawn, has_moved: false}),
    (Square{file: File::H, rank: Rank::Seven}, Piece{color: Color::Black, piece_type: PieceType::Pawn, has_moved: false}),
];

/// Represents a game of chess
pub struct ChessGame {
    board: HashMap<Square, Rc<Piece>>,
    current_turn: Color,
    white_pieces: Vec<Rc<Piece>>,
    black_pieces: Vec<Rc<Piece>>,
}

impl ChessGame {
    /// Create a new chess board with the default layout
    pub fn new() -> Self {
        let mut board = HashMap::new();
        let mut white_pieces = Vec::new();
        let mut black_pieces = Vec::new();

        for (square, piece) in BOARD {
            let rc = Rc::new(piece);

            board.insert(square, rc.clone());

            match &rc.color {
                Color::White => white_pieces.push(rc),
                Color::Black => black_pieces.push(rc),
            }
        }

        Self {
            board,
            current_turn: Color::White,
            white_pieces,
            black_pieces,
        }
    }

    /// Allow a move to be made in the game
    pub fn make_move(&mut self, chess_move: Move) -> Result<(), ChessError> {
        // Make sure the right person took their turn
        if chess_move.player != self.current_turn {
            return Err(ChessError::new(format!("It is currently not {}'s turn", chess_move.player)));
        }

        match chess_move.move_type {
            MoveType::Move(source, destination) => {
                // Validate a piece is at the source and any piece at the destination isn't of the same color
                let piece = self.get_piece_for_color_or_fail(&source, chess_move.player)?;
                let destination_piece = self.board.get(&destination);
                if destination_piece.is_some_and(|p| p.color == piece.color) {
                    return Err(ChessError::new(format!("Piece at destiniation {} is also {}", destination, piece.color)));
                }

                // You can't capture a King
                if destination_piece.is_some_and(|p| p.piece_type == PieceType::King) {
                    return Err(ChessError::new(format!("Piece at destiniation {} is {}", destination, piece.piece_type)));
                }

                // Validate we have a path to the destination and that there's nothing blocking us
                for square in  piece.get_path_to(&source, &destination, destination_piece.is_some())? {
                    if square != destination && let Some(blocker) = self.board.get(&square) {
                        return Err(ChessError::new(format!("Path to {} blocked by {} at {}", destination, blocker, square)));
                    }
                }

                // TODO: Validate move doesn't force self-check
                // If destination piece, remove it from the game
                // TODO: Remove the piece from the board
                if let Some(target) = self.board.remove(&destination) {
                    match target.color {
                        Color::White => self.white_pieces.remove(self.white_pieces.),
                        Color::Black => todo!(),
                    }
                }
            },
            MoveType::Castle(castle_side) => {

            },
            MoveType::Resign => {

            },
        };

        Ok(())
    }

    fn get_piece_for_color_or_fail(&self, location: &Square, color: Color) -> Result<&Rc<Piece>, ChessError> {
        match self.board.get(location) {
            Some(piece) => {
                if piece.color != color {
                    Err(ChessError::new(format!("Piece at {} is not {}", location, color))) 
                } else {
                    Ok(piece)
                }
            },
            None => Err(ChessError::new(format!("No piece found at {}", location))),
        }
    }
}

/// Represents a move
pub struct Move {
    player: Color,
    move_type: MoveType
}

impl Move {
    /// Create a new move for player of Color and with the MoveType
    pub fn new(player: Color, move_type: MoveType) -> Self { 
        Self { player, move_type}
    }
}

/// Represents the possible moves on a board
pub enum MoveType {
    /// Move from first to second square
    Move(Square, Square),
    /// A move to castle the king on one side
    Castle(CastleSide),
    /// Resign the game
    Resign,
}

/// Represents which side you wish to castle on
pub enum CastleSide {
    /// Castle Queen side (0-0-0)
    Queen,
    /// Castle King side (0-0)
    King
}

impl CastleSide {
    fn get_king_position(&self, color: Color) -> Square {
        match color {
            Color::White => Square{file: File::E, rank: Rank::One},
            Color::Black => Square{file: File::E, rank: Rank::Eight},
        }
    }
    
    fn get_knight_position(&self, color: Color) -> Square {
        match self {
            CastleSide::Queen => {
                match color {
                    Color::White => Square{file: File::A, rank: Rank::One},
                    Color::Black => Square{file: File::A, rank: Rank::Eight},
                }
            },
            CastleSide::King => {
                match color {
                    Color::White => Square{file: File::H, rank: Rank::One},
                    Color::Black => Square{file: File::H, rank: Rank::Eight},
                }
            },
        }
    }
}

/// Represents a square on the board
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Square {
    file: File,
    rank: Rank,
}

impl Square {
    /// Create a new square on file and rank
    pub fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

/// Represents a piece on the chess board
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    has_moved: bool,
}

impl Piece {
    /// Create a new chess piece
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self {
            color,
            piece_type,
            has_moved: false,
        }
    }

    fn valid_destinations(&self, location: &Square) -> Vec<Square> {
        let mut result = Vec::new();

        match self.piece_type {
            PieceType::King => {
                append_diagonals(&mut result, location, Some(1));
                append_verticals(&mut result, location, Some(1));
            },
            PieceType::Queen => {
                append_diagonals(&mut result, location, None);
                append_verticals(&mut result, location, None);
            },
            PieceType::Bishop => {
                append_diagonals(&mut result, location, None);
            }
            PieceType::Knight => {
                append_knight_moves(&mut result, location);
            }
            PieceType::Rook => {
                append_verticals(&mut result, location, None);
            },
            PieceType::Pawn => {
                match self.color {
                    Color::White => {
                        if let Some(new_square) = try_square_add(location, 1, 0) {
                            result.push(new_square);
                        }
                    },
                    Color::Black => {
                        if let Some(new_square) = try_square_add(location, -1, 0) {
                            result.push(new_square);
                        }
                    },
                }
            }
        };

        result
    }

    fn get_path_to(&self, source: &Square, destination: &Square, piece_at_destination: bool) -> Result<Vec<Square>, ChessError> {
        let delta_file = destination.file.to_isize() - source.file.to_isize();
        let delta_rank = destination.rank.to_isize() - source.rank.to_isize();
        let mut path = Vec::new();
        match self.piece_type {
            PieceType::King => {
                if delta_file.abs() <= 1  && delta_rank.abs() <= 1 {
                    path.push(destination.clone());
                }
            },
            PieceType::Queen => {
                if delta_file == 0 {
                    append_square_line(&mut path, source, 0, get_delta_direction(delta_rank), Some(delta_rank.abs() as usize));
                } else if delta_rank == 0  {
                    append_square_line(&mut path, source, get_delta_direction(delta_file), 0, Some(delta_file.abs() as usize));
                } else if delta_file.abs() == delta_rank.abs() {
                    append_square_line(&mut path, source, get_delta_direction(delta_file), get_delta_direction(delta_rank), Some(delta_file.abs() as usize));
                }
            },
            PieceType::Bishop => {
                if delta_file.abs() == delta_rank.abs() {
                    append_square_line(&mut path, source, get_delta_direction(delta_file), get_delta_direction(delta_rank), Some(delta_file.abs() as usize));
                }
            },
            PieceType::Knight => {
                if (delta_file.abs() == 2 && delta_rank.abs() == 1) || (delta_rank.abs() == 2 && delta_file.abs() == 1) {
                    path.push(destination.clone());
                }
            },
            PieceType::Rook => {
                if delta_file == 0 {
                    append_square_line(&mut path, source, 0, get_delta_direction(delta_rank), Some(delta_rank.abs() as usize));
                } else if delta_rank == 0 {
                    append_square_line(&mut path, source, get_delta_direction(delta_file), 0, Some(delta_file.abs() as usize));
                }
            },
            PieceType::Pawn => {
                if delta_file == 0 {
                    if self.color.get_pawn_direction() == delta_rank {
                        path.push(destination.clone())
                    } else if !self.has_moved && (self.color.get_pawn_direction() * 2) == delta_rank {
                        path.push(destination.clone())
                    }
                } else if delta_file.abs() == 1 {
                    if piece_at_destination && self.color.get_pawn_direction() == delta_rank {
                        path.push(destination.clone())
                    }
                }
                // Todo en passant
            },
        }

        if path.len() > 0 {
        Ok(path)
        } else {
            Err(ChessError::new(format!("The {} on {} cannot move to {}", self, source, destination)))
        }
    }

}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color, self.piece_type)
    }
}

/// Represents the different color pieces on a chess board
#[derive(PartialEq, Debug)]
pub enum Color {
    /// White
    White = 1,
    /// Black
    Black = -1,
}

impl Color {
    fn get_pawn_direction(&self) -> isize {
        match self {
            Color::White => Color::White as isize,
            Color::Black => Color::Black as isize,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

/// Represents the different type of pieces that can exist on a chess board
#[derive(PartialEq, Debug)]
pub enum PieceType {
    /// The King
    King,
    /// The Queen
    Queen,
    /// The Bishops
    Bishop,
    /// The Knights
    Knight,
    /// The Rooks
    Rook,
    /// The Pawns
    Pawn,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::King => write!(f, "King"),
            PieceType::Queen => write!(f, "Queen"),
            PieceType::Bishop => write!(f, "Bishop"),
            PieceType::Knight => write!(f, "Knight"),
            PieceType::Rook => write!(f, "Rook"),
            PieceType::Pawn => write!(f, "Pawn"),
        }
    }
}

/// Represents the possible files of a chess board
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum File {
    /// A File
    A = 1,
    /// B File
    B = 2,
    /// C File
    C = 3,
    /// D File
    D = 4,
    /// E File
    E = 5,
    /// F File
    F = 6,
    /// G File
    G = 7,
    /// H File
    H = 8,
}

impl File {
    /// All of the files in order
    pub const FILES: [File; 8] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];

    /// Add a number to this file to get the matching value
    pub fn add(&self, rhs: isize) -> Result<Self, ChessError> {
        Self::from_isize(self.to_isize() + rhs)
    }

    /// Convert an isize into a File
    pub fn from_isize(value: isize) -> Result<Self, ChessError> {
        // TODO: Can I do this in a better way?
        match value {
            1 => Ok(File::A),
            2 => Ok(File::B),
            3 => Ok(File::C),
            4 => Ok(File::D),
            5 => Ok(File::E),
            6 => Ok(File::F),
            7 => Ok(File::G),
            8 => Ok(File::H),
            _ => Err(ChessError { message: format!("Invalid File value {}", value) })
        }
    }

    /// Convert the File to an isize value
    pub fn to_isize(&self) -> isize {
        match self {
            File::A => File::A as isize,
            File::B => File::B as isize,
            File::C => File::C as isize,
            File::D => File::D as isize,
            File::E => File::E as isize,
            File::F => File::F as isize,
            File::G => File::G as isize,
            File::H => File::H as isize,
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            File::A => write!(f, "A"),
            File::B => write!(f, "B"),
            File::C => write!(f, "C"),
            File::D => write!(f, "D"),
            File::E => write!(f, "E"),
            File::F => write!(f, "F"),
            File::G => write!(f, "G"),
            File::H => write!(f, "H"),
        }
    }
}

/// Represents the possible ranks of a chess board
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Rank {
    /// One File
    One = 1,
    /// Two File
    Two = 2,
    /// Three File
    Three = 3,
    /// Four File
    Four = 4,
    /// Five File
    Five = 5,
    /// Six File
    Six = 6,
    /// Sevent File
    Seven = 7,
    /// Eight File
    Eight = 8,
}

impl Rank {
    /// All of the ranks in order
    pub const RANKS: [Rank; 8] = [
        Rank::One,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
    ];

    /// Add a number to this file to get the matching value
    pub fn add(&self, rhs: isize) -> Result<Self, ChessError> {
        Self::from_isize(self.to_isize() + rhs)
    }

    /// Convert an isize into a File
    pub fn from_isize(value: isize) -> Result<Self, ChessError> {
        // TODO: Can I do this in a better way?
        match value {
            1 => Ok(Rank::One),
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            _ => Err(ChessError { message: format!("Invalid Rank value {}", value) })
        }
    }

    /// Convert the Rank to an isize value
    pub fn to_isize(&self) -> isize {
        match self {
            Rank::One => Rank::One as isize,
            Rank::Two => Rank::Two as isize,
            Rank::Three => Rank::Three as isize,
            Rank::Four => Rank::Four as isize,
            Rank::Five => Rank::Five as isize,
            Rank::Six => Rank::Six as isize,
            Rank::Seven => Rank::Seven as isize,
            Rank::Eight => Rank::Eight as isize,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_isize())
    }
}

/// Represents an error that can occur while processing chess moves
#[derive(Debug)]
pub struct ChessError {
    /// The error message
    pub message: String,
}

impl ChessError {
    /// Create a new ChessError
    pub fn new(message: String) -> Self {
        ChessError { message }
    }
}

impl Error for ChessError {
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn append_diagonals(vector: &mut Vec<Square>, location: &Square, max: Option<usize>) {
    // Top Left
    append_square_line(vector, location, 1, -1, max);
    // Top Right
    append_square_line(vector, location, 1, 1, max);
    // Bottom Left
    append_square_line(vector, location, -1, -1, max);
    // Bottom Right
    append_square_line(vector, location, -1, 1, max);
}

fn append_verticals(vector: &mut Vec<Square>, location: &Square, max: Option<usize>) {
    // Up
    append_square_line(vector, location, 1, 0, max);
    // Down
    append_square_line(vector, location, -1, 0, max);
    // Left
    append_square_line(vector, location, 0, -1, max);
    // Right
    append_square_line(vector, location, 0, 1, max);
}

fn append_knight_moves(vector: &mut Vec<Square>, location: &Square) {
    // Two up one left
    if let Some(new_square) = try_square_add(location, 2, -1) {
        vector.push(new_square);
    }
    // Two up one right
    if let Some(new_square) = try_square_add(location, 2, 1) {
        vector.push(new_square);
    }
    // Two left one up
    if let Some(new_square) = try_square_add(location, 1, -2) {
        vector.push(new_square);
    }
    // Two left one down
    if let Some(new_square) = try_square_add(location, -1, -2) {
        vector.push(new_square);
    }
    // Two right one up
    if let Some(new_square) = try_square_add(location, 1, 2) {
        vector.push(new_square);
    }
    // Two right on down
    if let Some(new_square) = try_square_add(location, -1, 2) {
        vector.push(new_square);
    }
    // Two down one left
    if let Some(new_square) = try_square_add(location, -2, -1) {
        vector.push(new_square);
    }
    // Two down one right
    if let Some(new_square) = try_square_add(location, -2, 1) {
        vector.push(new_square);
    }
}

fn append_square_line(vector: &mut Vec<Square>, location: &Square, file_delta: isize, rank_delta: isize, max: Option<usize>) {
    let max = max.unwrap_or(usize::MAX);
    // Top Left
    let mut current_location = location.clone();
    for _ in 0..max {
        // Add delta to file, see if that's ok
        if let Some(new_square) = try_square_add(&current_location, file_delta, rank_delta) {
            current_location = new_square.clone();
            vector.push(new_square);
        } else {
            break;
        }
    }
}

fn try_square_add(location: &Square, file_delta: isize, rank_delta: isize) -> Option<Square> {
    if let Ok(new_file) = location.file.add(file_delta) {
        if let Ok(new_rank) = location.rank.add(rank_delta) {
            return Some(Square { file: new_file, rank: new_rank });
        }
    }

    None
}

fn get_delta_direction(delta: isize) -> isize {
    if delta < 0 {
        -1
    } else if delta > 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_square_add_adds_file_and_delta() {
        let starting_square = Square::new(File::A, Rank::One);
        let add = try_square_add(&starting_square, 1, 1);
        assert!(add.is_some());
        let unwrapped = add.unwrap();
        assert_eq!(File::B, unwrapped.file);
        assert_eq!(Rank::Two, unwrapped.rank);
    }

    #[test]
    fn try_square_add_wont_go_too_far_left() {
        let starting_square = Square::new(File::B, Rank::Four);
        let add = try_square_add(&starting_square, -3, 0);
        assert!(add.is_none());
    }

    #[test]
    fn try_square_add_wont_go_too_far_right() {
        let starting_square = Square::new(File::G, Rank::Five);
        let add = try_square_add(&starting_square, 4, 0);
        assert!(add.is_none());
    }

    #[test]
    fn try_square_add_wont_go_too_far_down() {
        let starting_square = Square::new(File::D, Rank::Two);
        let add = try_square_add(&starting_square, 0, -2);
        assert!(add.is_none());
    }

    #[test]
    fn try_square_add_wont_go_too_far_up() {
        let starting_square = Square::new(File::D, Rank::Seven);
        let add = try_square_add(&starting_square, 0, 5);
        assert!(add.is_none());
    }

    #[test]
    fn append_square_line_handles_max() {
        let mut vector = Vec::new();
        let starting_square = Square::new(File::A, Rank::One);
        append_square_line(&mut vector, &starting_square, 1, 1, Some(2));

        assert_eq!(2, vector.len());
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Two));
        assert!(vector.iter().any(|v| v.file == File::C && v.rank == Rank::Three));
    }

    #[test]
    fn append_square_line_handles_does_diagonal_up() {
        let mut vector = Vec::new();
        let starting_square = Square::new(File::A, Rank::One);
        append_square_line(&mut vector, &starting_square, 1, 1, None);
        assert_eq!(7, vector.len());
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Two));
        assert!(vector.iter().any(|v| v.file == File::C && v.rank == Rank::Three));
        assert!(vector.iter().any(|v| v.file == File::D && v.rank == Rank::Four));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::F && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::G && v.rank == Rank::Seven));
        assert!(vector.iter().any(|v| v.file == File::H && v.rank == Rank::Eight));
    }

    #[test]
    fn append_square_line_handles_does_diagonal_down() {
        let mut vector = Vec::new();
        let starting_square = Square::new(File::A, Rank::Eight);
        append_square_line(&mut vector, &starting_square, 1, -1, None);

        assert_eq!(7, vector.len());
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Seven));
        assert!(vector.iter().any(|v| v.file == File::C && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::D && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Four));
        assert!(vector.iter().any(|v| v.file == File::F && v.rank == Rank::Three));
        assert!(vector.iter().any(|v| v.file == File::G && v.rank == Rank::Two));
        assert!(vector.iter().any(|v| v.file == File::H && v.rank == Rank::One));
    }

    #[test]
    fn append_square_line_handles_does_vertical_up() {
        let mut vector = Vec::new();
        let starting_square = Square::new(File::E, Rank::One);
        append_square_line(&mut vector, &starting_square, 0, 1, None);

        assert_eq!(7, vector.len());
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Two));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Three));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Four));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Seven));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Eight));
    }

    #[test]
    fn append_square_line_handles_does_vertical_down() {
        let mut vector = Vec::new();
        let starting_square = Square::new(File::B, Rank::Eight);
        append_square_line(&mut vector, &starting_square, 0, -1, None);

        assert_eq!(7, vector.len());
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Seven));
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Four));
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Three));
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Two));
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::One));
    }

    #[test]
    fn append_square_line_handles_does_horizontal_right() {
        let mut vector = Vec::new();
        let starting_square = Square::new(File::A, Rank::Six);
        append_square_line(&mut vector, &starting_square, 1, 0, None);

        assert_eq!(7, vector.len());
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::C && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::D && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::F && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::G && v.rank == Rank::Six));
        assert!(vector.iter().any(|v| v.file == File::H && v.rank == Rank::Six));
    }

    #[test]
    fn append_square_line_handles_does_horizontal_left() {
        let mut vector = Vec::new();
        let starting_square = Square::new(File::H, Rank::Five);
        append_square_line(&mut vector, &starting_square, -1, 0, None);

        assert_eq!(7, vector.len());
        assert!(vector.iter().any(|v| v.file == File::G && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::F && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::E && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::D && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::C && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::B && v.rank == Rank::Five));
        assert!(vector.iter().any(|v| v.file == File::A && v.rank == Rank::Five));
    }

    // The King
    #[test]
    fn get_path_to_king_success_rank() {
        let source = Square::new(File::D, Rank::Four);
        let destination = Square::new(File::D, Rank::Three);
        let piece = Piece::new(Color::Black, PieceType::King);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::D && s.rank == Rank::Three));
    }

    #[test]
    fn get_path_to_king_success_file() {
        let source = Square::new(File::D, Rank::Four);
        let destination = Square::new(File::E, Rank::Four);
        let piece = Piece::new(Color::Black, PieceType::King);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::E && s.rank == Rank::Four));
    }

    #[test]
    fn get_path_to_king_success_diagonal() {
        let source = Square::new(File::D, Rank::Four);
        let destination = Square::new(File::E, Rank::Three);
        let piece = Piece::new(Color::Black, PieceType::King);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::E && s.rank == Rank::Three));
    }

    #[test]
    fn get_path_to_king_failure() {
        let source = Square::new(File::D, Rank::Four);
        let destination = Square::new(File::F, Rank::Three);
        let piece = Piece::new(Color::Black, PieceType::King);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    // The Queen
    #[test]
    fn get_path_to_queen_success_rank() {
        let source = Square::new(File::A, Rank::Two);
        let destination = Square::new(File::A, Rank::Seven);
        let piece = Piece::new(Color::Black, PieceType::Queen);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(5, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::A && s.rank == Rank::Three));
        assert!(unwrapped.iter().any(|s| s.file == File::A && s.rank == Rank::Four));
        assert!(unwrapped.iter().any(|s| s.file == File::A && s.rank == Rank::Five));
        assert!(unwrapped.iter().any(|s| s.file == File::A && s.rank == Rank::Six));
        assert!(unwrapped.iter().any(|s| s.file == File::A && s.rank == Rank::Seven));
    }

    #[test]
    fn get_path_to_queen_success_file() {
        let source = Square::new(File::C, Rank::Five);
        let destination = Square::new(File::F, Rank::Five);
        let piece = Piece::new(Color::Black, PieceType::Queen);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(3, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::D && s.rank == Rank::Five));
        assert!(unwrapped.iter().any(|s| s.file == File::E && s.rank == Rank::Five));
        assert!(unwrapped.iter().any(|s| s.file == File::F && s.rank == Rank::Five));
    }

    #[test]
    fn get_path_to_queen_success_diagonal() {
        let source = Square::new(File::G, Rank::Seven);
        let destination = Square::new(File::B, Rank::Two);
        let piece = Piece::new(Color::Black, PieceType::Queen);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(5, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::F && s.rank == Rank::Six));
        assert!(unwrapped.iter().any(|s| s.file == File::E && s.rank == Rank::Five));
        assert!(unwrapped.iter().any(|s| s.file == File::D && s.rank == Rank::Four));
        assert!(unwrapped.iter().any(|s| s.file == File::C && s.rank == Rank::Three));
        assert!(unwrapped.iter().any(|s| s.file == File::B && s.rank == Rank::Two));
    }

    #[test]
    fn get_path_to_queen_failure() {
        let source = Square::new(File::G, Rank::Seven);
        let destination = Square::new(File::E, Rank::Six);
        let piece = Piece::new(Color::Black, PieceType::Queen);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    // The Bishops
    #[test]
    fn get_path_to_bishop_success() {
        let source = Square::new(File::F, Rank::Seven);
        let destination = Square::new(File::B, Rank::Three);
        let piece = Piece::new(Color::Black, PieceType::Bishop);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(4, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::E && s.rank == Rank::Six));
        assert!(unwrapped.iter().any(|s| s.file == File::D && s.rank == Rank::Five));
        assert!(unwrapped.iter().any(|s| s.file == File::C && s.rank == Rank::Four));
        assert!(unwrapped.iter().any(|s| s.file == File::B && s.rank == Rank::Three));
    }

    #[test]
    fn get_path_to_bishop_failure() {
        let source = Square::new(File::G, Rank::Seven);
        let destination = Square::new(File::G, Rank::One);
        let piece = Piece::new(Color::Black, PieceType::Bishop);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    // The Knights
    #[test]
    fn get_path_to_knight_success() {
        let source = Square::new(File::C, Rank::Five);
        let destination = Square::new(File::B, Rank::Seven);
        let piece = Piece::new(Color::Black, PieceType::Knight);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::B && s.rank == Rank::Seven));
    }

    #[test]
    fn get_path_to_knight_failure() {
        let source = Square::new(File::F, Rank::Three);
        let destination = Square::new(File::D, Rank::Five);
        let piece = Piece::new(Color::Black, PieceType::Knight);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    // The Rooks
    #[test]
    fn get_path_to_rook_success_rank() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::C, Rank::Seven);
        let piece = Piece::new(Color::Black, PieceType::Rook);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(4, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::C && s.rank == Rank::Four));
        assert!(unwrapped.iter().any(|s| s.file == File::C && s.rank == Rank::Five));
        assert!(unwrapped.iter().any(|s| s.file == File::C && s.rank == Rank::Six));
        assert!(unwrapped.iter().any(|s| s.file == File::C && s.rank == Rank::Seven));
    }

    #[test]
    fn get_path_to_rook_success_file() {
        let source = Square::new(File::B, Rank::Six);
        let destination = Square::new(File::E, Rank::Six);
        let piece = Piece::new(Color::Black, PieceType::Rook);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(3, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::C && s.rank == Rank::Six));
        assert!(unwrapped.iter().any(|s| s.file == File::D && s.rank == Rank::Six));
        assert!(unwrapped.iter().any(|s| s.file == File::E && s.rank == Rank::Six));
    }

    #[test]
    fn get_path_to_rook_failure() {
        let source = Square::new(File::A, Rank::One);
        let destination = Square::new(File::G, Rank::Eight);
        let piece = Piece::new(Color::Black, PieceType::Rook);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    // The Pawns
    #[test]
    fn get_path_to_pawn_black_success_one() {
        let source = Square::new(File::B, Rank::Six);
        let destination = Square::new(File::B, Rank::Five);
        let mut piece = Piece::new(Color::Black, PieceType::Pawn);
        piece.has_moved = true;
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::B && s.rank == Rank::Five));
    }

    #[test]
    fn get_path_to_pawn_black_success_two() {
        let source = Square::new(File::F, Rank::Seven);
        let destination = Square::new(File::F, Rank::Five);
        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::F && s.rank == Rank::Five));
    }

    #[test]
    fn get_path_to_pawn_black_success_one_capture() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::B, Rank::Two);
        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, true);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::B && s.rank == Rank::Two));
    }

    #[test]
    fn get_path_to_pawn_black_failure_wrong_direction() {
        let source = Square::new(File::B, Rank::Six);
        let destination = Square::new(File::B, Rank::Seven);
        let mut piece = Piece::new(Color::Black, PieceType::Pawn);
        piece.has_moved = true;
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    #[test]
    fn get_path_to_pawn_black_failure_two_after_move() {
        let source = Square::new(File::F, Rank::Seven);
        let destination = Square::new(File::F, Rank::Five);
        let mut piece = Piece::new(Color::Black, PieceType::Pawn);
        piece.has_moved = true;
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    #[test]
    fn get_path_to_pawn_black_failure_capture_no_target() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::B, Rank::Two);
        let mut piece = Piece::new(Color::Black, PieceType::Pawn);
        piece.has_moved = true;
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    #[test]
    fn get_path_to_pawn_white_success_one() {
        let source = Square::new(File::A, Rank::Three);
        let destination = Square::new(File::A, Rank::Four);
        let mut piece = Piece::new(Color::White, PieceType::Pawn);
        piece.has_moved = true;
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::A && s.rank == Rank::Four));
    }

    #[test]
    fn get_path_to_pawn_white_success_two() {
        let source = Square::new(File::G, Rank::Six);
        let destination = Square::new(File::G, Rank::Eight);
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::G && s.rank == Rank::Eight));
    }

    #[test]
    fn get_path_to_pawn_white_success_one_capture() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::D, Rank::Four);
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, true);

        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(unwrapped.iter().any(|s| s.file == File::D && s.rank == Rank::Four));
    }

    #[test]
    fn get_path_to_pawn_white_failure_wrong_direction() {
        let source = Square::new(File::B, Rank::Six);
        let destination = Square::new(File::B, Rank::Five);
        let mut piece = Piece::new(Color::White, PieceType::Pawn);
        piece.has_moved = true;
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    #[test]
    fn get_path_to_pawn_white_failure_two_after_move() {
        let source = Square::new(File::F, Rank::Three);
        let destination = Square::new(File::F, Rank::Five);
        let mut piece = Piece::new(Color::White, PieceType::Pawn);
        piece.has_moved = true;
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }

    #[test]
    fn get_path_to_pawn_white_failure_capture_no_target() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::B, Rank::Four);
        let mut piece = Piece::new(Color::White, PieceType::Pawn);
        piece.has_moved = true;
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_err());
    }
}
