use std::{collections::HashMap, fmt};

use crate::{CastleSide, Color, DrawReason, File, GameResult, Move, PieceType, Rank, Square, error::ChessError};

/// The number of moves that can occur before a draw is declated (without a capture or a pawn move)
pub const DRAW_MOVE_LIMIT: usize = 50;

/// The layout for a default chess board
pub const DEFAULT_BOARD: [(Square, Piece); 32] = [
    (
        Square {
            file: File::A,
            rank: Rank::One,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
        },
    ),
    (
        Square {
            file: File::B,
            rank: Rank::One,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
        },
    ),
    (
        Square {
            file: File::C,
            rank: Rank::One,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
        },
    ),
    (
        Square {
            file: File::D,
            rank: Rank::One,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Queen,
        },
    ),
    (
        Square {
            file: File::E,
            rank: Rank::One,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::King,
        },
    ),
    (
        Square {
            file: File::F,
            rank: Rank::One,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
        },
    ),
    (
        Square {
            file: File::G,
            rank: Rank::One,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
        },
    ),
    (
        Square {
            file: File::H,
            rank: Rank::One,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
        },
    ),
    (
        Square {
            file: File::A,
            rank: Rank::Two,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::B,
            rank: Rank::Two,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::C,
            rank: Rank::Two,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::D,
            rank: Rank::Two,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::E,
            rank: Rank::Two,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::F,
            rank: Rank::Two,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::G,
            rank: Rank::Two,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::H,
            rank: Rank::Two,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::A,
            rank: Rank::Eight,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
        },
    ),
    (
        Square {
            file: File::B,
            rank: Rank::Eight,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
        },
    ),
    (
        Square {
            file: File::C,
            rank: Rank::Eight,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
        },
    ),
    (
        Square {
            file: File::D,
            rank: Rank::Eight,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Queen,
        },
    ),
    (
        Square {
            file: File::E,
            rank: Rank::Eight,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::King,
        },
    ),
    (
        Square {
            file: File::F,
            rank: Rank::Eight,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
        },
    ),
    (
        Square {
            file: File::G,
            rank: Rank::Eight,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
        },
    ),
    (
        Square {
            file: File::H,
            rank: Rank::Eight,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
        },
    ),
    (
        Square {
            file: File::A,
            rank: Rank::Seven,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::B,
            rank: Rank::Seven,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::C,
            rank: Rank::Seven,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::D,
            rank: Rank::Seven,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::E,
            rank: Rank::Seven,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::F,
            rank: Rank::Seven,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::G,
            rank: Rank::Seven,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
    (
        Square {
            file: File::H,
            rank: Rank::Seven,
        },
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        },
    ),
];

pub(crate) struct ChessBoard {
    board: InternalBoardState,
    pending_move: Option<ChessMoveHistory>,
    current_turn: Color,
    draw_moves: usize,
}

impl ChessBoard {
    pub(crate) fn new() -> Self {
        Self {
            board: InternalBoardState::new(),
            pending_move: None,
            current_turn: Color::White,
            draw_moves: 0,
        }
    }

    fn append_move<T: ChessMoveTrait>(&mut self, chess_move: T) -> Result<(), ChessError> {
        if self.has_pending_move() {
            return Err(ChessError::InvalidMove(format!(
                "Pending move. Commit first"
            )));
        }

        // Validate that this move is possible
        chess_move.validate(&self.board)?;

        // See if the current player is in check
        let pre_in_check = self.board.is_in_check(&self.current_turn);

        // Now get the move for this and apply it
        let history = chess_move.get_chess_move(&self.board);
        history.apply(&mut self.board);

        // Mark this move as pending
        self.pending_move = Some(history);

        // Check to see if apply this move lead to that player being in check
        let post_in_check = self.board.is_in_check(&self.current_turn);
        if post_in_check {
            self.abort_moves();
            if pre_in_check {
                return Err(ChessError::MustMoveOutOfCheck);
            } else {
                return Err(ChessError::CannotMoveIntoCheck);
            }
        }

        Ok(())
    }

    fn commit_move(&mut self) -> Option<GameResult> {
        if let Some(pending) = &self.pending_move {
            self.board.move_history.push(pending.clone());
            self.current_turn = self.current_turn.get_next_turn();
            

            // If we didn't move a pawn or capture, increment the draw counter
            if pending.captured.is_some() || pending.pieces_moved.iter().any(|(piece, _, _)| piece.piece_type == PieceType::Pawn) {
                self.draw_moves = 0;
            } else {
                self.draw_moves += 1;

            }
        }

        // TODO: Check for checkmate
        // Check for draw
        let result = if self.draw_moves >= DRAW_MOVE_LIMIT {
            Some(GameResult::Draw(DrawReason::FiftyMoves))
        } else {
            None
        };

        self.pending_move = None;

        result
    }

    fn abort_moves(&mut self) {
        if let Some(pending) = &self.pending_move {
            pending.rollback(&mut self.board);
        }

        self.pending_move = None;
    }

    fn has_pending_move(&self) -> bool {
        self.pending_move.is_some()
    }
}

struct InternalBoardState {
    board: HashMap<Square, Piece>,
    move_history: Vec<ChessMoveHistory>,
}

impl InternalBoardState {
    fn new() -> Self {
        let mut board = HashMap::new();

        for (square, piece) in DEFAULT_BOARD {
            board.insert(square, piece);
        }

        Self {
            board,
            move_history: Vec::new(),
        }
    }

    fn get_piece(&self, square: &Square) -> Option<Piece> {
        if let Some(piece) = self.board.get(square) {
            Some(piece.clone())
        } else {
            None
        }
    }

    fn get_piece_or_error(&self, square: &Square) -> Result<Piece, ChessError> {
        if let Some(piece) = self.board.get(square) {
            Ok(piece.clone())
        } else {
            Err(ChessError::NoPieceFound(square.clone()))
        }
    }

    fn is_in_check(&self, color: &Color) -> bool {
        let king_square = self
            .find_pieces(color, None)
            .pop()
            .expect("Invalid game state (no king?)")
            .0;
        self.square_in_check(&color.get_next_turn(), &king_square)
    }

    fn find_pieces(&self, color: &Color, piece_type: Option<PieceType>) -> Vec<(Square, Piece)> {
        let mut result = Vec::new();
        for (square, board_piece) in &self.board {
            if &board_piece.color == color {
                if let Some(p_type) = &piece_type {
                    if &board_piece.piece_type == p_type {
                        result.push((square.clone(), board_piece.clone()));
                    }
                } else {
                    result.push((square.clone(), board_piece.clone()));
                }
            }
        }

        result
    }

    fn square_in_check(&self, color: &Color, square: &Square) -> bool {
        // Get a list of all possible squares we could be checked from
        let mut paths = Vec::new();
        append_verticals(&mut paths, square, None);
        for subpath in &paths {
            let mut distance = 1;
            for square in subpath {
                if let Some(piece) = self.board.get(square) {
                    if &piece.color != color {
                        if piece.piece_type.can_check_straight(distance) {
                            return true;
                        }
                    }

                    break;
                }
                distance += 1;
            }
        }
        paths.clear();

        append_diagonals(&mut paths, &square, None);
        for subpath in &paths {
            let mut distance = 1;
            for square in subpath {
                if let Some(piece) = self.board.get(square) {
                    if &piece.color != color {
                        if piece.piece_type.can_check_diagonal(distance) {
                            return true;
                        }
                    }

                    break;
                }
                distance += 1;
            }
        }
        paths.clear();

        append_knight_moves(&mut paths, &square);
        for subpath in &paths {
            for square in subpath {
                if let Some(piece) = self.board.get(square) {
                    if &piece.color != color {
                        if piece.piece_type.can_check_knight() {
                            return true;
                        }
                    }
                }
            }
        }
        paths.clear();

        false
    }
}

trait ChessMoveTrait {
    fn get_chess_move(self, board: &InternalBoardState) -> ChessMoveHistory;
    fn validate(&self, board: &InternalBoardState) -> Result<(), ChessError>;
}

struct MovePiece {
    color: Color,
    source: Square,
    destination: Square,
    promotion: Option<PieceType>,
}

impl ChessMoveTrait for MovePiece {
    fn get_chess_move(self, board: &InternalBoardState) -> ChessMoveHistory {
        let captured = board
            .get_piece(&self.destination)
            .map(|f| (self.destination.clone(), f));
        let pieces_moved = vec![(
            board
                .get_piece(&self.source)
                .expect("We verified this already"),
            self.source,
            self.destination,
        )];

        ChessMoveHistory {
            pieces_moved,
            captured,
            promoted: self.promotion,
        }
    }

    fn validate(&self, board: &InternalBoardState) -> Result<(), ChessError> {
        // Make sure a piece exists at the source and validate it
        let piece = board.get_piece_or_error(&self.source)?;

        if &piece.color != &self.color {
            return Err(ChessError::IncorrectColorForMove);
        }

        // Make sure the piece can move there
        let target = board.get_piece(&self.destination);
        if let Some(path) = piece.get_path_to(&self.source, &self.destination, target.is_some()) {
            for square in path {
                if let Some(blocker) = board.get_piece(&square) {
                    if &square == &self.destination {
                        // Make sure that the piece we're ending on is a different color
                        if blocker.color == self.color {
                            return Err(ChessError::CannotCaptureOwnPiece);
                        }
                    } else {
                        return Err(ChessError::InvalidMove(format!(
                            "{} at {} cannot move to {} due to {} on {}",
                            piece, self.source, self.destination, blocker, square
                        )));
                    }
                }
            }
        } else {
            return Err(ChessError::InvalidMove(format!(
                "{} at {} cannot move to {}",
                piece, self.source, self.destination
            )));
        }

        // Make sure that if this is a promotion, it can be promoted
        if self.promotion.is_some() {
            if !piece.piece_type.can_be_promoted() {
                return Err(ChessError::InvalidPawnPromotion);
            } else if &self.destination.rank != &self.color.pawn_promotion_rank() {
                return Err(ChessError::InvalidPawnPromotion);
            }
        }
        todo!()
    }
}

struct Castle {
    color: Color,
    side: CastleSide,
}

impl Castle {
    fn rook_start_end_positions(&self) -> (Square, Square) {
        match &self.side {
            CastleSide::Queen => match &self.color {
                Color::White => (
                    Square {
                        file: File::A,
                        rank: Rank::One,
                    },
                    Square {
                        file: File::D,
                        rank: Rank::One,
                    },
                ),
                Color::Black => (
                    Square {
                        file: File::A,
                        rank: Rank::Eight,
                    },
                    Square {
                        file: File::D,
                        rank: Rank::Eight,
                    },
                ),
            },
            CastleSide::King => match &self.color {
                Color::White => (
                    Square {
                        file: File::H,
                        rank: Rank::One,
                    },
                    Square {
                        file: File::F,
                        rank: Rank::One,
                    },
                ),
                Color::Black => (
                    Square {
                        file: File::H,
                        rank: Rank::Eight,
                    },
                    Square {
                        file: File::F,
                        rank: Rank::Eight,
                    },
                ),
            },
        }
    }

    fn king_start_end_positions(&self) -> (Square, Square) {
        match &self.side {
            CastleSide::Queen => match &self.color {
                Color::White => (
                    Square {
                        file: File::E,
                        rank: Rank::One,
                    },
                    Square {
                        file: File::C,
                        rank: Rank::One,
                    },
                ),
                Color::Black => (
                    Square {
                        file: File::E,
                        rank: Rank::Eight,
                    },
                    Square {
                        file: File::C,
                        rank: Rank::Eight,
                    },
                ),
            },
            CastleSide::King => match &self.color {
                Color::White => (
                    Square {
                        file: File::E,
                        rank: Rank::One,
                    },
                    Square {
                        file: File::G,
                        rank: Rank::One,
                    },
                ),
                Color::Black => (
                    Square {
                        file: File::E,
                        rank: Rank::Eight,
                    },
                    Square {
                        file: File::G,
                        rank: Rank::Eight,
                    },
                ),
            },
        }
    }

    fn king_path(&self) -> Vec<Square> {
        match &self.side {
            CastleSide::Queen => match &self.color {
                Color::White => vec![
                    Square::new(File::D, Rank::One),
                    Square::new(File::C, Rank::One),
                ],
                Color::Black => vec![
                    Square::new(File::D, Rank::Eight),
                    Square::new(File::C, Rank::Eight),
                ],
            },
            CastleSide::King => match &self.color {
                Color::White => vec![
                    Square::new(File::F, Rank::One),
                    Square::new(File::G, Rank::One),
                ],
                Color::Black => vec![
                    Square::new(File::F, Rank::Eight),
                    Square::new(File::G, Rank::Eight),
                ],
            },
        }
    }
}

impl ChessMoveTrait for Castle {
    fn get_chess_move(self, board: &InternalBoardState) -> ChessMoveHistory {
        let (king_start, king_end) = self.king_start_end_positions();
        let (rook_start, rook_end) = self.king_start_end_positions();

        let pieces_moved = vec![
            (
                board
                    .get_piece(&king_start)
                    .expect("We verified this already"),
                king_start,
                king_end,
            ),
            (
                board
                    .get_piece(&rook_start)
                    .expect("We verified this already"),
                rook_start,
                rook_end,
            ),
        ];

        ChessMoveHistory {
            pieces_moved,
            captured: None,
            promoted: None,
        }
    }

    fn validate(&self, board: &InternalBoardState) -> Result<(), ChessError> {
        // TODO: Make sure we're not currently in check
        if board.is_in_check(&self.color) {
            return Err(ChessError::InvalidCastle(
                "Cannot castle out of check".to_string(),
            ));
        }

        let (rook_space, _) = self.rook_start_end_positions();

        // Make sure the king and rook haven't moved
        for previous_move in &board.move_history {
            for (piece, start, _) in &previous_move.pieces_moved {
                if &piece.color == &self.color && piece.piece_type == PieceType::King {
                    return Err(ChessError::InvalidCastle(format!("King has already moved")));
                } else if &piece.color == &self.color
                    && piece.piece_type == PieceType::Rook
                    && &rook_space == start
                {
                    return Err(ChessError::InvalidCastle(format!(
                        "{} Rook has already moved",
                        self.side
                    )));
                }
            }
        }

        // For each spot on the board where the king will move through, ensure we don't put ourselves in check (include the end)
        for square in self.king_path() {
            if board.square_in_check(&self.color, &square) {
                return Err(ChessError::InvalidCastle(format!(
                    "Cannot move through {}",
                    square
                )));
            }
        }

        Ok(())
    }
}

/// Represents a move used internaly by the board for tracking pending and future moves
#[derive(Clone)]
struct ChessMoveHistory {
    pieces_moved: Vec<(Piece, Square, Square)>,
    captured: Option<(Square, Piece)>,
    promoted: Option<PieceType>,
}

impl ChessMoveHistory {
    fn apply(&self, board: &mut InternalBoardState) {
        // First remove any pieces that were captured
        if let Some((square, _)) = &self.captured {
            board.board.remove(square);
        }

        // Then move the pieces (and promote them)
        for (_, start, finish) in &self.pieces_moved {
            // We don't do the validation, so we're trusting that everything here was good
            let mut piece = board.board.remove(start).expect("TODO: Message");

            if let Some(promotion) = &self.promoted {
                piece.piece_type = promotion.clone();
            }

            board.board.insert(finish.clone(), piece);
        }
    }

    fn rollback(&self, board: &mut InternalBoardState) {
        // TODO: Undo the move from the board
        // Move any pieces back to their original places (and type)
        for (piece, start, finish) in &self.pieces_moved {
            board.board.remove(finish);
            board.board.insert(start.clone(), piece.clone());
        }

        // Now replace the pieces that were captured
        if let Some((square, piece)) = &self.captured {
            board.board.insert(square.clone(), piece.clone());
        }
    }
}

/// Represents a piece on the chess board
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    /// Create a new chess piece
    fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type }
    }

    fn valid_paths(&self, current_location: &Square) -> Vec<Vec<Square>> {
        let mut result = Vec::new();

        match self.piece_type {
            PieceType::King => {
                append_diagonals(&mut result, current_location, Some(1));
                append_verticals(&mut result, current_location, Some(1));
            }
            PieceType::Queen => {
                append_diagonals(&mut result, current_location, None);
                append_verticals(&mut result, current_location, None);
            }
            PieceType::Bishop => {
                append_diagonals(&mut result, current_location, None);
            }
            PieceType::Knight => {
                append_knight_moves(&mut result, current_location);
            }
            PieceType::Rook => {
                append_verticals(&mut result, current_location, None);
            }
            PieceType::Pawn => {
                // One and Two
                let max = if current_location.rank == self.color.starting_pawn_rank() {
                    2
                } else {
                    1
                };

                append_square_line(
                    &mut result,
                    current_location,
                    self.color.get_pawn_direction(),
                    0,
                    Some(max),
                );

                // Captures left
                append_square_line(
                    &mut result,
                    current_location,
                    self.color.get_pawn_direction(),
                    -1,
                    Some(1),
                );
                append_square_line(
                    &mut result,
                    current_location,
                    self.color.get_pawn_direction(),
                    1,
                    Some(1),
                );
                // TODO: En Passant
            }
        };

        result
    }

    fn get_path_to(
        &self,
        current_location: &Square,
        destination: &Square,
        piece_at_destination: bool,
    ) -> Option<Vec<Square>> {
        let delta_file = destination.file.to_isize() - current_location.file.to_isize();
        let delta_rank = destination.rank.to_isize() - current_location.rank.to_isize();
        let mut paths = Vec::new();
        match self.piece_type {
            PieceType::King => {
                if delta_file.abs() <= 1 && delta_rank.abs() <= 1 {
                    paths.push(vec![destination.clone()]);
                }
            }
            PieceType::Queen => {
                if delta_file == 0 {
                    append_square_line(
                        &mut paths,
                        current_location,
                        0,
                        get_delta_direction(delta_rank),
                        Some(delta_rank.abs() as usize),
                    );
                } else if delta_rank == 0 {
                    append_square_line(
                        &mut paths,
                        current_location,
                        get_delta_direction(delta_file),
                        0,
                        Some(delta_file.abs() as usize),
                    );
                } else if delta_file.abs() == delta_rank.abs() {
                    append_square_line(
                        &mut paths,
                        current_location,
                        get_delta_direction(delta_file),
                        get_delta_direction(delta_rank),
                        Some(delta_file.abs() as usize),
                    );
                }
            }
            PieceType::Bishop => {
                if delta_file.abs() == delta_rank.abs() {
                    append_square_line(
                        &mut paths,
                        current_location,
                        get_delta_direction(delta_file),
                        get_delta_direction(delta_rank),
                        Some(delta_file.abs() as usize),
                    );
                }
            }
            PieceType::Knight => {
                if (delta_file.abs() == 2 && delta_rank.abs() == 1)
                    || (delta_rank.abs() == 2 && delta_file.abs() == 1)
                {
                    paths.push(vec![destination.clone()]);
                }
            }
            PieceType::Rook => {
                if delta_file == 0 {
                    append_square_line(
                        &mut paths,
                        current_location,
                        0,
                        get_delta_direction(delta_rank),
                        Some(delta_rank.abs() as usize),
                    );
                } else if delta_rank == 0 {
                    append_square_line(
                        &mut paths,
                        current_location,
                        get_delta_direction(delta_file),
                        0,
                        Some(delta_file.abs() as usize),
                    );
                }
            }
            PieceType::Pawn => {
                let mut pawn_moves = Vec::new();
                if delta_file == 0 {
                    if self.color.get_pawn_direction() == delta_rank {
                        pawn_moves.push(destination.clone())
                    } else if current_location.rank == self.color.starting_pawn_rank()
                        && (self.color.get_pawn_direction() * 2) == delta_rank
                    {
                        append_square_line(
                            &mut paths,
                            current_location,
                            0,
                            self.color.get_pawn_direction(),
                            Some(2),
                        );
                    }
                } else if delta_file.abs() == 1 {
                    if piece_at_destination && self.color.get_pawn_direction() == delta_rank {
                        pawn_moves.push(destination.clone())
                    }
                }

                // TODO: en passant
                if pawn_moves.len() > 0 {
                    paths.push(pawn_moves);
                }
            }
        }

        paths.pop()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color, self.piece_type)
    }
}

fn append_diagonals(vector: &mut Vec<Vec<Square>>, location: &Square, max: Option<usize>) {
    // Top Left
    append_square_line(vector, location, 1, -1, max);
    // Top Right
    append_square_line(vector, location, 1, 1, max);
    // Bottom Left
    append_square_line(vector, location, -1, -1, max);
    // Bottom Right
    append_square_line(vector, location, -1, 1, max);
}

fn append_verticals(vector: &mut Vec<Vec<Square>>, location: &Square, max: Option<usize>) {
    // Up
    append_square_line(vector, location, 1, 0, max);
    // Down
    append_square_line(vector, location, -1, 0, max);
    // Left
    append_square_line(vector, location, 0, -1, max);
    // Right
    append_square_line(vector, location, 0, 1, max);
}

fn append_knight_moves(vector: &mut Vec<Vec<Square>>, location: &Square) {
    let mut addition = Vec::new();
    // Two up one left
    if let Some(new_square) = try_square_add(location, 2, -1) {
        addition.push(new_square);
    }
    // Two up one right
    if let Some(new_square) = try_square_add(location, 2, 1) {
        addition.push(new_square);
    }
    // Two left one up
    if let Some(new_square) = try_square_add(location, 1, -2) {
        addition.push(new_square);
    }
    // Two left one down
    if let Some(new_square) = try_square_add(location, -1, -2) {
        addition.push(new_square);
    }
    // Two right one up
    if let Some(new_square) = try_square_add(location, 1, 2) {
        addition.push(new_square);
    }
    // Two right on down
    if let Some(new_square) = try_square_add(location, -1, 2) {
        addition.push(new_square);
    }
    // Two down one left
    if let Some(new_square) = try_square_add(location, -2, -1) {
        addition.push(new_square);
    }
    // Two down one right
    if let Some(new_square) = try_square_add(location, -2, 1) {
        addition.push(new_square);
    }

    if addition.len() > 0 {
        vector.push(addition);
    }
}

fn append_square_line(
    vector: &mut Vec<Vec<Square>>,
    location: &Square,
    file_delta: isize,
    rank_delta: isize,
    max: Option<usize>,
) {
    let max = max.unwrap_or(usize::MAX);
    let mut line = Vec::new();
    // Top Left
    let mut location = location.clone();
    for _ in 0..max {
        // Add delta to file, see if that's ok
        if let Some(new_square) = try_square_add(&location, file_delta, rank_delta) {
            location = new_square.clone();
            line.push(new_square);
        } else {
            break;
        }
    }

    if line.len() > 0 {
        vector.push(line);
    }
}

fn try_square_add(location: &Square, file_delta: isize, rank_delta: isize) -> Option<Square> {
    if let Ok(new_file) = location.file.add(file_delta) {
        if let Ok(new_rank) = location.rank.add(rank_delta) {
            return Some(Square {
                file: new_file,
                rank: new_rank,
            });
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
        let mut total = Vec::new();
        let starting_square = Square::new(File::A, Rank::One);
        append_square_line(&mut total, &starting_square, 1, 1, Some(2));

        assert_eq!(1, total.len());

        let vector = total.pop().unwrap();
        assert_eq!(2, vector.len());
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Two)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::C && v.rank == Rank::Three)
        );
    }

    #[test]
    fn append_square_line_handles_does_diagonal_up() {
        let mut total = Vec::new();
        let starting_square = Square::new(File::A, Rank::One);
        append_square_line(&mut total, &starting_square, 1, 1, None);

        assert_eq!(1, total.len());
        let vector = total.pop().unwrap();

        assert_eq!(7, vector.len());
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Two)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::C && v.rank == Rank::Three)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::D && v.rank == Rank::Four)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::F && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::G && v.rank == Rank::Seven)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::H && v.rank == Rank::Eight)
        );
    }

    #[test]
    fn append_square_line_handles_does_diagonal_down() {
        let mut total = Vec::new();
        let starting_square = Square::new(File::A, Rank::Eight);
        append_square_line(&mut total, &starting_square, 1, -1, None);

        assert_eq!(1, total.len());
        let vector = total.pop().unwrap();

        assert_eq!(7, vector.len());
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Seven)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::C && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::D && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Four)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::F && v.rank == Rank::Three)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::G && v.rank == Rank::Two)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::H && v.rank == Rank::One)
        );
    }

    #[test]
    fn append_square_line_handles_does_vertical_up() {
        let mut total = Vec::new();
        let starting_square = Square::new(File::E, Rank::One);
        append_square_line(&mut total, &starting_square, 0, 1, None);

        assert_eq!(1, total.len());
        let vector = total.pop().unwrap();

        assert_eq!(7, vector.len());
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Two)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Three)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Four)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Seven)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Eight)
        );
    }

    #[test]
    fn append_square_line_handles_does_vertical_down() {
        let mut total = Vec::new();
        let starting_square = Square::new(File::B, Rank::Eight);
        append_square_line(&mut total, &starting_square, 0, -1, None);

        assert_eq!(1, total.len());
        let vector = total.pop().unwrap();

        assert_eq!(7, vector.len());
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Seven)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Four)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Three)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Two)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::One)
        );
    }

    #[test]
    fn append_square_line_handles_does_horizontal_right() {
        let mut total = Vec::new();
        let starting_square = Square::new(File::A, Rank::Six);
        append_square_line(&mut total, &starting_square, 1, 0, None);

        assert_eq!(1, total.len());
        let vector = total.pop().unwrap();

        assert_eq!(7, vector.len());
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::C && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::D && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::F && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::G && v.rank == Rank::Six)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::H && v.rank == Rank::Six)
        );
    }

    #[test]
    fn append_square_line_handles_does_horizontal_left() {
        let mut total = Vec::new();
        let starting_square = Square::new(File::H, Rank::Five);
        append_square_line(&mut total, &starting_square, -1, 0, None);

        assert_eq!(1, total.len());
        let vector = total.pop().unwrap();

        assert_eq!(7, vector.len());
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::G && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::F && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::E && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::D && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::C && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::B && v.rank == Rank::Five)
        );
        assert!(
            vector
                .iter()
                .any(|v| v.file == File::A && v.rank == Rank::Five)
        );
    }

    // The King
    #[test]
    fn get_path_to_king_success_rank() {
        let source = Square::new(File::D, Rank::Four);
        let destination = Square::new(File::D, Rank::Three);
        let piece = Piece::new(Color::Black, PieceType::King);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::D && s.rank == Rank::Three)
        );
    }

    #[test]
    fn get_path_to_king_success_file() {
        let source = Square::new(File::D, Rank::Four);
        let destination = Square::new(File::E, Rank::Four);
        let piece = Piece::new(Color::Black, PieceType::King);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::E && s.rank == Rank::Four)
        );
    }

    #[test]
    fn get_path_to_king_success_diagonal() {
        let source = Square::new(File::D, Rank::Four);
        let destination = Square::new(File::E, Rank::Three);
        let piece = Piece::new(Color::Black, PieceType::King);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::E && s.rank == Rank::Three)
        );
    }

    #[test]
    fn get_path_to_king_failure() {
        let source = Square::new(File::D, Rank::Four);
        let destination = Square::new(File::F, Rank::Three);
        let piece = Piece::new(Color::Black, PieceType::King);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    // The Queen
    #[test]
    fn get_path_to_queen_success_rank() {
        let source = Square::new(File::A, Rank::Two);
        let destination = Square::new(File::A, Rank::Seven);
        let piece = Piece::new(Color::Black, PieceType::Queen);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(5, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::A && s.rank == Rank::Three)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::A && s.rank == Rank::Four)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::A && s.rank == Rank::Five)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::A && s.rank == Rank::Six)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::A && s.rank == Rank::Seven)
        );
    }

    #[test]
    fn get_path_to_queen_success_file() {
        let source = Square::new(File::C, Rank::Five);
        let destination = Square::new(File::F, Rank::Five);
        let piece = Piece::new(Color::Black, PieceType::Queen);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(3, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::D && s.rank == Rank::Five)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::E && s.rank == Rank::Five)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::F && s.rank == Rank::Five)
        );
    }

    #[test]
    fn get_path_to_queen_success_diagonal() {
        let source = Square::new(File::G, Rank::Seven);
        let destination = Square::new(File::B, Rank::Two);
        let piece = Piece::new(Color::Black, PieceType::Queen);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(5, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::F && s.rank == Rank::Six)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::E && s.rank == Rank::Five)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::D && s.rank == Rank::Four)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::C && s.rank == Rank::Three)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::B && s.rank == Rank::Two)
        );
    }

    #[test]
    fn get_path_to_queen_failure() {
        let source = Square::new(File::G, Rank::Seven);
        let destination = Square::new(File::E, Rank::Six);
        let piece = Piece::new(Color::Black, PieceType::Queen);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    // The Bishops
    #[test]
    fn get_path_to_bishop_success() {
        let source = Square::new(File::F, Rank::Seven);
        let destination = Square::new(File::B, Rank::Three);
        let piece = Piece::new(Color::Black, PieceType::Bishop);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(4, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::E && s.rank == Rank::Six)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::D && s.rank == Rank::Five)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::C && s.rank == Rank::Four)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::B && s.rank == Rank::Three)
        );
    }

    #[test]
    fn get_path_to_bishop_failure() {
        let source = Square::new(File::G, Rank::Seven);
        let destination = Square::new(File::G, Rank::One);
        let piece = Piece::new(Color::Black, PieceType::Bishop);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    // The Knights
    #[test]
    fn get_path_to_knight_success() {
        let source = Square::new(File::C, Rank::Five);
        let destination = Square::new(File::B, Rank::Seven);
        let piece = Piece::new(Color::Black, PieceType::Knight);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::B && s.rank == Rank::Seven)
        );
    }

    #[test]
    fn get_path_to_knight_failure() {
        let source = Square::new(File::F, Rank::Three);
        let destination = Square::new(File::D, Rank::Five);
        let piece = Piece::new(Color::Black, PieceType::Knight);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    // The Rooks
    #[test]
    fn get_path_to_rook_success_rank() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::C, Rank::Seven);
        let piece = Piece::new(Color::Black, PieceType::Rook);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(4, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::C && s.rank == Rank::Four)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::C && s.rank == Rank::Five)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::C && s.rank == Rank::Six)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::C && s.rank == Rank::Seven)
        );
    }

    #[test]
    fn get_path_to_rook_success_file() {
        let source = Square::new(File::B, Rank::Six);
        let destination = Square::new(File::E, Rank::Six);
        let piece = Piece::new(Color::Black, PieceType::Rook);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(3, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::C && s.rank == Rank::Six)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::D && s.rank == Rank::Six)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::E && s.rank == Rank::Six)
        );
    }

    #[test]
    fn get_path_to_rook_failure() {
        let source = Square::new(File::A, Rank::One);
        let destination = Square::new(File::G, Rank::Eight);
        let piece = Piece::new(Color::Black, PieceType::Rook);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    // The Pawns
    #[test]
    fn get_path_to_pawn_black_success_one() {
        let source = Square::new(File::B, Rank::Six);
        let destination = Square::new(File::B, Rank::Five);
        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::B && s.rank == Rank::Five)
        );
    }

    #[test]
    fn get_path_to_pawn_black_success_two() {
        let source = Square::new(File::F, Rank::Seven);
        let destination = Square::new(File::F, Rank::Five);
        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(2, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::F && s.rank == Rank::Six)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::F && s.rank == Rank::Five)
        );
    }

    #[test]
    fn get_path_to_pawn_black_success_one_capture() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::B, Rank::Two);
        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, true);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::B && s.rank == Rank::Two)
        );
    }

    #[test]
    fn get_path_to_pawn_black_failure_wrong_direction() {
        let source = Square::new(File::B, Rank::Six);
        let destination = Square::new(File::B, Rank::Seven);
        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    #[test]
    fn get_path_to_pawn_black_failure_two_after_move() {
        let source = Square::new(File::F, Rank::Six);
        let destination = Square::new(File::F, Rank::Four);
        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    #[test]
    fn get_path_to_pawn_black_failure_capture_no_target() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::B, Rank::Two);
        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    #[test]
    fn get_path_to_pawn_white_success_one() {
        let source = Square::new(File::A, Rank::Three);
        let destination = Square::new(File::A, Rank::Four);
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::A && s.rank == Rank::Four)
        );
    }

    #[test]
    fn get_path_to_pawn_white_success_two() {
        let source = Square::new(File::G, Rank::Two);
        let destination = Square::new(File::G, Rank::Four);
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(2, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::G && s.rank == Rank::Three)
        );
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::G && s.rank == Rank::Four)
        );
    }

    #[test]
    fn get_path_to_pawn_white_success_one_capture() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::D, Rank::Four);
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, true);

        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(1, unwrapped.len());
        assert!(
            unwrapped
                .iter()
                .any(|s| s.file == File::D && s.rank == Rank::Four)
        );
    }

    #[test]
    fn get_path_to_pawn_white_failure_wrong_direction() {
        let source = Square::new(File::B, Rank::Six);
        let destination = Square::new(File::B, Rank::Five);
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    #[test]
    fn get_path_to_pawn_white_failure_two_after_move() {
        let source = Square::new(File::F, Rank::Three);
        let destination = Square::new(File::F, Rank::Five);
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }

    #[test]
    fn get_path_to_pawn_white_failure_capture_no_target() {
        let source = Square::new(File::C, Rank::Three);
        let destination = Square::new(File::B, Rank::Four);
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = piece.get_path_to(&source, &destination, false);

        assert!(result.is_none());
    }
}
