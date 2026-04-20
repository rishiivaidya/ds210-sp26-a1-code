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
        let (score, best_move) = minimax(board, player, player, true);
        match best_move {
            Some((row, col)) => (score, row, col),
            None => (score, 0, 0),
        }
    }
}

fn minimax(board: &mut Board, current: Player, original: Player, is_maximizing: bool) -> (i32, Option<(usize, usize)>) {
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

    let mut best_move = None; //stores best move found for board state

    if is_maximizing {
        let mut best_score = i32::MIN; //start at lowest

        for m in moves {
            board.apply_move(m, current); //simulate making move on shared board

            let (score, _) = minimax(board, current.flip(), original, false); //recursive call for
            //opponent's best response

            board.undo_move(m, current); //undo move to reset board for next branch

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        (best_score, best_move) //best for maximizing player

    } else {
        let mut best_score = i32::MAX; //start at highest

        for m in moves {
            board.apply_move(m, current); //simulate move

            let (score, _) = minimax(board, current.flip(), original, true); //back to maxing

            board.undo_move(m, current); //undo move

            if score < best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        (best_score, best_move) //best for min player
    }
}