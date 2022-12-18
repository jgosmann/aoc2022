use std::collections::HashSet;

use super::{base::AocSolver, error::InputParseError};
use lazy_static::lazy_static;
use regex::Regex;

type Pos = (i64, i64);

fn manhatten(a: Pos, b: Pos) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn len(&self) -> usize {
        ((self.end - self.start) + 1) as usize
    }
}

#[derive(Clone, Copy, Debug)]
struct Reading {
    pub sensor: Pos,
    pub beacon: Pos,
}

impl TryFrom<&str> for Reading {
    type Error = InputParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
            )
            .unwrap();
        }
        let mut captures = RE.captures_iter(value);
        let matched = captures
            .next()
            .ok_or_else(|| InputParseError::new("invalid format".into()))?;
        Ok(Self {
            sensor: (matched[1].parse()?, matched[2].parse()?),
            beacon: (matched[3].parse()?, matched[4].parse()?),
        })
    }
}

impl Reading {
    fn dist(&self) -> i64 {
        manhatten(self.beacon, self.sensor)
    }
}

pub struct Solver {
    readings: Vec<Reading>,
    beacons: HashSet<Pos>,
}

impl<'a> AocSolver<'a, usize, i64> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let readings: Vec<Reading> = input
            .split('\n')
            .map(Reading::try_from)
            .filter_map(Result::ok)
            .collect();
        let beacons = readings.iter().map(|reading| reading.beacon).collect();
        Ok(Self { readings, beacons })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        self.count_non_beacon_fields_in_row(2_000_000)
    }

    fn solve_part2(&self) -> anyhow::Result<Option<i64>> {
        Ok(Some(self.tuning_frequency(4_000_000)?))
    }
}

impl Solver {
    fn count_beacons_in_interval(&self, interval: Interval, row: i64) -> usize {
        self.beacons
            .iter()
            .filter(|beacon| {
                beacon.1 == row && interval.start <= beacon.0 && beacon.0 <= interval.end
            })
            .count()
    }

    fn count_non_beacon_fields_in_row(&self, row: i64) -> anyhow::Result<usize> {
        let mut intervals: Vec<Interval> = self
            .readings
            .iter()
            .filter_map(|reading| {
                let delta_x = reading.dist() - (reading.sensor.1 - row).abs();
                if delta_x > 0 {
                    Some(Interval::new(
                        reading.sensor.0 - delta_x,
                        reading.sensor.0 + delta_x,
                    ))
                } else {
                    None
                }
            })
            .collect();
        intervals.sort_by_key(|interval| interval.start);

        let mut no_beacon_count: usize =
            intervals[0].len() - self.count_beacons_in_interval(intervals[0], row);
        let mut current_interval: Interval = intervals[0];
        for interval in intervals {
            if interval.end > current_interval.end {
                if interval.start > current_interval.end {
                    no_beacon_count +=
                        interval.len() - self.count_beacons_in_interval(interval, row);
                    current_interval = interval;
                } else {
                    let interval = Interval::new(current_interval.end + 1, interval.end);
                    no_beacon_count +=
                        interval.len() - self.count_beacons_in_interval(interval, row);
                    current_interval.end = interval.end;
                }
            }
        }

        Ok(no_beacon_count)
    }

    fn tuning_frequency(&self, max_coordinate: i64) -> anyhow::Result<i64> {
        let mut positive_slope_intercepts = HashSet::new();
        let mut negative_slope_intercepts = HashSet::new();

        for (b_start_idx, reading_a) in self.readings.iter().enumerate() {
            for reading_b in &self.readings[b_start_idx..] {
                let sensor_dist = manhatten(reading_a.sensor, reading_b.sensor);
                let covered_dist = reading_a.dist() + reading_b.dist();
                let gap = (sensor_dist - covered_dist) / 2;
                if gap == 1 || gap == 2 {
                    let delta_x = reading_b.sensor.0 - reading_a.sensor.0;
                    let delta_y = reading_b.sensor.1 - reading_a.sensor.1;
                    if delta_x == 0 || delta_y == 0 {
                        // skip, special case not accounted for
                    } else {
                        let sign = (delta_x * delta_y).signum();
                        let intercept_sensor = sign * reading_a.sensor.0 + reading_a.sensor.1;
                        for i in 0..gap {
                            let offset = delta_y.signum() * (reading_a.dist() + i + 1);
                            if sign > 0 {
                                positive_slope_intercepts.insert(intercept_sensor + offset);
                            } else {
                                negative_slope_intercepts.insert(intercept_sensor + offset);
                            }
                        }
                    }
                }
            }
        }

        for positive_slope_intercept in &positive_slope_intercepts {
            for negative_slope_intercept in &negative_slope_intercepts {
                let x = (-negative_slope_intercept + positive_slope_intercept) / 2;
                let y = -x + positive_slope_intercept;
                if 0 <= x
                    && x <= max_coordinate
                    && 0 <= y
                    && y <= max_coordinate
                    && self
                        .readings
                        .iter()
                        .all(|reading| Self::check(reading, (x, y)))
                {
                    return Ok(4_000_000 * x + y);
                }
            }
        }

        Err(anyhow::anyhow!("special case not accounted for"))
    }

    fn check(reading: &Reading, pos: Pos) -> bool {
        reading.dist() < manhatten(reading.sensor, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = include_str!("examples/day15");
        let solver = Solver::new(input).unwrap();
        assert_eq!(solver.count_non_beacon_fields_in_row(10).unwrap(), 26);
    }

    #[test]
    fn test_example_part2() {
        let input = include_str!("examples/day15");
        let solver = Solver::new(input).unwrap();
        assert_eq!(solver.tuning_frequency(20).unwrap(), 56000011);
    }
}
