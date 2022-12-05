use std::fs::read_to_string;

fn main() {
    let input =read_to_string("input.txt").expect("No File");
    let mut sum = 0;
    let mut list: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line == ""{
            list.push(sum);
            sum = 0;
        }
        else{
            sum += line.parse::<i32>().unwrap();
        }
    }
    list.push(sum);
    list.sort();
    list.reverse();
    println!("{}",list[0]);
    println!("{}",list[0]+list[1]+list[2]);
}
