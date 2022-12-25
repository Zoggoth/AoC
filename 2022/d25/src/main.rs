use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut part1: i64 = 0;
    for line in input.lines(){
        let mut current: i64 = 0;
        for x in line.chars(){
            current *= 5;
            match x {
                '2' => {current += 2;}
                '1' => {current += 1;}
                '0' => {current += 0;}
                '-' => {current += -1;}
                '=' => {current += -2;}
                _ => {}
            }
        }
        part1 += current;
    }
    let mut current = "".to_string();
    while part1 != 0{
        match part1 % 5 {
            0 => {current = "0".to_string() + &current}
            1 => {current = "1".to_string() + &current}
            2 => {current = "2".to_string() + &current}
            3 => {current = "=".to_string() + &current}
            4 => {current = "-".to_string() + &current}
            _ => {}
        }
        part1 = (part1 + 2)/5
    }
    println!("{}",current);
}
