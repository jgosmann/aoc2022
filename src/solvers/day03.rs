use super::base::AocSolver;

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

impl AocSolver<'_, u64, u64> for Solver {
    fn new(input: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            items: input
                .lines()
                .map(|line| {
                    line.as_bytes()
                        .iter()
                        .copied()
                        .map(Item::new)
                        .collect::<anyhow::Result<Vec<Item>>>()
                })
                .collect::<anyhow::Result<Vec<Vec<Item>>>>()?,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<u64> {
        let mut priority_sum: u64 = 0;
        for elf_items in &self.items {
            let mut seen: u64 = 0;
            for item in elf_items[0..elf_items.len() / 2].iter() {
                seen |= 1 << item.priority();
            }
            for item in elf_items[elf_items.len() / 2..].iter() {
                let priority = item.priority();
                if seen & (1 << priority) != 0 {
                    priority_sum += priority as u64;
                    break;
                }
            }
        }
        Ok(priority_sum)
    }

    fn solve_part2(&self) -> anyhow::Result<Option<u64>> {
        let mut priority_sum: u64 = 0;
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
                        priority_sum += priority as u64;
                        break;
                    }
                }
            }
        }

        Ok(Some(priority_sum))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day03");
        test_example_input::<Solver, _, _>(input, 157, Some(70));
    }
}
