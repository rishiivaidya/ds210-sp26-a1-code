use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let (_, best_move) = minimax(board, player, player, true);
        match best_move {
            Some((row, col)) => (0, row, col),
            None => (0, 0, 0),
        }
    }
}

fn minimax(board: &Board, current: Player, original: Player, is_maximizing: bool) -> (i32, Option<(usize, usize)>) {
    // BASE CASE - game is over, return score from perspective of original player
    if board.game_over() {
        let score = board.score();
        let perspective = if original == Player::X { score } else { -score }; // If original player is X, score is from X's perspective. If original player is O, score is from O's perspective (negate).
        return (perspective, None);
    }
    
    // Get all possible moves for current player
    let moves = board.moves();
    if moves.is_empty() {
        return (0, None);
    }
    
    // TODO: Implement recursive case - PERSON B
    // TODO: Return best score and move - PERSON B
    
    // Temporary placeholder to satisfy compiler until teammate adds code
    (0, None)
}