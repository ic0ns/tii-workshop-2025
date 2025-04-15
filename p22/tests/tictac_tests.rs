use p22::tictac::{TicTacField, Player, Cell, GameResult, TicTacError, make_move};

#[test]
fn test_create_empty_field() {
    let field = TicTacField::new();
    assert_eq!(field.analyze(), GameResult::GameOn);
    
    // Check that all cells are empty
    for y in 0..3 {
        for x in 0..3 {
            assert_eq!(field.get(x, y), Cell::Empty);
        }
    }
}

#[test]
fn test_winning_conditions() {
    // Test row win
    let mut field = TicTacField::new();
    field = make_move(field, 0, 0, Player::X).unwrap();
    field = make_move(field, 0, 1, Player::O).unwrap();
    field = make_move(field, 1, 0, Player::X).unwrap();
    field = make_move(field, 1, 1, Player::O).unwrap();
    field = make_move(field, 2, 0, Player::X).unwrap();
    assert_eq!(field.analyze(), GameResult::WinX);
    
    // Test column win
    let mut field = TicTacField::new();
    field = make_move(field, 0, 0, Player::O).unwrap();
    field = make_move(field, 1, 0, Player::X).unwrap();
    field = make_move(field, 0, 1, Player::O).unwrap();
    field = make_move(field, 1, 1, Player::X).unwrap();
    field = make_move(field, 0, 2, Player::O).unwrap();
    assert_eq!(field.analyze(), GameResult::WinO);
    
    // Test diagonal win
    let mut field = TicTacField::new();
    field = make_move(field, 0, 0, Player::X).unwrap();
    field = make_move(field, 0, 1, Player::O).unwrap();
    field = make_move(field, 1, 1, Player::X).unwrap();
    field = make_move(field, 0, 2, Player::O).unwrap();
    field = make_move(field, 2, 2, Player::X).unwrap();
    assert_eq!(field.analyze(), GameResult::WinX);
    
    // Test anti-diagonal win
    let mut field = TicTacField::new();
    field = make_move(field, 2, 0, Player::O).unwrap();
    field = make_move(field, 0, 0, Player::X).unwrap();
    field = make_move(field, 1, 1, Player::O).unwrap();
    field = make_move(field, 1, 0, Player::X).unwrap();
    field = make_move(field, 0, 2, Player::O).unwrap();
    assert_eq!(field.analyze(), GameResult::WinO);
}

#[test]
fn test_draw_condition() {
    // Set up a draw situation
    let mut field = TicTacField::new();
    
    // X | O | X
    // O | X | O
    // O | X | O
    field = make_move(field, 0, 0, Player::X).unwrap();
    field = make_move(field, 1, 0, Player::O).unwrap();
    field = make_move(field, 2, 0, Player::X).unwrap();
    
    field = make_move(field, 0, 1, Player::O).unwrap();
    field = make_move(field, 1, 1, Player::X).unwrap();
    field = make_move(field, 2, 1, Player::O).unwrap();
    
    field = make_move(field, 0, 2, Player::O).unwrap();
    field = make_move(field, 1, 2, Player::X).unwrap();
    
    // Game should still be on
    assert_eq!(field.analyze(), GameResult::GameOn);
    
    // Last move
    field = make_move(field, 2, 2, Player::O).unwrap();
    
    // Now it's a draw
    assert_eq!(field.analyze(), GameResult::Draw);
}

#[test]
fn test_error_handling() {
    let field = TicTacField::new();
    
    // Test out of bounds
    assert_eq!(
        make_move(field.clone(), 3, 0, Player::X),
        Err(TicTacError::OutOfBounds)
    );
    
    assert_eq!(
        make_move(field.clone(), 0, 3, Player::X),
        Err(TicTacError::OutOfBounds)
    );
    
    // Test cell already filled
    let field = make_move(field, 1, 1, Player::X).unwrap();
    assert_eq!(
        make_move(field.clone(), 1, 1, Player::O),
        Err(TicTacError::CellAlreadyFilled)
    );
    
    // Test game already over
    let mut field = TicTacField::new();
    field = make_move(field, 0, 0, Player::X).unwrap();
    field = make_move(field, 1, 0, Player::X).unwrap();
    field = make_move(field, 2, 0, Player::X).unwrap();
    
    assert_eq!(
        make_move(field, 1, 1, Player::O),
        Err(TicTacError::GameAlreadyOver)
    );
}

#[test]
fn test_display_implementation() {
    let mut field = TicTacField::new();
    field = make_move(field, 0, 0, Player::X).unwrap();
    field = make_move(field, 1, 1, Player::O).unwrap();
    
    let expected = "-------\n|X| | |\n-------\n| |O| |\n-------\n| | | |\n-------\n";
    assert_eq!(format!("{}", field), expected);
}

#[test]
fn test_game_flow() {
    let mut field = TicTacField::new();
    
    // Alternate moves between X and O
    field = make_move(field, 0, 0, Player::X).unwrap();
    assert_eq!(field.get(0, 0), Cell::Filled(Player::X));
    
    field = make_move(field, 1, 0, Player::O).unwrap();
    assert_eq!(field.get(1, 0), Cell::Filled(Player::O));
    
    field = make_move(field, 0, 1, Player::X).unwrap();
    assert_eq!(field.get(0, 1), Cell::Filled(Player::X));
    
    field = make_move(field, 1, 1, Player::O).unwrap();
    assert_eq!(field.get(1, 1), Cell::Filled(Player::O));
    
    field = make_move(field, 0, 2, Player::X).unwrap();
    
    // X has won with a column
    assert_eq!(field.analyze(), GameResult::WinX);
    
    // Should not be able to make more moves
    assert_eq!(
        make_move(field, 2, 2, Player::O),
        Err(TicTacError::GameAlreadyOver)
    );
}