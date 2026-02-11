use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part1 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part1 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        for guess in min..=max {
            if player.ask_if_equal(guess) {
                return guess;
            }
        }
        unreachable!("You think you're so funny choosing 
        number out of the range, huh?") //error message if outside of range ;)
    }
}




            
    
            
