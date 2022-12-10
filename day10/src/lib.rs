use crate::crt::Crt;
use cpu::Cpu;
use instruction::Instruction;

mod cpu;
mod crt;
mod instruction;

fn parse_program(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.trim().lines().map(|line| line.parse().unwrap())
}

#[must_use]
pub fn part_1(input: &str) -> i32 {
    let mut cpu = Cpu::new();
    cpu.execute_program(parse_program(input));
    let mut signal_strength = 0;
    for i in 0..6 {
        cpu.tick_untill(20 + i * 40);
        signal_strength += cpu.signal_strength();
    }
    signal_strength
}

#[must_use]
pub fn part_2(input: &str) -> String {
    let mut cpu = Cpu::new();
    cpu.execute_program(parse_program(input));
    let mut crt = Crt::new();
    crt.run_on(&mut cpu);
    crt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_simple_example() {
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

    #[test]
    fn part_1_example() {
        assert_eq!(13140, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(13820, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            include_str!("../tests/example_output.txt").replace('\r', ""),
            part_2(EXAMPLE)
        );
    }

    #[test]
    fn part_2_input() {
        assert_eq!(
            include_str!("../tests/final_output.txt").replace('\r', ""),
            part_2(INPUT)
        );
    }
}
