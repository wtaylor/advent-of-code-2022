fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn item_priority(item: &char) -> u32 {
    if item.is_lowercase() {
        return (*item as u32) - 96
    }

    (*item as u32) - 38
}

fn get_common_chars(left: &str, right: &str) -> String {
    left.chars().filter(|&li| right.contains(li)).collect()
}

pub fn part_1(input: String) {
    let input = parse_input(&input);
    let input = input.iter()
        .map(|b| b.split_at(b.len()/2));

    let common_items: Vec<String> = input
        .map(|backpack| get_common_chars(backpack.0, backpack.1)).collect();

    let total_item_priority: u32 =
        common_items.iter().map(|i| item_priority(&i.chars().next().unwrap()))
        .sum();

    println!("Total priority of items in both compartments: {0}", total_item_priority);
}

pub fn part_2(input: String) {
    let input = parse_input(&input);
    let input = input.chunks(3);

    let badge_item_type_per_group: Vec<char> = input.map(|g| get_common_chars(get_common_chars(g[0], g[1]).as_str(), g[2])).map(|b| b.chars().next().unwrap()).collect();

    let total_score: u32 = badge_item_type_per_group.iter().map(|i| item_priority(i)).sum();

    println!("Total score for all badge items: {0}", total_score);
}