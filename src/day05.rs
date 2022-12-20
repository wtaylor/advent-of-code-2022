

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(u16, u8, u8)>) {
    let lines: Vec<&str> = input.lines().collect();

    let mut line_groups = lines.split(|l| l.is_empty());
    let starting_input_lines = line_groups.next();
    let instructions_input_lines = line_groups.next();


    // .map(|l| l.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();


}

pub fn part_1(input: String) {
    let input = parse_input(&input);

    // let elf_calorie_totals = input.iter().map(|elf| elf.iter().fold(0, |elf_total, elf_calorie_item| elf_total + elf_calorie_item));
    //
    // let elf_with_most_calories = elf_calorie_totals.max().unwrap();
    //
    // println!("The elf carrying the most calories has {} cal", elf_with_most_calories);
}
