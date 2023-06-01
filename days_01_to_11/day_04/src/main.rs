use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

type Pair = [RangeInclusive<i32>; 2];

fn count_contains(pairs: &Vec<Pair>) -> i32 {
    pairs.iter().fold(0, |acc, pair| {
        if pair[0].contains(&pair[1].start()) && pair[0].contains(&pair[1].end())
            || pair[1].contains(&pair[0].start()) && pair[1].contains(&pair[0].end())
        {
            acc + 1
        } else {
            acc
        }
    })
}

fn count_intersects(pairs: &Vec<Pair>) -> i32 {
    pairs.iter().fold(0, |acc, pair| {
        if pair[0].contains(&pair[1].start())
            || pair[0].contains(&pair[1].end())
            || pair[1].contains(&pair[0].start())
            || pair[1].contains(&pair[0].end())
        {
            acc + 1
        } else {
            acc
        }
    })
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let pairs: Vec<Pair> = lines
        .map(|line| {
            let line_str = line.unwrap();
            let range_strs = line_str.split(',');
            let pair: Pair = range_strs
                .map(|range_str| {
                    let mut nums = range_str
                        .split("-")
                        .map(|num_str| num_str.parse::<i32>().unwrap());

                    let start = nums.next().unwrap();
                    let end = nums.next().unwrap();
                    start..=end
                })
                .collect::<Vec<RangeInclusive<i32>>>()
                .try_into()
                .unwrap();
            pair
        })
        .collect();

    let contains_count = count_contains(&pairs);
    let intersects_count = count_intersects(&pairs);

    println!(
        "contains_count: {:?}, intersects_count: {:?}",
        contains_count, intersects_count
    );
}
