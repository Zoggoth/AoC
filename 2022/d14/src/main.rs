use std::collections::HashSet;
use std::fs::read_to_string;
use std::cmp::min;
use std::cmp::max;

fn print_filled(filled: &HashSet<(i32,i32)>, initial_filled: &HashSet<(i32, i32)>){
    let mut max_x = 0;
    let mut min_y = 500;
    let mut max_y = 500;
    let filled2 = filled.clone();
    for x in filled2{
        max_x = max(max_x,x.1);
        min_y = min(min_y,x.0);
        max_y = max(max_y,x.0);
    }
    for x in 0..max_x+1{
        for y in min_y..max_y+1{
            if filled.contains(&(y,x)){
                if initial_filled.contains(&(y,x)){
                    print!("#");
                }
                else{
                    print!("O");
                }
            }
            else{
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut filled: HashSet<(i32,i32)> = HashSet::new();
    let lines:Vec<Vec<Vec<i32>>> = input.lines().map(|x| x.split(" -> ")
        .map(|x| x.split(",")
            .map(|x| x.parse::<i32>().unwrap())
                .collect()).collect()).collect();
    for line in lines{
        for i in 1..line.len(){
            let first = &line[i-1];
            let second = &line[i];
            if first[0] == second[0]{
                for j in min(first[1], second[1])..max(first[1], second[1])+1{
                    filled.insert(((first[0]),j));
                }
            }
            else{
                for j in min(first[0], second[0])..max(first[0], second[0])+1{
                    filled.insert((j,(first[1])));
                }
            }
        }
    }
    let mut max_x = 0;
    let mut initial_filled = filled.clone();
    for x in &filled{
        max_x = max(max_x,x.1);
    }
    let mut todo:Vec<(i32,i32)> = vec![];
    todo.push((500,0));
    while !todo.is_empty(){
        let current = todo.last().unwrap();
        if filled.contains(&(current.0, current.1+1)){
            if filled.contains(&(current.0-1, current.1+1)){
                if filled.contains(&(current.0+1, current.1+1)){
                    filled.insert(*current);
                    todo.pop();
                }
                else{
                    todo.push((current.0+1, current.1+1));
                }
            }
            else{
                todo.push((current.0-1, current.1+1));
            }
        }
        else{
            if current.1 > max_x{
                break;
            }
            todo.push((current.0, current.1+1));
        }
    }
    println!("{:?}",filled.len()-initial_filled.len());
    for i in 500-max_x-2..500+max_x+3{
        filled.insert((i,max_x+2));
        initial_filled.insert((i,max_x+2));
    }
    while !todo.is_empty(){
        let current = todo.last().unwrap();
        if filled.contains(&(current.0, current.1+1)){
            if filled.contains(&(current.0-1, current.1+1)){
                if filled.contains(&(current.0+1, current.1+1)){
                    filled.insert(*current);
                    todo.pop();
                }
                else{
                    todo.push((current.0+1, current.1+1));
                }
            }
            else{
                todo.push((current.0-1, current.1+1));
            }
        }
        else{
            todo.push((current.0, current.1+1));
        }
    }
    println!("{:?}",filled.len()-initial_filled.len());
    // print_filled(&filled,&initial_filled);
}
