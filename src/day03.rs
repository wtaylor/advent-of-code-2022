use std::slice::Iter;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn item_priority(item: &char) -> u32 {
    if item.is_lowercase() {
        return (*item as u32) - 96
    }

    (*item as u32) - 38
}

fn get_common_chars<'a>(left: &'a str, right: &str) -> &'a str {
    left.chars().filter(|&li| right.contains(li)).collect::<String>().as_str()
}

pub fn part_1(input: String) {
    let input = parse_input(&input);
    let input = input.iter()
        .map(|b| b.split_at(b.len()/2));

    let total_item_priority: u32 = input
        .map(|backpack| get_common_chars(backpack.0, backpack.1).chars().next().unwrap())
        .map(|i| item_priority(&i))
        .sum();

    println!("Total priority of items in both compartments: {0}", total_item_priority);
}

// pub fn part_2(input: String) {
//     let input = parse_input(&input).chunks(3);
//
//     let badge_item_types_per_group = input.map(|g| )
// }