fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .collect::<Vec<&str>>()
        .split(|&s| s.is_empty())
        .map(|elf| elf.iter().map(|&cal_item| cal_item.parse::<i32>().unwrap()).collect()).collect()
}

pub fn part_1(input: String) {
    let input = parse_input(&input);
    let elf_calorie_totals = input.iter().map(|elf| elf.iter().fold(0, |elf_total, elf_calorie_item| elf_total + elf_calorie_item));

    let elf_with_most_calories = elf_calorie_totals.max().unwrap();

    println!("The elf carrying the most calories has {} cal", elf_with_most_calories);
}

pub fn part_2(input: String) {
    let input = parse_input(&input);
    let mut elf_calorie_totals_ordered = input.iter().map(|elf| elf.iter().fold(0, |elf_total, elf_calorie_item| elf_total + elf_calorie_item)).collect::<Vec<i32>>();
    elf_calorie_totals_ordered.sort_unstable();
    elf_calorie_totals_ordered.reverse();

    let top_3_elves_summed: i32 = elf_calorie_totals_ordered.iter().take(3).sum();

    println!("Top 3 elves carrying the most calories have: {}", top_3_elves_summed)
}
