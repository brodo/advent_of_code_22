fn main() {
    let input_str = include_str!("../assets/day1.txt");
    let mut curr_sum:u32 = 0;
    let mut elfs = vec![];
    for line in input_str.lines() {
        if line.is_empty() {
            elfs.push(curr_sum);
            curr_sum = 0;
        } else {
            curr_sum += line.parse::<u32>().unwrap();
        }
    }
    elfs.sort();
    elfs.reverse();
    let sums: u32 = elfs.iter().take(3).sum();
    println!("{}",sums)

}
