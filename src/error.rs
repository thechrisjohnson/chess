//! Contains the error handling code for the chess library

use crate::Square;
use std::{error::Error, fmt};

/// Represents an error that can occur while processing chess moves
#[derive(Debug)]
pub enum ChessError {
    /// The color of the piece and the currernt player's turn do not match
    IncorrectColorForMove,
    /// A piece wasn't found in this location
    NoPieceFound(Square),
    /// You cannot capture your own piece
    CannotCaptureOwnPiece,
    /// The move put you into check
    CannotMoveIntoCheck,
    /// The mvoe did not get you out of check
    MustMoveOutOfCheck,
    /// Other types of invalid moves
    InvalidMove(String),
    /// An error attempting to parse the data happened
    ParseError(String),
    /// An attempt was made to promote a pawn not at the back line
    InvalidPawnPromotion,
    /// Represents an invalid attempt to Castle
    InvalidCastle(String),
}

impl Error for ChessError {}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessError::IncorrectColorForMove => {
                write!(f, "Incorrect color piece attempted to be moved")
            }
            ChessError::NoPieceFound(location) => write!(f, "No piece was found at {}", location),
            ChessError::CannotMoveIntoCheck => write!(f, "Move would lead player into check"),
            ChessError::MustMoveOutOfCheck => write!(f, "Move lead to out of check"),
            ChessError::CannotCaptureOwnPiece => write!(f, ""),
            ChessError::InvalidMove(error) => write!(f, "Invalid move: {}", error),
            ChessError::ParseError(error) => write!(f, "Error parsing move: {}", error),
            ChessError::InvalidPawnPromotion => write!(
                f,
                "Only able to promote pawns which are also on the far side"
            ),
            ChessError::InvalidCastle(error) => write!(f, "Unable to castle: {}", error),
        }
    }
}
