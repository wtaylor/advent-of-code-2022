pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u8) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part_1, day01::part_2),
        2 => (day02::part_1, day02::part_2),
        3 => (day03::part_1, day03::part_2),
        4 => (day04::part_1, day04::part_2),
        5 => (day05::part_1, noop),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        },
    };
}
