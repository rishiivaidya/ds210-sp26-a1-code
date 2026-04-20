use std::time::Instant;
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

struct SearchConfig {
    original: Player,
    start: Instant,
    budget_ms: u64,
}

impl SearchConfig {
    fn timed_out(&self) -> bool {
        self.start.elapsed().as_millis() as u64 > self.budget_ms
    }

    fn perspective(&self, score: i32) -> i32 {
        if self.original == Player::X { score } else { -score }
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
        let config = SearchConfig {
            original: player,
            start: Instant::now(),
            budget_ms: (time_limit as f64 * 0.75) as u64,
        };

        let (fr, fc) = board.moves()[0];
        let mut best = (0, fr, fc);

        for depth in 1..=20 {
            if config.timed_out() { break; }
            if let Some((score, r, c)) = search(board, player, depth, i32::MIN, i32::MAX, &config) {
                best = (score, r, c);
            }
        }
        best
    }
}

fn search(
    board: &mut Board,
    current: Player,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
    config: &SearchConfig,
) -> Option<(i32, usize, usize)> {
    if config.timed_out() {
        return None;
    }

    if board.game_over() {
        return Some((config.perspective(board.score()), 0, 0));
    }

    let moves = board.moves();
    if moves.is_empty() || depth == 0 {
        return Some((heuristic(board, config), 0, 0));
    }

    let ordered = order_moves(board, moves, current);
    let maximizing = current == config.original;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    let mut best_move = (0, 0);

    for m in ordered {
        board.apply_move(m, current);
        let result = search(board, current.flip(), depth - 1, alpha, beta, config);
        board.undo_move(m, current);

        let score = match result {
            Some((s, _, _)) => s,
            None => return None,
        };

        if maximizing && score > best_score || !maximizing && score < best_score {
            best_score = score;
            best_move = m;
        }

        if maximizing { alpha = alpha.max(best_score); }
        else          { beta  = beta.min(best_score);  }

        if beta <= alpha { break; }
    }

    Some((best_score, best_move.0, best_move.1))
}

fn order_moves(board: &mut Board, moves: Vec<(usize, usize)>, current: Player) -> Vec<(usize, usize)> {
    let mut scored: Vec<(i32, (usize, usize))> = moves
        .into_iter()
        .map(|m| {
            board.apply_move(m, current);
            let s = board.score();
            board.undo_move(m, current);
            (s, m)
        })
        .collect();

    if current == Player::X {
        scored.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    } else {
        scored.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    }

    scored.into_iter().map(|(_, m)| m).collect()
}

fn heuristic(board: &Board, config: &SearchConfig) -> i32 {
    let cells = board.get_cells();
    let size = cells.len();
    let total_cells = size * size;

    // Count filled cells to determine game stage
    let filled = cells.iter().flatten()
        .filter(|c| **c != Cell::Empty && **c != Cell::Wall)
        .count();
    let empty = cells.iter().flatten()
        .filter(|c| **c == Cell::Empty)
        .count();

    // Early game: weight potential heavily
    // Late game: weight actual score heavily
    let progress = filled as f32 / total_cells as f32;
    let score_weight    = (50.0 + 200.0 * progress) as i32;
    let potential_weight = (100.0 - 80.0 * progress) as i32;

    let current_score   = config.perspective(board.score());
    let my_potential    = count_potential(cells, size, config.original);
    let their_potential = count_potential(cells, size, config.original.flip());

    // Penalize moves that leave opponent with open threats
    // Bonus for moves near the center (more scoring opportunities)
    let center_bonus = center_control(cells, size, config.original);

    // Urgency: if board is nearly full, raw score matters most
    if empty <= 4 {
        return current_score * 1000;
    }

    current_score * score_weight
        + (my_potential - their_potential) * potential_weight
        + center_bonus * 5
}

fn center_control(cells: &Vec<Vec<Cell>>, size: usize, player: Player) -> i32 { 
    let player_cell = match player { Player::X => Cell::X, Player::O => Cell::O };
    let mut score = 0;
    let center = size / 2;

    for i in 0..size {
        for j in 0..size {
            if cells[i][j] == player_cell {
                // Closer to center = higher value
                let dist = ((i as i32 - center as i32).abs() + (j as i32 - center as i32).abs()) as i32;
                score += (size as i32) - dist;
            }
        }
    }
    score
}

fn count_potential(cells: &Vec<Vec<Cell>>, size: usize, player: Player) -> i32 { 
    let player_cell = match player { Player::X => Cell::X, Player::O => Cell::O };
    let mut score = 0;

    for i in 0..size {
        for j in 0..size {
            if j + 2 < size {
                score += score_window(
                    [&cells[i][j], &cells[i][j+1], &cells[i][j+2]],
                    &player_cell,
                );
            }
            if i + 2 < size {
                score += score_window(
                    [&cells[i][j], &cells[i+1][j], &cells[i+2][j]],
                    &player_cell,
                );
            }
            if i + 2 < size && j + 2 < size {
                score += score_window(
                    [&cells[i][j], &cells[i+1][j+1], &cells[i+2][j+2]],
                    &player_cell,
                );
            }
            if i + 2 < size && j >= 2 {
                score += score_window(
                    [&cells[i][j], &cells[i+1][j-1], &cells[i+2][j-2]],
                    &player_cell,
                );
            }
        }
    }
    score
}

fn score_window(window: [&Cell; 3], player_cell: &Cell) -> i32 { // 
    if window.iter().any(|&c| c != player_cell && *c != Cell::Empty) {
        return 0;
    }
    match window.iter().filter(|&&c| c == player_cell).count() {
        2 => 3,
        1 => 1,
        _ => 0,
    }
}