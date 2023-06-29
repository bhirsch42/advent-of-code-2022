use std::ops::Range;

use rayon::{
    prelude::{IntoParallelIterator, ParallelIterator},
    vec,
};
use regex::Regex;

use crate::utils::read_input_lines;

const INPUT_PATTERN: &str =
    "Sensor at x=(?P<sensor_x>-?[0-9]+), y=(?P<sensor_y>-?[0-9]+): closest beacon is at x=(?P<beacon_x>-?[0-9]+), y=(?P<beacon_y>-?[0-9]+)";
//   Sensor at x=24822222222222223411, y=39222222222222202983: closest beacon is at x=22895722222222222229, y=36322223423423423785

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point::new(value.0, value.1)
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn range(&self) -> i32 {
        self.location.dist(&self.closest_beacon)
    }

    fn coverage_at_y(&self, y: &i32) -> Option<Range<i32>> {
        let y_dist = (self.location.y - y).abs();
        let x_dist = self.range() - y_dist;

        if x_dist < 0 {
            return None;
        }

        let start = self.location.x - x_dist;
        let end = self.location.x + x_dist;

        Some(start..end)
    }
}

fn load_input(file_name: &str) -> Vec<Sensor> {
    let re = Regex::new(INPUT_PATTERN).unwrap();
    let lines = read_input_lines(file_name);

    lines
        .iter()
        .map(|line| -> Sensor {
            let capture = re.captures(line).unwrap();

            Sensor {
                location: Point {
                    x: capture.name("sensor_x").unwrap().as_str().parse().unwrap(),
                    y: capture.name("sensor_y").unwrap().as_str().parse().unwrap(),
                },
                closest_beacon: Point {
                    x: capture.name("beacon_x").unwrap().as_str().parse().unwrap(),
                    y: capture.name("beacon_y").unwrap().as_str().parse().unwrap(),
                },
            }
        })
        .collect()
}

#[derive(Debug, Default)]
struct RangeSet {
    ranges: Vec<Range<i32>>,
}

impl RangeSet {
    fn insert(&mut self, range: Range<i32>) {
        if range.end < range.start {
            panic!("start must be less than or equal to end")
        }

        let mut overlapping_ranges: Vec<&Range<i32>> = self
            .ranges
            .iter()
            .filter(|other| range.start - 1 <= other.end && other.start <= range.end + 1)
            .collect();

        overlapping_ranges.push(&range);

        let min = overlapping_ranges.iter().map(|r| r.start).min();
        let max = overlapping_ranges.iter().map(|r| r.end).max();

        // Remove overlapping ranges
        self.ranges
            .retain(|other| range.start - 1 > other.end || other.start > range.end + 1);

        if let (Some(min), Some(max)) = (min, max) {
            self.ranges.push(min..max);
            self.ranges.sort_by(|a, b| a.start.cmp(&b.start))
        }
    }

    fn len(&self) -> usize {
        self.ranges.iter().map(|r| r.len()).sum()
    }

    fn from_sensors(sensors: &Vec<Sensor>, y: &i32) -> Self {
        let mut range_set = Self::default();

        sensors.iter().for_each(|sensor| {
            if let Some(range) = sensor.coverage_at_y(y) {
                range_set.insert(range);
            }
        });

        range_set
    }

    fn first_gap(&self, max: &i32) -> Option<i32> {
        self.ranges
            .get(0)
            .map(|range| range.end + 1)
            .filter(|value| value < max)
    }
}

fn cannot_contain_beacon_count(sensors: &Vec<Sensor>, y: &i32) -> i32 {
    RangeSet::from_sensors(sensors, y).len() as i32
}

fn first_gap(sensors: &Vec<Sensor>, max: &i32) -> Option<Point> {
    (0..*max).into_par_iter().find_map_any(|y| {
        if let Some(x) = RangeSet::from_sensors(sensors, &y).first_gap(max) {
            return Some(Point::new(x, y));
        }

        None
    })
}

#[cfg(test)]

mod tests {
    use super::*;
    use anyhow::{Ok, Result};

    #[test]
    fn example_part_1() -> Result<()> {
        let sensors = load_input("day_15_example");
        let result: i32 = cannot_contain_beacon_count(&sensors, &10);
        assert_eq!(result, 26);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        let sensors = load_input("day_15");
        let result: i32 = cannot_contain_beacon_count(&sensors, &2000000);
        assert_eq!(result, 5100463);
        Ok(())
    }

    #[test]
    fn example_part_2() -> Result<()> {
        let sensors = load_input("day_15_example");
        let point = first_gap(&sensors, &20).unwrap();
        println!("{point:#?}");
        let result = point.x * 4000000 + point.y;
        assert_eq!(result, 56000011);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let sensors = load_input("day_15");
        let point = first_gap(&sensors, &4000000).unwrap();
        let result = point.x as i64 * 4000000 + point.y as i64;
        assert_eq!(result, 11557863040754);
        Ok(())
    }
}
