use day_05::part2_rev::process;

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{result}");
}