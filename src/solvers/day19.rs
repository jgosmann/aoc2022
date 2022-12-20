use std::collections::HashMap;

use super::{base::AocSolver, day01::TopK, error::InputParseError};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: usize,
    cost_robot_ore: Cost,
    cost_robot_clay: Cost,
    cost_robot_obsidian: Cost,
    cost_robot_geode: Cost,
}

impl TryFrom<&str> for Blueprint {
    type Error = InputParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        }
        let mut captures = RE.captures_iter(value);
        let matched = captures
            .next()
            .ok_or_else(|| InputParseError::new("invalid format".into()))?;
        Ok(Self {
            id: matched[1].parse()?,
            cost_robot_ore: Cost {
                ore: matched[2].parse()?,
                clay: 0,
                obsidian: 0,
            },
            cost_robot_clay: Cost {
                ore: matched[3].parse()?,
                clay: 0,
                obsidian: 0,
            },
            cost_robot_obsidian: Cost {
                ore: matched[4].parse()?,
                clay: matched[5].parse()?,
                obsidian: 0,
            },
            cost_robot_geode: Cost {
                ore: matched[6].parse()?,
                clay: 0,
                obsidian: matched[7].parse()?,
            },
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Resources {
    fn cover(&self, cost: &Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn pay(&self, cost: &Cost) -> Self {
        Self {
            ore: self.ore - cost.ore,
            clay: self.clay - cost.clay,
            obsidian: self.obsidian - cost.obsidian,
        }
    }

    fn produce(&self, producers: &RobotCounts) -> Self {
        Self {
            ore: self.ore + producers.ore,
            clay: self.clay + producers.clay,
            obsidian: self.obsidian + producers.obsidian,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct RobotCounts {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

struct GeodeMaximizer<'a> {
    blueprint: &'a Blueprint,
    cache: HashMap<(usize, Resources, RobotCounts), usize>,
}

impl<'a> GeodeMaximizer<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            cache: HashMap::new(),
        }
    }

    pub fn maximize(&mut self, minutes: usize) -> usize {
        self._maximize(
            minutes,
            Resources::default(),
            RobotCounts {
                ore: 1,
                clay: 0,
                obsidian: 0,
            },
        )
    }

    fn _maximize(
        &mut self,
        remaining_minutes: usize,
        resources: Resources,
        count_robots: RobotCounts,
    ) -> usize {
        if remaining_minutes == 0 {
            return 0;
        }
        let cache_key = (remaining_minutes, resources, count_robots);
        if let Some(result) = self.cache.get(&cache_key) {
            return *result;
        }

        let mut best = TopK::<usize, 1>::new();

        if resources.cover(&self.blueprint.cost_robot_geode) {
            best.push(
                self._maximize(
                    remaining_minutes - 1,
                    resources
                        .pay(&self.blueprint.cost_robot_geode)
                        .produce(&count_robots),
                    RobotCounts {
                        ore: count_robots.ore,
                        clay: count_robots.clay,
                        obsidian: count_robots.obsidian,
                    },
                ) + remaining_minutes
                    - 1,
            )
        } else if resources.cover(&self.blueprint.cost_robot_obsidian) {
            best.push(
                self._maximize(
                    remaining_minutes - 1,
                    resources
                        .pay(&self.blueprint.cost_robot_obsidian)
                        .produce(&count_robots),
                    RobotCounts {
                        ore: count_robots.ore,
                        clay: count_robots.clay,
                        obsidian: count_robots.obsidian + 1,
                    },
                ),
            )
        } else {
            if resources.cover(&self.blueprint.cost_robot_ore) {
                best.push(
                    self._maximize(
                        remaining_minutes - 1,
                        resources
                            .pay(&self.blueprint.cost_robot_ore)
                            .produce(&count_robots),
                        RobotCounts {
                            ore: count_robots.ore + 1,
                            clay: count_robots.clay,
                            obsidian: count_robots.obsidian,
                        },
                    ),
                )
            }
            if resources.cover(&self.blueprint.cost_robot_clay) {
                best.push(
                    self._maximize(
                        remaining_minutes - 1,
                        resources
                            .pay(&self.blueprint.cost_robot_clay)
                            .produce(&count_robots),
                        RobotCounts {
                            ore: count_robots.ore,
                            clay: count_robots.clay + 1,
                            obsidian: count_robots.obsidian,
                        },
                    ),
                )
            }
        }
        best.push(self._maximize(
            remaining_minutes - 1,
            resources.produce(&count_robots),
            count_robots,
        ));

        let result = best.peek().copied().unwrap_or_default();
        self.cache.insert(cache_key, result);
        result
    }
}

pub struct Solver {
    blueprints: Vec<Blueprint>,
}

impl<'a> AocSolver<'a, usize, usize> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let blueprints: Vec<Blueprint> = input
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(Blueprint::try_from)
            .collect::<Result<Vec<Blueprint>, InputParseError>>()?;
        Ok(Self { blueprints })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        Ok(self
            .blueprints
            .iter()
            .map(|blueprint| {
                let mut maximizer = GeodeMaximizer::new(blueprint);
                blueprint.id * maximizer.maximize(24)
            })
            .sum())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        Ok(Some(
            self.blueprints
                .iter()
                .take(3)
                .map(|blueprint| {
                    let mut maximizer = GeodeMaximizer::new(blueprint);
                    maximizer.maximize(32)
                })
                .product(),
        ))
    }
}

#[cfg(test)]
mod tests {
    // too slow to run by default
    // use super::*;

    // #[test]
    // fn test_example() {
    //     let input = include_str!("examples/day19");
    //     let solver = Solver::new(input).unwrap();
    //     assert_eq!(solver.solve_part1().unwrap(), 33);
    // }
}
