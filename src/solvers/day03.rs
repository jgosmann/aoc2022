use super::base::{AocAnswer, AocPartSolution, AocSolution, AocSolver};

pub type ItemType = u8;
pub type Priority = u8;

pub struct Item {
    priority: Priority,
}

impl Item {
    pub fn new(item_type: ItemType) -> anyhow::Result<Self> {
        Ok(Self {
            priority: Self::to_priority(item_type),
        })
    }

    pub fn to_priority(item: ItemType) -> Priority {
        ((item - b'A' + 26 + 1) % (b'a' - b'A' + 26)) as Priority
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }
}

pub struct Solver {
    items: Vec<Vec<Item>>,
}

impl AocSolver for Solver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            items: input
                .map(|line| {
                    line.trim()
                        .bytes()
                        .map(Item::new)
                        .collect::<anyhow::Result<Vec<Item>>>()
                })
                .collect::<anyhow::Result<Vec<Vec<Item>>>>()?,
        })
    }

    fn solve(&self) -> anyhow::Result<super::base::AocSolution> {
        Ok(AocSolution {
            part1: AocPartSolution {
                name: "Sum of priorities (part 1):",
                answer: self.solve_part1(),
            },
            part2: Some(AocPartSolution {
                name: "Sum of priorities (part 2):",
                answer: self.solve_part2(),
            }),
        })
    }
}

impl Solver {
    fn solve_part1(&self) -> AocAnswer {
        let mut priority_sum: AocAnswer = 0;
        for elf_items in &self.items {
            let mut seen: u64 = 0;
            for item in elf_items[0..elf_items.len() / 2].iter() {
                seen |= 1 << item.priority();
            }
            for item in elf_items[elf_items.len() / 2..].iter() {
                let priority = item.priority();
                if seen & (1 << priority) != 0 {
                    priority_sum += priority as AocAnswer;
                    break;
                }
            }
        }
        priority_sum
    }

    fn solve_part2(&self) -> AocAnswer {
        let mut priority_sum: AocAnswer = 0;
        let mut elf_iter = self.items.iter();
        while let Some(elf) = elf_iter.next() {
            let mut seen0 = [false; 2 * 26];
            for item in elf {
                seen0[item.priority() as usize - 1] = true;
            }

            let mut seen1 = [false; 2 * 26];
            if let Some(elf) = elf_iter.next() {
                for item in elf {
                    seen1[item.priority() as usize - 1] = seen0[item.priority() as usize - 1];
                }
            }

            if let Some(elf) = elf_iter.next() {
                for item in elf {
                    let priority = item.priority();
                    if seen1[priority as usize - 1] {
                        priority_sum += priority as AocAnswer;
                        break;
                    }
                }
            }
        }

        priority_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = "\
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";
        test_example_input::<Solver>(input, 157, Some(70));
    }
}
