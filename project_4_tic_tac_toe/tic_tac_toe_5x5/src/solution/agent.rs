use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// fixed search depth for 5x5
const MAX_DEPTH: usize = 4;

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // start minimax at depth 0 from the current player's perspective
        let (score, best_move) = minimax(board, player, player, true);

        match best_move {
            Some((row, col)) => (score, row, col),
            None => (score, 0, 0),
        }
    }
}

// heuristic for unfinished boards
// starting simple: just use the board's current score
fn heuristic(board: &Board) -> i32 {
    board.score()
}

// wrapper function so solve can still call minimax normally
fn minimax(
    board: &mut Board,
    current: Player,
    original: Player,
    is_maximizing: bool,
) -> (i32, Option<(usize, usize)>) {
    minimax_depth(board, current, original, is_maximizing, 0, MAX_DEPTH)
}

// depth-limited minimax
fn minimax_depth(
    board: &mut Board,
    current: Player,
    original: Player,
    is_maximizing: bool,
    depth: usize,
    max_depth: usize,
) -> (i32, Option<(usize, usize)>) {
    // BASE CASE 1 - true game over
    // return score from the original player's perspective
    if board.game_over() {
        let score = board.score();
        let perspective = if original == Player::X { score } else { -score };
        return (perspective, None);
    }

    // BASE CASE 2 - reached search depth limit
    // use heuristic instead of searching deeper
    if depth == max_depth {
        let score = heuristic(board);
        let perspective = if original == Player::X { score } else { -score };
        return (perspective, None);
    }

    // get all available moves
    let moves = board.moves();
    if moves.is_empty() {
        return (0, None);
    }

    let mut best_move = None; // stores best move found for this board state

    if is_maximizing {
        let mut best_score = i32::MIN; // start as low as possible

        for m in moves {
            board.apply_move(m, current); // simulate move on shared board

            // recursive call for opponent's turn, increasing depth
            let (score, _) = minimax_depth(
                board,
                current.flip(),
                original,
                false,
                depth + 1,
                max_depth,
            );

            board.undo_move(m, current); // undo move so next branch starts clean

            // update best score and best move if this branch is better
            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        (best_score, best_move) // return best maximizing result
    } else {
        let mut best_score = i32::MAX; // start as high as possible

        for m in moves {
            board.apply_move(m, current); // simulate move

            // recursive call for next turn, increasing depth
            let (score, _) = minimax_depth(
                board,
                current.flip(),
                original,
                true,
                depth + 1,
                max_depth,
            );

            board.undo_move(m, current); // undo move after simulation

            // update best score and best move if this branch is better for minimizing
            if score < best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        (best_score, best_move) // return best minimizing result
    }
}