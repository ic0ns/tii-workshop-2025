use std::fmt;

/// Represents a player in the Tic-tac-toe game
///
/// # Examples
///
/// ```
/// use p22::tictac::Player;
/// 
/// let player = Player::X;
/// assert_eq!(format!("{}", player), "X");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

/// Represents a cell in the Tic-tac-toe field
///
/// # Examples
///
/// ```
/// use p22::tictac::{Cell, Player};
/// 
/// let cell = Cell::Empty;
/// assert_eq!(format!("{}", cell), " ");
/// 
/// let cell = Cell::Filled(Player::X);
/// assert_eq!(format!("{}", cell), "X");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Filled(Player),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Filled(player) => write!(f, "{}", player),
        }
    }
}

/// Represents the result of a game analysis
///
/// # Examples
///
/// ```
/// use p22::tictac::GameResult;
/// 
/// let result = GameResult::GameOn;
/// assert_eq!(result.to_string(), "Game is still ongoing");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    WinX,
    WinO,
    Draw,
    GameOn,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::WinX => write!(f, "Player X won"),
            GameResult::WinO => write!(f, "Player O won"),
            GameResult::Draw => write!(f, "Game ended in a draw"),
            GameResult::GameOn => write!(f, "Game is still ongoing"),
        }
    }
}

/// Error type for Tic-tac-toe game operations
///
/// # Examples
///
/// ```
/// use p22::tictac::TicTacError;
/// 
/// let error = TicTacError::OutOfBounds;
/// assert_eq!(error.to_string(), "Position is out of bounds");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TicTacError {
    OutOfBounds,
    CellAlreadyFilled,
    GameAlreadyOver,
}

impl fmt::Display for TicTacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicTacError::OutOfBounds => write!(f, "Position is out of bounds"),
            TicTacError::CellAlreadyFilled => write!(f, "Cell is already filled"),
            TicTacError::GameAlreadyOver => write!(f, "Game is already over"),
        }
    }
}

impl std::error::Error for TicTacError {}

/// Represents a Tic-tac-toe game field
///
/// # Examples
///
/// ```
/// use p22::tictac::TicTacField;
/// 
/// let field = TicTacField::new();
/// assert_eq!(field.analyze().to_string(), "Game is still ongoing");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TicTacField {
    field: [[Cell; 3]; 3],
}

impl TicTacField {
    /// Creates a new empty Tic-tac-toe field
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::tictac::TicTacField;
    /// 
    /// let field = TicTacField::new();
    /// ```
    pub fn new() -> Self {
        Self {
            field: [[Cell::Empty; 3]; 3],
        }
    }

    /// Gets the cell at the specified position
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::tictac::{TicTacField, Cell};
    /// 
    /// let field = TicTacField::new();
    /// assert_eq!(field.get(0, 0), Cell::Empty);
    /// ```
    pub fn get(&self, x: usize, y: usize) -> Cell {
        if x < 3 && y < 3 {
            self.field[y][x]
        } else {
            Cell::Empty
        }
    }

    /// Analyzes the field to determine the game result
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::tictac::{TicTacField, GameResult};
    /// 
    /// let field = TicTacField::new();
    /// assert_eq!(field.analyze(), GameResult::GameOn);
    /// ```
    pub fn analyze(&self) -> GameResult {
        // Check rows
        for row in 0..3 {
            if self.field[row][0] != Cell::Empty
                && self.field[row][0] == self.field[row][1]
                && self.field[row][1] == self.field[row][2]
            {
                match self.field[row][0] {
                    Cell::Filled(Player::X) => return GameResult::WinX,
                    Cell::Filled(Player::O) => return GameResult::WinO,
                    Cell::Empty => {}
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if self.field[0][col] != Cell::Empty
                && self.field[0][col] == self.field[1][col]
                && self.field[1][col] == self.field[2][col]
            {
                match self.field[0][col] {
                    Cell::Filled(Player::X) => return GameResult::WinX,
                    Cell::Filled(Player::O) => return GameResult::WinO,
                    Cell::Empty => {}
                }
            }
        }

        // Check diagonals
        if self.field[0][0] != Cell::Empty
            && self.field[0][0] == self.field[1][1]
            && self.field[1][1] == self.field[2][2]
        {
            match self.field[0][0] {
                Cell::Filled(Player::X) => return GameResult::WinX,
                Cell::Filled(Player::O) => return GameResult::WinO,
                Cell::Empty => {}
            }
        }

        if self.field[0][2] != Cell::Empty
            && self.field[0][2] == self.field[1][1]
            && self.field[1][1] == self.field[2][0]
        {
            match self.field[0][2] {
                Cell::Filled(Player::X) => return GameResult::WinX,
                Cell::Filled(Player::O) => return GameResult::WinO,
                Cell::Empty => {}
            }
        }

        // Check if board is full (draw)
        let mut is_full = true;
        for row in 0..3 {
            for col in 0..3 {
                if self.field[row][col] == Cell::Empty {
                    is_full = false;
                    break;
                }
            }
            if !is_full {
                break;
            }
        }

        if is_full {
            GameResult::Draw
        } else {
            GameResult::GameOn
        }
    }
}

/// Makes a move on the Tic-tac-toe field
///
/// # Arguments
///
/// * `field` - The current field
/// * `x` - The x coordinate (0-2)
/// * `y` - The y coordinate (0-2)
/// * `player` - The player making the move
///
/// # Returns
///
/// * `Ok(TicTacField)` - The updated field after the move
/// * `Err(TicTacError)` - Error if the move is invalid
///
/// # Examples
///
/// ```
/// use p22::tictac::{make_move, TicTacField, Player};
/// 
/// let field = TicTacField::new();
/// let updated_field = make_move(field, 0, 0, Player::X).unwrap();
/// ```
pub fn make_move(field: TicTacField, x: u32, y: u32, player: Player) -> Result<TicTacField, TicTacError> {
    // Check if game is already over
    if field.analyze() != GameResult::GameOn {
        return Err(TicTacError::GameAlreadyOver);
    }

    // Check if x and y are within bounds
    if x >= 3 || y >= 3 {
        return Err(TicTacError::OutOfBounds);
    }

    // Create a mutable copy of the field
    let mut new_field = field.clone();

    // Check if the cell is empty
    if new_field.field[y as usize][x as usize] != Cell::Empty {
        return Err(TicTacError::CellAlreadyFilled);
    }

    // Make the move
    new_field.field[y as usize][x as usize] = Cell::Filled(player);

    Ok(new_field)
}

impl fmt::Display for TicTacField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "-------")?;
        for row in 0..3 {
            write!(f, "|")?;
            for col in 0..3 {
                write!(f, "{}|", self.field[row][col])?;
            }
            writeln!(f)?;
            writeln!(f, "-------")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_field() {
        let field = TicTacField::new();
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(field.field[row][col], Cell::Empty);
            }
        }
    }

    #[test]
    fn test_analyze_empty_field() {
        let field = TicTacField::new();
        assert_eq!(field.analyze(), GameResult::GameOn);
    }

    #[test]
    fn test_analyze_row_win_x() {
        let mut field = TicTacField::new();
        field.field[0][0] = Cell::Filled(Player::X);
        field.field[0][1] = Cell::Filled(Player::X);
        field.field[0][2] = Cell::Filled(Player::X);
        assert_eq!(field.analyze(), GameResult::WinX);
    }

    #[test]
    fn test_analyze_row_win_o() {
        let mut field = TicTacField::new();
        field.field[1][0] = Cell::Filled(Player::O);
        field.field[1][1] = Cell::Filled(Player::O);
        field.field[1][2] = Cell::Filled(Player::O);
        assert_eq!(field.analyze(), GameResult::WinO);
    }

    #[test]
    fn test_analyze_column_win_x() {
        let mut field = TicTacField::new();
        field.field[0][0] = Cell::Filled(Player::X);
        field.field[1][0] = Cell::Filled(Player::X);
        field.field[2][0] = Cell::Filled(Player::X);
        assert_eq!(field.analyze(), GameResult::WinX);
    }

    #[test]
    fn test_analyze_diagonal_win_o() {
        let mut field = TicTacField::new();
        field.field[0][0] = Cell::Filled(Player::O);
        field.field[1][1] = Cell::Filled(Player::O);
        field.field[2][2] = Cell::Filled(Player::O);
        assert_eq!(field.analyze(), GameResult::WinO);
    }

    #[test]
    fn test_analyze_anti_diagonal_win_x() {
        let mut field = TicTacField::new();
        field.field[0][2] = Cell::Filled(Player::X);
        field.field[1][1] = Cell::Filled(Player::X);
        field.field[2][0] = Cell::Filled(Player::X);
        assert_eq!(field.analyze(), GameResult::WinX);
    }

    #[test]
    fn test_analyze_draw() {
        let mut field = TicTacField::new();
        // X | O | X
        // O | X | O
        // O | X | O
        field.field[0][0] = Cell::Filled(Player::X);
        field.field[0][1] = Cell::Filled(Player::O);
        field.field[0][2] = Cell::Filled(Player::X);
        field.field[1][0] = Cell::Filled(Player::O);
        field.field[1][1] = Cell::Filled(Player::X);
        field.field[1][2] = Cell::Filled(Player::O);
        field.field[2][0] = Cell::Filled(Player::O);
        field.field[2][1] = Cell::Filled(Player::X);
        field.field[2][2] = Cell::Filled(Player::O);
        assert_eq!(field.analyze(), GameResult::Draw);
    }

    #[test]
    fn test_make_move_valid() {
        let field = TicTacField::new();
        let result = make_move(field, 1, 1, Player::X);
        assert!(result.is_ok());
        
        let updated_field = result.unwrap();
        assert_eq!(updated_field.field[1][1], Cell::Filled(Player::X));
    }

    #[test]
    fn test_make_move_out_of_bounds() {
        let field = TicTacField::new();
        let result = make_move(field, 3, 1, Player::X);
        assert_eq!(result, Err(TicTacError::OutOfBounds));
    }

    #[test]
    fn test_make_move_cell_already_filled() {
        let mut field = TicTacField::new();
        field.field[0][0] = Cell::Filled(Player::X);
        let result = make_move(field, 0, 0, Player::O);
        assert_eq!(result, Err(TicTacError::CellAlreadyFilled));
    }

    #[test]
    fn test_make_move_game_already_over() {
        let mut field = TicTacField::new();
        field.field[0][0] = Cell::Filled(Player::X);
        field.field[0][1] = Cell::Filled(Player::X);
        field.field[0][2] = Cell::Filled(Player::X);
        let result = make_move(field, 1, 1, Player::O);
        assert_eq!(result, Err(TicTacError::GameAlreadyOver));
    }

    #[test]
    fn test_game_sequence() {
        let mut field = TicTacField::new();
        
        // X makes a move
        field = make_move(field, 0, 0, Player::X).unwrap();
        assert_eq!(field.analyze(), GameResult::GameOn);
        
        // O makes a move
        field = make_move(field, 1, 0, Player::O).unwrap();
        assert_eq!(field.analyze(), GameResult::GameOn);
        
        // X makes a move
        field = make_move(field, 0, 1, Player::X).unwrap();
        assert_eq!(field.analyze(), GameResult::GameOn);
        
        // O makes a move
        field = make_move(field, 1, 1, Player::O).unwrap();
        assert_eq!(field.analyze(), GameResult::GameOn);
        
        // X makes a move and wins
        field = make_move(field, 0, 2, Player::X).unwrap();
        assert_eq!(field.analyze(), GameResult::WinX);
    }
}