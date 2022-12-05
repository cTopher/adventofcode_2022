use crate::instruction::Move;
use crate::Ship;

pub trait Crane {
    fn do_instructions(ship: &mut Ship, instructions: &[Move]) {
        for &instruction in instructions {
            Self::do_instruction(ship, instruction);
        }
    }
    fn do_instruction(ship: &mut Ship, instruction: Move);
}

pub struct CrateMover9000;
pub struct CrateMover9001;

impl Crane for CrateMover9000 {
    fn do_instruction(ship: &mut Ship, instruction: Move) {
        for _ in 0..instruction.amount {
            let crate_ = ship.take_crate(instruction.from);
            ship.add_crate(instruction.to, crate_);
        }
    }
}

impl Crane for CrateMover9001 {
    fn do_instruction(ship: &mut Ship, instruction: Move) {
        let crates = ship.take_crates(instruction.from, instruction.amount);
        ship.add_crates(instruction.to, crates);
    }
}
