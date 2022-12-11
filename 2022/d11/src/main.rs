use std::fs::read_to_string;

#[derive(Debug)]
#[derive(Clone)]
struct Monkey {
    items: Vec<i128>,
    plus: i128,
    times: i128,
    square: bool,
    div_test: i128,
    succeed: i128,
    fail: i128,
    business: i128
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut monkey_list: Vec<Monkey> = vec![];
    let mut modulus = 1;
    let split_text:Vec<&str> = input.split("\r\n\r\n").collect();
    for x in split_text{
        let lines:Vec<&str> = x.split("\r\n").collect();
        let items = &lines[1][18..].split(", ").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<i128>>();
        let operand = &lines[2][25..];
        let operator = &lines[2][23..24];
        let mut square = false;
        let mut plus = 0;
        let mut times = 1;
        if operand == "old"{
            square = true
        }
        else if operator == "+"{
            plus = operand.parse::<i128>().unwrap();
        }
        else {
            times = operand.parse::<i128>().unwrap();
        }
        let div_test = *&lines[3][21..].parse::<i128>().unwrap();
        modulus *= div_test;
        let succeed = *&lines[4][29..].parse::<i128>().unwrap();
        let fail = *&lines[5][30..].parse::<i128>().unwrap();
        let new_monkey = Monkey{
            items: (*items.clone()).to_owned(),
            plus,
            times,
            square,
            div_test,
            succeed,
            fail,
            business: 0
        };
        monkey_list.push(new_monkey);
    }
    let part2_list = monkey_list.clone();
    for _ in 0..20 {
        for x in 0..monkey_list.len(){
            for y in 0..monkey_list[x].items.len(){
                let mut new_y = monkey_list[x].items[y] as i128;
                new_y = new_y + monkey_list[x].plus;
                new_y = new_y * monkey_list[x].times;
                if monkey_list[x].square{
                    new_y = new_y * new_y;
                }
                new_y = new_y /3;
                let succeed = monkey_list[x].succeed as usize;
                let fail = monkey_list[x].fail as usize;
                if new_y % monkey_list[x].div_test == 0{
                    monkey_list[succeed].items.push(new_y);
                }
                else{
                    monkey_list[fail].items.push(new_y);
                }
                monkey_list[x].business += 1;
            }
            monkey_list[x].items = vec![];
        }
    }
    let mut part1: Vec<i128> = vec![];
    for x in monkey_list{
        part1.push(x.business);
    }
    part1.sort();
    println!("{:?}",part1[part1.len()-2]*part1[part1.len()-1]);
    monkey_list = part2_list;
    for _ in 0..10000 {
        for x in 0..monkey_list.len(){
            for y in 0..monkey_list[x].items.len(){
                let mut new_y = monkey_list[x].items[y] as i128;
                new_y = new_y + monkey_list[x].plus;
                new_y = new_y * monkey_list[x].times;
                if monkey_list[x].square{
                    new_y = new_y * new_y;
                }
                new_y = new_y % modulus;
                let succeed = monkey_list[x].succeed as usize;
                let fail = monkey_list[x].fail as usize;
                if new_y % monkey_list[x].div_test == 0{
                    monkey_list[succeed].items.push(new_y);
                }
                else{
                    monkey_list[fail].items.push(new_y);
                }
                monkey_list[x].business += 1;
            }
            monkey_list[x].items = vec![];
        }
    }
    part1= vec![];
    for x in monkey_list{
        part1.push(x.business);
    }
    part1.sort();
    print!("{:?}",part1[part1.len()-2]*part1[part1.len()-1]);
}
