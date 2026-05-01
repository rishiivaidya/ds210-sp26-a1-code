use std::time::{Duration, Instant};
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

// heuristic estimates how good the board is when we can't search all the way to the end
// positive score favors X, negative favors O
fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells();
    let n = cells.len();
    let total_cells = n * n;

    // count empties by walking cells instead of calling board.moves() 
    let mut empty_count: usize = 0;
    for row in cells {
        for c in row {
            if matches!(c, Cell::Empty) {
                empty_count += 1;
            }
        }
    }

    // determine game stage based on how many empty cells remain
    // adjust weights so strategy changes throughout the game
    let (triplet_weight, two_weight, one_weight) =
        if empty_count > total_cells * 2 / 3 {
            
            (100, 10, 2)
        } else if empty_count > total_cells / 3 {
         
            (100, 20, 3)
        } else {
            
            (150, 40, 3)
        };

        //adjusting weight depending on stage in game

    let mut score: i32 = board.score() * triplet_weight as i32;

    //both X and O now prefer the cente
    let center = (n / 2) as i32;
    for i in 0..n {
        for j in 0..n {
            let dist = (i as i32 - center).abs() + (j as i32 - center).abs();
            let position_bonus = n as i32 - dist;
            match &cells[i][j] {
                Cell::X => score += position_bonus,
                Cell::O => score -= position_bonus,
                _ => {}
            }
        }
    }

    //window scanning in 4 directions
    for i in 0..n {
        for j in 0..n {
            let dirs: &[(i32, i32)] = &[(0, 1), (1, 0), (1, 1), (1, -1)];
            for (di, dj) in dirs {
                let i2 = i as i32 + di;
                let j2 = j as i32 + dj;
                let i3 = i as i32 + 2 * di;
                let j3 = j as i32 + 2 * dj;
                if i2 < 0 || j2 < 0 || i3 < 0 || j3 < 0 {
                    continue;
                }
                if i2 >= n as i32 || j2 >= n as i32 || i3 >= n as i32 || j3 >= n as i32 {
                    continue;
                }
                let a = &cells[i][j];
                let b = &cells[i2 as usize][j2 as usize];
                let c = &cells[i3 as usize][j3 as usize];
                score += eval_window_weighted(a, b, c, two_weight, one_weight);
            }
        }
    }

    score
}

fn eval_window_weighted(a: &Cell, b: &Cell, c: &Cell, two_weight: i32, one_weight: i32) -> i32 {
    let cells = [a, b, c];
    let x_count = cells.iter().filter(|&&c| c == &Cell::X).count();
    let o_count = cells.iter().filter(|&&c| c == &Cell::O).count();
    let empty_count = cells.iter().filter(|&&c| c == &Cell::Empty).count();

    // if both players have pieces in this window, neither can complete a triplet here
    if x_count > 0 && o_count > 0 {
        return 0;
    }
    // if the window contains a wall, it can never become a triplet
    if x_count + o_count + empty_count < 3 {
        return 0;
    }

    // two in a row with an open end; one move from completing a triplet
    if x_count == 2 && empty_count == 1 {
        return two_weight;
    }
    if o_count == 2 && empty_count == 1 {
        return -two_weight;
    }
    // single piece with open space
    if x_count == 1 && empty_count == 2 {
        return one_weight;
    }
    if o_count == 1 && empty_count == 2 {
        return -one_weight;
    }
    0
}

// order moves before searching them

fn order_moves(
    board: &mut Board,
    moves: Vec<(usize, usize)>,
    player: Player,
) -> Vec<(usize, usize)> {
    let opp = player.flip();
    // sign = +1 for X (higher raw score is good), -1 for O (lower is good).
    let sign = match player {
        Player::X => 1,
        Player::O => -1,
    };

    let mut scored: Vec<(i32, (usize, usize))> = moves
        .into_iter()
        .map(|m| {
            board.apply_move(m, player);
            let after_us = board.score();
            board.undo_move(m, player);

            board.apply_move(m, opp);
            let after_opp = board.score();
            board.undo_move(m, opp);

            // both terms get the same sign treatment so "good for the side to
            // move" is always a high number, regardless of which side that is
            let priority = sign * (after_us - after_opp);
            (priority, m)
        })
        .collect();

    scored.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, m)| m).collect()
}

fn minimax(
    board: &mut Board,
    player: Player,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    deadline: &Instant, // now an absolute deadline (Instant), not a start time
) -> Option<(i32, usize, usize)> {
    // single source of truth for time

    if Instant::now() >= *deadline {
        return None;
    }

    if board.game_over() {
        return Some((board.score(), 0, 0));
    }
    if depth == 0 {
        return Some((heuristic(board), 0, 0));
    }

    let moves = board.moves();
    //order moves before searching them
    let ordered = order_moves(board, moves, player);

    let mut best_move = ordered[0]; // safe default — guaranteed to exist since game is not over
    let mut best_score = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };

    for m in ordered {
        board.apply_move(m, player);
        let result = minimax(board, player.flip(), depth - 1, alpha, beta, deadline);
        board.undo_move(m, player);

        let (score, _, _) = match result {
            Some(r) => r,
            None => return None,
        };

        match player {
            Player::X => {
                if score > best_score {
                    best_score = score;
                    best_move = m;
                }
                if best_score > alpha {
                    alpha = best_score;
                }
            }
            Player::O => {
                if score < best_score {
                    best_score = score;
                    best_move = m;
                }
                if best_score < beta {
                    beta = best_score;
                }
            }
        }

        if alpha >= beta {
            break;
        }
    }

    Some((best_score, best_move.0, best_move.1))
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
        let start = Instant::now(); 
        let budget = Duration::from_millis(time_limit).mul_f64(0.90);
        let deadline = start + budget;

        //safe default in case time runs out before depth 1
        let first_moves = board.moves();
        let mut best = (0, first_moves[0].0, first_moves[0].1);

        // iterative deepening - try depth 1, 2, 3 etc until time runs out
        for depth in 1.. {
            match minimax(board, player, depth, i32::MIN, i32::MAX, &deadline) {
                Some(result) => {
                    best = result;
                    // If we already burned most of the budget on this depth,
                    // the next iteration won't finish
                    if Instant::now() >= deadline {
                        break;
                    }
                }
                None => break,
            }
        }

        best
    }
}