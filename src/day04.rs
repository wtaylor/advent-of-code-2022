use std::ops::{RangeInclusive};

fn parse_input(input: &str) -> Vec<(RangeInclusive<u8>, RangeInclusive<u8>)> {
    input.lines()
        .map(|l| l.split(","))
        .map(|l| l.flat_map(|r| r.split("-").map(|i| i.parse::<u8>().unwrap())))
        .map(|mut p| (p.next().unwrap()..=p.next().unwrap(), p.next().unwrap()..=p.next().unwrap()))
        .collect()
}

fn range_contains_other(range: &RangeInclusive<u8>, other: &RangeInclusive<u8>) -> bool {
    range.start() <= other.start() && range.end() >= other.end()
}

fn range_overlaps_other(range: &RangeInclusive<u8>, other: &RangeInclusive<u8>) -> bool {
    range.start() <= other.start() && range.end() >= other.start()
}

pub fn part_1(input: String) {
    let input = parse_input(&input);

    let containing_pairs = input.iter()
        .filter(|&p|
            range_contains_other(&p.0, &p.1)
            || range_contains_other(&p.1, &p.0))
        .collect::<Vec<&(RangeInclusive<u8>, RangeInclusive<u8>)>>();

    println!("Number of containing pairs: {0}", containing_pairs.len())
}

pub fn part_2(input: String) {
    let input = parse_input(&input);

    let containing_pairs = input.iter()
        .filter(|&p|
            range_overlaps_other(&p.0, &p.1)
                || range_overlaps_other(&p.1, &p.0))
        .collect::<Vec<&(RangeInclusive<u8>, RangeInclusive<u8>)>>();

    println!("Number of overlapping pairs: {0}", containing_pairs.len())
}