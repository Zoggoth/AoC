use std::fs::read_to_string;
use crate::Instruction::{Left, Right, Steps};

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Debug)]
enum Instruction {
    Steps(i32),
    Right,
    Left
}

fn print_map(map: &Vec<Vec<char>>,position:(i32,i32)){
    let mut new_map = map.clone();
    new_map[position.0 as usize][position.1 as usize] = '*';
    for x in new_map{
        for y in x{
            print!("{}",y);
        }
        println!();
    }
}

fn is_oob(map: &Vec<Vec<char>>,position:(i32,i32))->bool{
    if position.0 < 0{
        return true;
    }
    if position.1 < 0{
        return true;
    }
    if position.0 >= map.len() as i32{
        return true;
    }
    else{
        if position.1 >= map[position.0 as usize].len() as i32{
            return true;
        }
        else{
            if map[position.0 as usize][position.1 as usize] == ' '{
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut mapping = true;
    let mut map: Vec<Vec<char>> = vec![];
    let mut instructions: Vec<Instruction> = vec![];
    for line in input.lines(){
        if mapping{
            if line == ""{
                mapping = false;
            }
            else{
                map.push(line.chars().collect());
            }
        }
        else{
            let mut capture = 0;
            for x in line.chars(){
                if x == 'R'{
                    instructions.push(Steps(capture));
                    capture = 0;
                    instructions.push(Right);
                }
                else if x == 'L'{
                    instructions.push(Steps(capture));
                    capture = 0;
                    instructions.push(Left);
                }
                else{
                    capture *= 10;
                    capture += x as i32 - 48;
                }
            }
            instructions.push(Steps(capture));
        }
    }
    let mut position:(i32,i32) = (0,0);
    let mut direction:(i32,i32) = (0,1);
    for x in 0..map[0].len(){
        if map[0][x] == '.'{
            position.1 = x as i32;
            break;
        }
    }
    for x in &instructions{
        match x {
            Steps(mut y) => {
                while y > 0{
                    let mut next_step = (position.0 + direction.0, position.1 + direction.1);
                    if is_oob(&map,next_step){
                        match direction {
                            (1, 0) => { next_step.0 = 0; }
                            (-1, 0) => { next_step.0 = map.len() as i32;}
                            (0, 1) => { next_step.1 = 0;}
                            (0, -1) => { next_step.1 = map[next_step.0 as usize].len() as i32;}
                            (_, _) => {}
                        }
                        while is_oob(&map,next_step) {
                            next_step = (next_step.0 + direction.0, next_step.1 + direction.1);
                        }
                    }
                    if map[next_step.0 as usize][next_step.1 as usize] == '.'{
                        position = next_step;
                    }
                    else{
                        y = 0;
                    }
                    y -= 1;
                }
                // print_map(&map,position);
                // println!();
            }
            Right => {
                direction = (direction.1,-direction.0);
            }
            Left => {
                direction = (-direction.1,direction.0);
            }
        }
    }
    let mut part1 = (position.0+1)*1000 + (position.1+1)*4;
    match direction{
        (0,1) => {part1 += 0;}
        (0,-1) => {part1 += 2;}
        (1,0) => {part1 += 1;}
        (-1,0) => {part1 += 3;}
        _ => {}
    }
    position = (0,0);
    direction = (0,1);
    for x in 0..map[0].len(){
        if map[0][x] == '.'{
            position.1 = x as i32;
            break;
        }
    }
    for x in &instructions{
        println!("{:?}",x);
        match x {
            Steps(mut y) => {
                while y > 0{
                    let mut next_step = (position.0 + direction.0, position.1 + direction.1);
                    let mut new_direction = direction;
                    if is_oob(&map,next_step){
                        match direction {
                            (1, 0) => {
                                match next_step.1 {
                                    0..=49 => {
                                        let from_left = next_step.1;
                                        next_step.0 = 0;
                                        next_step.1 = 100 + from_left;
                                        new_direction = (1,0);
                                    }
                                    50..=99 => {
                                        let from_left = next_step.1 - 50;
                                        next_step.1 = 49;
                                        next_step.0 = 150 + from_left;
                                        new_direction = (0,-1);
                                    }
                                    100..=149 => {
                                        let from_left = next_step.1 - 100;
                                        next_step.1 = 99;
                                        next_step.0 = 50 + from_left;
                                        new_direction = (0,-1);
                                    }
                                    _ => {}
                                }
                            }
                            (-1, 0) => {
                                match next_step.1 {
                                    0..=49 => {
                                        let from_left = next_step.1;
                                        next_step.1 = 50;
                                        next_step.0 = 50 + from_left;
                                        new_direction = (0,1);
                                    }
                                    50..=99 => {
                                        let from_left = next_step.1 - 50;
                                        next_step.1 = 0;
                                        next_step.0 = 150 + from_left;
                                        new_direction = (0,1);
                                    }
                                    100..=149 => {
                                        let from_left = next_step.1 - 100;
                                        next_step.0 = 199;
                                        next_step.1 = from_left;
                                        new_direction = (-1,0);
                                    }
                                    _ => {}
                                }
                            }
                            (0, 1) => {
                                match next_step.0 {
                                    0..=49 => {
                                        let from_top = next_step.0;
                                        next_step.1 = 99;
                                        next_step.0 = 149-from_top;
                                        new_direction = (0,-1);
                                    }
                                    50..=99 => {
                                        let from_top = next_step.0 - 50;
                                        next_step.0 = 49;
                                        next_step.1 = 100 + from_top;
                                        new_direction = (-1,0);
                                    }
                                    100..=149 => {
                                        let from_top = next_step.0 - 100;
                                        next_step.1 = 149;
                                        next_step.0 = 49-from_top;
                                        new_direction = (0,-1);
                                    }
                                    150..=199 => {
                                        let from_top = next_step.0 - 150;
                                        next_step.0 = 149;
                                        next_step.1 = 50 + from_top;
                                        new_direction = (-1,0);
                                    }
                                    _ => {}
                                }
                            }
                            (0, -1) => {
                                match next_step.0 {
                                    0..=49 => {
                                        let from_top = next_step.0;
                                        next_step.1 = 0;
                                        next_step.0 = 149-from_top;
                                        new_direction = (0,1);
                                    }
                                    50..=99 => {
                                        let from_top = next_step.0 - 50;
                                        next_step.0 = 100;
                                        next_step.1 = from_top;
                                        new_direction = (1,0);

                                    }
                                    100..=149 => {
                                        let from_top = next_step.0 - 100;
                                        next_step.1 = 50;
                                        next_step.0 = 49-from_top;
                                        new_direction = (0,1);
                                    }
                                    150..=199 => {
                                        let from_top = next_step.0 - 150;
                                        next_step.0 = 0;
                                        next_step.1 = 50 + from_top;
                                        new_direction = (1,0);
                                    }
                                    _ => {}
                                }
                            }
                            (_, _) => {}
                        }
                    }
                    println!("{:?}",next_step);
                    if map[next_step.0 as usize][next_step.1 as usize] == '.'{
                        position = next_step;
                        direction = new_direction;
                    }
                    else{
                        y = 0;
                    }
                    y -= 1;
                }
                // print_map(&map,position);
                // println!();
                // pause();
            }
            Right => {
                direction = (direction.1,-direction.0);
            }
            Left => {
                direction = (-direction.1,direction.0);
            }
        }
    }
    let mut part2 = (position.0+1)*1000 + (position.1+1)*4;
    match direction{
        (0,1) => {part2 += 0;}
        (0,-1) => {part2 += 2;}
        (1,0) => {part2 += 1;}
        (-1,0) => {part2 += 3;}
        _ => {}
    }
    println!("{:?}",part1);
    println!("{:?}",part2);
}
