use super::{base::AocSolver, error::InputParseError};

enum Instruction {
    Noop,
    AddX(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = InputParseError;

    fn try_from(instruction: &str) -> Result<Self, Self::Error> {
        let mut args_iter = instruction.split(' ');
        let value = args_iter
            .next()
            .ok_or_else(|| InputParseError::new("no instruction given".into()))?;
        match value {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::AddX(
                args_iter
                    .next()
                    .ok_or_else(|| InputParseError::new("missing argument".into()))?
                    .parse()?,
            )),
            _ => Err(InputParseError::new("not a valid instruction".into())),
        }
    }
}

impl Instruction {
    fn cycle_count(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

struct Cpu {
    cycle: usize,
    register_x: i64,
    accumulated_signal_strength: i64,
    screen_buffer: [[char; 40]; 6],
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycle: 0,
            register_x: 1,
            accumulated_signal_strength: 0,
            screen_buffer: [[' '; 40]; 6],
        }
    }

    fn accumulated_signal_strength(&self) -> i64 {
        self.accumulated_signal_strength
    }

    fn screen_row(&self, row: usize) -> &[char] {
        &self.screen_buffer[row]
    }

    fn process(&mut self, instruction: Instruction) {
        for _ in 0..instruction.cycle_count() {
            if ((self.cycle % 40) as i64 - self.register_x).abs() <= 1 {
                self.screen_buffer[self.cycle / 40][self.cycle % 40] = '█';
            }
            self.cycle += 1;
            if (self.cycle + 20) % 40 == 0 {
                self.accumulated_signal_strength += self.cycle as i64 * self.register_x;
            }
        }
        match instruction {
            Instruction::Noop => {}
            Instruction::AddX(value) => {
                self.register_x += value;
            }
        }
    }
}

pub struct Solver {
    cpu: Cpu,
}

impl AocSolver<i64, String> for Solver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut cpu = Cpu::new();
        for line in input {
            let instruction = Instruction::try_from(line.as_str())?;
            cpu.process(instruction);
        }
        Ok(Self { cpu })
    }

    fn solve_part1(&self) -> anyhow::Result<i64> {
        Ok(self.cpu.accumulated_signal_strength())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<String>> {
        let lines: Vec<String> = (0..6)
            .map(|i| self.cpu.screen_row(i).iter().collect::<String>())
            .collect();
        Ok(Some(lines.join("\n")))
    }
}
