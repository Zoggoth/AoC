use std::collections::HashMap;
use std::fs::read_to_string;
use crate::Job::{Input, Operation};
use crate::Op::{Divide, Minus, Plus, Times};

#[derive(Debug)]
#[derive(Clone)]
enum Op {
    Plus,
    Minus,
    Times,
    Divide
}

#[derive(Debug)]
#[derive(Clone)]
enum Job{
    Input(i128),
    Operation(String,Op,String)
}

#[derive(Debug)]
#[derive(Clone)]
struct Monkey {
    name: String,
    job: Job
}

fn gcd (mut x:i128,mut y:i128)->i128 {
    if x < y{ return gcd(y, x)}
    while y != 0 { (x, y) = (y, x % y)}
    return x.abs();
}


fn evaluate(root: String, monkey_list: &Vec<Monkey>,monkey_map: &HashMap<String,usize>)->i128{
    return match &monkey_list[monkey_map[&root]].job {
        Input(x) => { *x }
        Operation(left, Plus, right) => { evaluate(left.to_string(), monkey_list, monkey_map) + evaluate(right.to_string(), monkey_list, monkey_map) }
        Operation(left, Minus, right) => { evaluate(left.to_string(), monkey_list, monkey_map) - evaluate(right.to_string(), monkey_list, monkey_map) }
        Operation(left, Times, right) => { evaluate(left.to_string(), monkey_list, monkey_map) * evaluate(right.to_string(), monkey_list, monkey_map) }
        Operation(left, Divide, right) => { evaluate(left.to_string(), monkey_list, monkey_map) / evaluate(right.to_string(), monkey_list, monkey_map) }
    }
}

fn function_print(root: String, monkey_list: &Vec<Monkey>,monkey_map: &HashMap<String,usize>)->String{
    if root == "humn"{ return "humn".to_string();}
    return match &monkey_list[monkey_map[&root]].job {
        Input(x) => { x.to_string() }
        Operation(left, Plus, right) => { "(".to_owned() + &function_print(left.to_string(), monkey_list, monkey_map) + "+" + &function_print(right.to_string(), monkey_list, monkey_map) + ")" }
        Operation(left, Minus, right) => { "(".to_owned() + &function_print(left.to_string(), monkey_list, monkey_map) + "-" + &function_print(right.to_string(), monkey_list, monkey_map) + ")" }
        Operation(left, Times, right) => { "(".to_owned() + &function_print(left.to_string(), monkey_list, monkey_map) + "*" + &function_print(right.to_string(), monkey_list, monkey_map) + ")" }
        Operation(left, Divide, right) => { "(".to_owned() + &function_print(left.to_string(), monkey_list, monkey_map) + "/" + &function_print(right.to_string(), monkey_list, monkey_map) + ")" }
    }
}

fn clever_evaluate(root: String, monkey_list: &Vec<Monkey>,monkey_map: &HashMap<String,usize>)->(i128,i128,i128,i128){
    if root == "humn"{
        return (0,1,1,1);
    }
    match &monkey_list[monkey_map[&root]].job {
        Input(x) => { return (*x,1,0,1) }
        Operation(left, op, right) => {
            let mut left_eval = clever_evaluate(left.to_string(),monkey_list,monkey_map);
            let mut right_eval = clever_evaluate(right.to_string(),monkey_list,monkey_map);
            let left_denom_0 = left_eval.1;
            let left_denom_1 = left_eval.3;
            let right_denom_0 = right_eval.1;
            let right_denom_1 = right_eval.3;
            match op {
                Plus => {
                    left_eval.0 *= right_denom_0;
                    left_eval.1 *= right_denom_0;
                    right_eval.0 *= left_denom_0;
                    right_eval.1 *= left_denom_0;
                    left_eval.2 *= right_denom_1;
                    left_eval.3 *= right_denom_1;
                    right_eval.2 *= left_denom_1;
                    right_eval.3 *= left_denom_1;
                    let mut output_eval = (left_eval.0+right_eval.0,left_eval.1,left_eval.2+right_eval.2,left_eval.3);
                    let gcd_0 = gcd(output_eval.0, output_eval.1);
                    let gcd_1 = gcd(output_eval.2,output_eval.3);
                    output_eval.0 /= gcd_0;
                    output_eval.1 /= gcd_0;
                    output_eval.2 /= gcd_1;
                    output_eval.3 /= gcd_1;
                    return output_eval;
                }
                Minus => {
                    left_eval.0 *= right_denom_0;
                    left_eval.1 *= right_denom_0;
                    right_eval.0 *= left_denom_0;
                    right_eval.1 *= left_denom_0;
                    left_eval.2 *= right_denom_1;
                    left_eval.3 *= right_denom_1;
                    right_eval.2 *= left_denom_1;
                    right_eval.3 *= left_denom_1;
                    let mut output_eval = (left_eval.0-right_eval.0,left_eval.1,left_eval.2-right_eval.2,left_eval.3);
                    let gcd_0 = gcd(output_eval.0, output_eval.1);
                    let gcd_1 = gcd(output_eval.2,output_eval.3);
                    output_eval.0 /= gcd_0;
                    output_eval.1 /= gcd_0;
                    output_eval.2 /= gcd_1;
                    output_eval.3 /= gcd_1;
                    return output_eval;
                }
                Times => {
                    let mut output_eval = (left_eval.0*right_eval.0,left_eval.1*right_eval.1,0,0);
                    if left_eval.2 == 0{
                        output_eval.2 = right_eval.2*left_eval.0;
                        output_eval.3 = right_eval.3*left_eval.1;
                    }
                    else{
                        output_eval.2 = left_eval.2*right_eval.0;
                        output_eval.3 = left_eval.3*right_eval.1;
                    }
                    let gcd_0 = gcd(output_eval.0, output_eval.1);
                    let gcd_1 = gcd(output_eval.2,output_eval.3);
                    output_eval.0 /= gcd_0;
                    output_eval.1 /= gcd_0;
                    output_eval.2 /= gcd_1;
                    output_eval.3 /= gcd_1;
                    return output_eval;
                }
                Divide => {
                    if right_eval.2 != 0{
                        println!("Uh Oh!");
                        return (0,0,0,0);
                    }
                    let mut output_eval = (left_eval.0*right_eval.1,left_eval.1*right_eval.0,left_eval.2*right_eval.1,left_eval.3*right_eval.0);
                    let gcd_0 = gcd(output_eval.0, output_eval.1);
                    let gcd_1 = gcd(output_eval.2,output_eval.3);
                    output_eval.0 /= gcd_0;
                    output_eval.1 /= gcd_0;
                    output_eval.2 /= gcd_1;
                    output_eval.3 /= gcd_1;
                    return output_eval;
                }
            }
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut now = std::time::Instant::now();
    let mut monkey_list: Vec<Monkey> = vec![];
    let mut monkey_map: HashMap<String,usize> = HashMap::new();
    for line in input.lines(){
        let name_split:Vec<&str> = line.split(": ").collect();
        let new_monkey = Monkey { name: name_split[0].parse().unwrap(),
            job: if name_split[1].chars().nth(0).unwrap().is_digit(10) {
                Input(name_split[1].parse::<i128>().unwrap())
            }
            else{
                match &name_split[1][5..6] {
                    "+" => {Operation(name_split[1][0..4].to_string(),Plus,name_split[1][7..11].to_string())}
                    "-" => {Operation(name_split[1][0..4].to_string(),Minus,name_split[1][7..11].to_string())}
                    "*" => {Operation(name_split[1][0..4].to_string(),Times,name_split[1][7..11].to_string())}
                    "/" => {Operation(name_split[1][0..4].to_string(),Divide,name_split[1][7..11].to_string())}
                    _ => {Input(0)}
                }
            }
        };
        monkey_map.insert(new_monkey.name.clone(), monkey_list.len());
        monkey_list.push(new_monkey);
    }
    let (left_name,op, right_name) = if let Operation(left_name,op,right_name) = &monkey_list[monkey_map["root"]].job { (left_name, op, right_name) } else { todo!() };
    let left = clever_evaluate(left_name.to_string(), &monkey_list, &monkey_map);
    let right = clever_evaluate(right_name.to_string(), &monkey_list, &monkey_map);
    let &humn = if let Input(humn) = &monkey_list[monkey_map["humn"]].job { humn } else { todo!() };
    match op{
        Plus => {
            let plus = (left.0*right.1+left.1*right.0,left.1*right.1,left.2*right.3+left.3*right.2,left.3*right.3);
            let minus = (left.0*right.1-left.1*right.0,left.1*right.1,left.2*right.3-left.3*right.2,left.3*right.3);
            let part1 = ((humn*plus.2*plus.1)+(plus.0*plus.3))/plus.1/plus.3;
            let part2 = -minus.0*minus.3/minus.1/minus.2;
            let elapsed = now.elapsed();
            println!("{:.2?}",elapsed);
            println!("{}", part1);
            println!("{}", part2);
        }
        Minus => {
            let minus = (left.0*right.1-left.1*right.0,left.1*right.1,left.2*right.3-left.3*right.2,left.3*right.3);
            let part1 = ((humn*minus.2*minus.1)+(minus.0*minus.3))/minus.1/minus.3;
            let part2 = -minus.0*minus.3/minus.1/minus.2;
            let elapsed = now.elapsed();
            println!("{:.2?}",elapsed);
            println!("{}", part1);
            println!("{}", part2);
        }
        Times => {
            let mut times = (left.0*right.0,left.1*right.1,0,0);
            if left.2 == 0{
                times.2 = right.2*left.0;
                times.3 = right.3*left.1;
            }
            else{
                times.2 = left.2*right.0;
                times.3 = left.3*right.1;
            }
            let minus = (left.0*right.1-left.1*right.0,left.1*right.1,left.2*right.3-left.3*right.2,left.3*right.3);
            let part1 = ((humn* times.2* times.1)+(times.0* times.3))/ times.1/ times.3;
            let part2 = -minus.0*minus.3/minus.1/minus.2;
            let elapsed = now.elapsed();
            println!("{:.2?}",elapsed);
            println!("{}", part1);
            println!("{}", part2);
        }
        Divide => {
            let divide = (left.0*right.1,left.1*right.0,left.2*right.1,left.3*right.0);
            let minus = (left.0*right.1-left.1*right.0,left.1*right.1,left.2*right.3-left.3*right.2,left.3*right.3);
            let part1 = ((humn*divide.2*divide.1)+(divide.0*divide.3))/divide.1/divide.3;
            let part2 = -minus.0*minus.3/minus.1/minus.2;
            let elapsed = now.elapsed();
            println!("{:.2?}",elapsed);
            println!("{}", part1);
            println!("{}", part2);
        }
    }
}
