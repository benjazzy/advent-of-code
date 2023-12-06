use day_04::part2_cache::process;

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{result}");
}
