use crate::instruction::Instruction;
use std::collections::VecDeque;

pub struct Cpu {
    cycle: i32,
    pub x: i32,
    instructions: VecDeque<Instruction>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            cycle: 1,
            x: 1,
            instructions: VecDeque::new(),
        }
    }

    pub const fn signal_strength(&self) -> i32 {
        self.x * self.cycle
    }

    pub fn execute_program<I: IntoIterator<Item = Instruction>>(&mut self, program: I) {
        for instruction in program {
            if matches!(instruction, Instruction::AddX(_)) {
                self.instructions.push_back(Instruction::Noop);
            }
            self.instructions.push_back(instruction);
        }
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if let Some(Instruction::AddX(value)) = self.instructions.pop_front() {
            self.x += value;
        }
    }

    pub fn tick_untill(&mut self, cycle: i32) {
        while self.cycle < cycle {
            self.tick();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_program() {
        let mut cpu = Cpu::new();
        cpu.execute_program([
            Instruction::Noop,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ]);
        cpu.tick();
        assert_eq!(1, cpu.x);
        cpu.tick();
        assert_eq!(1, cpu.x);
        cpu.tick();
        assert_eq!(4, cpu.x);
        cpu.tick();
        assert_eq!(4, cpu.x);
        cpu.tick();
        assert_eq!(-1, cpu.x);
    }
}
