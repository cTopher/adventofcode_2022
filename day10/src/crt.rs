use crate::Cpu;
use std::fmt;

pub struct Crt {
    pixels: Vec<char>,
}

impl Crt {
    pub const fn new() -> Self {
        Self { pixels: Vec::new() }
    }

    pub fn run_on(&mut self, cpu: &mut Cpu) {
        for _ in 0..6 {
            for position in 0..40 {
                if (position - cpu.x).abs() <= 1 {
                    self.pixels.push('#');
                } else {
                    self.pixels.push('.');
                }
                cpu.tick();
            }
        }
    }
}

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.pixels.chunks(40) {
            for pixel in row {
                write!(f, "{pixel}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
