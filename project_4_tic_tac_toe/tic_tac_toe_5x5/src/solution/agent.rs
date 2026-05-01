use std::time::{Duration, Instant};
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

//count how many length-3 windows through (i,j) contain no walls.
fn alive_lines_through(cells: &[Vec<Cell>], n: usize, i: usize, j: usize) -> i32 {
    let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
    let mut count = 0;
    for (di, dj) in &dirs {
        for offset in -2..=0i32 {
            let i_start = i as i32 + offset * di;
            let j_start = j as i32 + offset * dj;
            let mut alive = true;
            for k in 0..3i32 {
                let r = i_start + k * di;
                let c = j_start + k * dj;
                if r < 0 || c < 0 || r >= n as i32 || c >= n as i32 {
                    alive = false;
                    break;
                }
                if matches!(cells[r as usize][c as usize], Cell::Wall) {
                    alive = false;
                    break;
                }
            }
            if alive {
                count += 1;
            }
        }
    }
    count
}

//count open-twos (length-3 windows containing exactly 2 of the
//player's pieces and 1 empty square; one move from a triplet)

fn count_open_twos(cells: &[Vec<Cell>], n: usize, target: &Cell) -> i32 {
    let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            for (di, dj) in &dirs {
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
                let window = [
                    &cells[i][j],
                    &cells[i2 as usize][j2 as usize],
                    &cells[i3 as usize][j3 as usize],
                ];
                let mut t = 0;
                let mut e = 0;
                let mut blocked = false;
                for &c in &window {
                    if c == target {
                        t += 1;
                    } else if matches!(c, Cell::Empty) {
                        e += 1;
                    } else {
                        // wall or opponent piece, this window is dead
                        blocked = true;
                        break;
                    }
                }
                if !blocked && t == 2 && e == 1 {
                    count += 1;
                }
            }
        }
    }
    count
}

//heuristic estimates how good the board is when we can't search all the way to the end
//positive score favors X, negative favors O
fn heuristic(board: &Board, heuristic_called: &mut bool) -> i32 {
    *heuristic_called = true;

    let cells = board.get_cells();
    let n = cells.len();
    let total_cells = n * n;

    let mut empty_count: usize = 0;
    for row in cells {
        for c in row {
            if matches!(c, Cell::Empty) {
                empty_count += 1;
            }
        }
    }

    if empty_count <= 4 {
        return board.score() * 1000;
    }

    let (triplet_weight, two_weight, one_weight) =
        if empty_count > total_cells * 2 / 3 {
            (100, 10, 2)
        } else if empty_count > total_cells / 3 {
            (100, 20, 3)
        } else {
            (150, 40, 3)
        };

    let mut score: i32 = board.score() * triplet_weight as i32;

    //wall-aware position bonus
    for i in 0..n {
        for j in 0..n {
            let bonus = alive_lines_through(cells, n, i, j);
            match &cells[i][j] {
                Cell::X => score += bonus,
                Cell::O => score -= bonus,
                _ => {}
            }
        }
    }

    //window scanning in 4 directions (linear open-two and one-piece scoring)
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

    //fork detection via a quadratic open-two term
    
    const FORK_FACTOR: i32 = 20;
    let x_threats = count_open_twos(cells, n, &Cell::X);
    let o_threats = count_open_twos(cells, n, &Cell::O);
    score += x_threats * x_threats * FORK_FACTOR;
    score -= o_threats * o_threats * FORK_FACTOR;

    score
}

fn eval_window_weighted(a: &Cell, b: &Cell, c: &Cell, two_weight: i32, one_weight: i32) -> i32 {
    let cells = [a, b, c];
    let x_count = cells.iter().filter(|&&c| c == &Cell::X).count();
    let o_count = cells.iter().filter(|&&c| c == &Cell::O).count();
    let empty_count = cells.iter().filter(|&&c| c == &Cell::Empty).count();

    if x_count > 0 && o_count > 0 {
        return 0;
    }
    if x_count + o_count + empty_count < 3 {
        return 0;
    }

    if x_count == 2 && empty_count == 1 {
        return two_weight;
    }
    if o_count == 2 && empty_count == 1 {
        return -two_weight;
    }
    if x_count == 1 && empty_count == 2 {
        return one_weight;
    }
    if o_count == 1 && empty_count == 2 {
        return -one_weight;
    }
    0
}

fn order_moves(
    board: &mut Board,
    moves: Vec<(usize, usize)>,
    player: Player,
) -> Vec<(usize, usize)> {
    let opp = player.flip();
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
    deadline: &Instant,
    heuristic_called: &mut bool,
) -> Option<(i32, usize, usize)> {
    if Instant::now() >= *deadline {
        return None;
    }

    if board.game_over() {
        return Some((board.score(), 0, 0));
    }
    if depth == 0 {
        return Some((heuristic(board, heuristic_called), 0, 0));
    }

    let moves = board.moves();
    let ordered = order_moves(board, moves, player);

    let mut best_move = ordered[0];
    let mut best_score = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };

    for m in ordered {
        board.apply_move(m, player);
        let result = minimax(board, player.flip(), depth - 1, alpha, beta, deadline, heuristic_called);
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

        let first_moves = board.moves();
        let mut best = (0, first_moves[0].0, first_moves[0].1);

        for depth in 1.. {
            let iter_start = Instant::now();
            let mut heuristic_called = false;

            match minimax(
                board,
                player,
                depth,
                i32::MIN,
                i32::MAX,
                &deadline,
                &mut heuristic_called,
            ) {
                Some(result) => {
                    best = result;

                    if !heuristic_called {
                        break;
                    }

                    let now = Instant::now();
                    if now >= deadline {
                        break;
                    }

                    let iter_elapsed = now - iter_start;
                    let remaining = deadline.saturating_duration_since(now);
                    if iter_elapsed.saturating_mul(3) > remaining {
                        break;
                    }
                }
                None => break,
            }
        }

        best
    }
}
