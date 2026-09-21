//! A library for playing a chess game
#![deny(missing_docs)]

mod board;
pub mod error;

use crate::{board::ChessBoard, error::ChessError};
use std::fmt;

/// The number of moves that can occur before a draw is declated (without a capture or a pawn move)
pub const DRAW_MOVE_LIMIT: usize = 50;

/// Represents a game of chess
pub struct ChessGame {
    board: ChessBoard,
    current_turn: Color,
    moves_without_capture: usize,
    previous_moves: Vec<Move>,
}

impl ChessGame {
    /// Create a new chess board with the default layout
    pub fn new() -> Self {
        Self {
            board: ChessBoard::new(),
            current_turn: Color::White,
            moves_without_capture: 0,
            previous_moves: Vec::new(),
        }
    }

    /// Make a move in the game
    pub fn make_move(&mut self, chess_move: Move) -> Result<Option<GameResult>, ChessError> {
        match chess_move {
            Move::Move(square, square1) => todo!(),
            Move::Castle(castle_side) => todo!(),
            Move::Resign => todo!(),
            Move::Draw => todo!(),
        }

        Ok(None)
    }
}

/// Represents the possible moves on a board
#[derive(PartialEq)]
pub enum Move {
    /// Move from first to second square
    Move(Square, Square),
    /// A move to castle the king on one side
    Castle(CastleSide),
    /// Resign the game
    Resign,
    /// Offer or accept a draw on the game
    Draw,
}

/// Represents which side you wish to castle on
#[derive(PartialEq)]
pub enum CastleSide {
    /// Castle Queen side (0-0-0)
    Queen,
    /// Castle King side (0-0)
    King,
}

impl fmt::Display for CastleSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastleSide::Queen => write!(f, "Queenside"),
            CastleSide::King => write!(f, "Kingside"),
        }
    }
}

/// Represents the possible outcomes of the game
pub enum GameResult {
    /// Black wins
    Win(Color),
    /// A draw occurs
    Draw(DrawReason),
}

/// Represents the reasons that can cause a draw
pub enum DrawReason {
    /// The players agreed to a draw
    Players,
    /// The same position ocurred three times
    ThreefoldRepetition,
    /// Fifty moves occured without a capture or a pawn movement
    FiftyMoves,
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

/// Represents the different color pieces on a chess board
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
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

    fn get_next_turn(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn starting_pawn_rank(&self) -> Rank {
        match self {
            Color::White => Rank::Two,
            Color::Black => Rank::Seven,
        }
    }

    fn pawn_promotion_rank(&self) -> Rank {
        match self {
            Color::White => Rank::Eight,
            Color::Black => Rank::One,
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
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
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

impl PieceType {
    fn can_check_straight(&self, distance: isize) -> bool {
        match self {
            PieceType::King if distance == 1 => true,
            PieceType::Rook | PieceType::Queen => true,
            _ => false,
        }
    }

    fn can_check_diagonal(&self, distance: isize) -> bool {
        match self {
            PieceType::Pawn if distance == 1 => true,
            PieceType::Bishop | PieceType::Queen => true,
            _ => false,
        }
    }

    fn can_check_knight(&self) -> bool {
        match self {
            PieceType::Knight => true,
            _ => false,
        }
    }

    fn can_be_promoted(&self) -> bool {
        match self {
            PieceType::Pawn => true,
            _ => false,
        }
    }
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
            _ => Err(ChessError::ParseError(format!(
                "Invalid File value {}",
                value
            ))),
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
            _ => Err(ChessError::ParseError(format!(
                "Invalid Rank value {}",
                value
            ))),
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
