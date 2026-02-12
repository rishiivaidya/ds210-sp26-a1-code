use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mid = min + (max - min) / 2;

        match player.ask_to_compare(mid) {
            0 => mid,
            -1 => Self::guess_the_number(player, min, mid - 1),
            1 => Self::guess_the_number(player, mid + 1, max),
            _ => unreachable!("You done goofed."),
        }
    }
}
