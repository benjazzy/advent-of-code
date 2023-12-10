use day_08::part1::process;

fn main() {
    let file = include_str!("../../test_input1.txt");
    let result = process(file);
    println!("{result}");
}
