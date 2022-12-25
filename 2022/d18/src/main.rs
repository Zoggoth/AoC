use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut now = std::time::Instant::now();
    let cubes: Vec<Vec<i32>> = input.lines().map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect()).collect();
    let mut cube_hash: HashSet<Vec<i32>> = HashSet::new();
    let mut bounds = (cubes[0][0],cubes[0][0],cubes[0][1],cubes[0][1],cubes[0][2],cubes[0][2]);
    for x in &cubes{
        cube_hash.insert(x.to_vec());
        bounds.0 = min(bounds.0,x[0]);
        bounds.2 = min(bounds.2,x[1]);
        bounds.4 = min(bounds.4,x[2]);
        bounds.1 = max(bounds.1,x[0]);
        bounds.3 = max(bounds.3,x[1]);
        bounds.5 = max(bounds.5,x[2]);
    }
    bounds.0 -= 1;
    bounds.2 -= 1;
    bounds.4 -= 1;
    bounds.1 += 1;
    bounds.3 += 1;
    bounds.5 += 1;
    let mut part1 = 0;
    for x in &cube_hash{
        let mut test_vec = x.clone();
        part1 +=6;
        test_vec[0]+=1;
        if cube_hash.contains(&*test_vec){
            part1 -= 1;
        }
        test_vec[0]-=2;
        if cube_hash.contains(&*test_vec){
            part1 -= 1;
        }
        test_vec[0]+=1;
        test_vec[1]+=1;
        if cube_hash.contains(&*test_vec){
            part1 -= 1;
        }
        test_vec[1]-=2;
        if cube_hash.contains(&*test_vec){
            part1 -= 1;
        }
        test_vec[1]+=1;
        test_vec[2]+=1;
        if cube_hash.contains(&*test_vec){
            part1 -= 1;
        }
        test_vec[2]-=2;
        if cube_hash.contains(&*test_vec){
            part1 -= 1;
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:?}",part1);
    now = std::time::Instant::now();
    let mut external: HashSet<Vec<i32>> = HashSet::new();
    let mut todo:Vec<Vec<i32>> = vec![];
    todo.push(vec![bounds.0,bounds.2,bounds.4]);
    let mut part2 = 0;
    while !todo.is_empty(){
        let position = todo.pop().unwrap().clone();
        if !external.contains(&*position){
            let mut new_position = position.clone();
            external.insert(new_position);
            new_position = position.clone();
            new_position[0] += 1;
            if !cube_hash.contains(&*new_position){
                if new_position[0] <= bounds.1{
                    todo.push(new_position);
                }
            }
            else{
                part2 +=1;
            }
            new_position = position.clone();
            new_position[0] -= 1;
            if !cube_hash.contains(&*new_position){
                if new_position[0] >= bounds.0{
                    todo.push(new_position);
                }
            }
            else{
                part2 +=1;
            }
            new_position = position.clone();
            new_position[1] += 1;
            if !cube_hash.contains(&*new_position){
                if new_position[1] <= bounds.3{
                    todo.push(new_position);
                }
            }
            else{
                part2 +=1;
            }
            new_position = position.clone();
            new_position[1] -= 1;
            if !cube_hash.contains(&*new_position){
                if new_position[1] >= bounds.2{
                    todo.push(new_position);
                }
            }
            else{
                part2 +=1;
            }
            new_position = position.clone();
            new_position[2] += 1;
            if !cube_hash.contains(&*new_position){
                if new_position[2] <= bounds.5{
                    todo.push(new_position);
                }
            }
            else{
                part2 +=1;
            }
            new_position = position.clone();
            new_position[2] -= 1;
            if !cube_hash.contains(&*new_position){
                if new_position[2] >= bounds.4{
                    todo.push(new_position);
                }
            }
            else{
                part2 +=1;
            }
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:?}",part2);
}
