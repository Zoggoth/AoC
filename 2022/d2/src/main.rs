fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No File");
    let mut score = 0;
    for line in input.lines(){
        let outcome = line.chars().nth(2).unwrap() as i32 - line.chars().nth(0).unwrap() as i32;
        score += ((outcome-1)%3)*3;
        score += ((line.chars().nth(2).unwrap() as i32-1) % 3) + 1;
    }
    println!("{}",score);
    score = 0;
    for line in input.lines(){
        let yours = line.chars().nth(2).unwrap() as i32 + line.chars().nth(0).unwrap() as i32;
        score += (line.chars().nth(2).unwrap() as i32-1) % 3 * 3;
        score += (yours - 1) % 3 + 1;
    }
    println!("{}",score);
}
