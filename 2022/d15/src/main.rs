use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let now = std::time::Instant::now();
    // let input = include_str!("../input.txt");
    let mut sb_pairs: Vec<((i32,i32),(i32,i32))> = vec![];
    for line in input.lines(){
        let parts = line.split("=").collect::<Vec<&str>>();
        sb_pairs.push(((parts[1].split(",").nth(0).unwrap().parse::<i32>().unwrap(),
                        parts[2].split(":").nth(0).unwrap().parse::<i32>().unwrap()),
                       (parts[3].split(",").nth(0).unwrap().parse::<i32>().unwrap(),
                       parts[4].parse::<i32>().unwrap())))
    }
    let mut relevant_beacons: Vec<i32> = Vec::new();
    let mut empty_regions:Vec<(i32,i32)> = vec![];
    for ((x1, y1), (x2, y2)) in &sb_pairs {
        let distance_to_relevant = (y1 - 2000000).abs();
        let distance_to_beacon = (x1 - x2).abs() + (y1 - y2).abs();
        let slack = distance_to_beacon - distance_to_relevant;
        if x1-slack <= x1+slack{
            empty_regions.push((x1-slack,x1+slack));
        }
        if y2 == &2000000{
            let mut new = true;
            for beacon in &relevant_beacons{
                if beacon == x2{
                    new = false;
                }
            }
            if new{
                relevant_beacons.push(*x2);
            }
        }
    }
    let mut start = empty_regions[0].0;
    let mut end = empty_regions[0].1;
    for i in 1..empty_regions.len(){
        start = min(start,empty_regions[i].0);
        end = max(end,empty_regions[i].1);
    }
    let part1 = end-start+1-relevant_beacons.len() as i32;
    let mut left_plus_right = 0;
    let mut left_minus_right = 0;
    for i in 0..sb_pairs.len()-1{
        for j in i..sb_pairs.len(){
            let x = sb_pairs[i];
            let y = sb_pairs[j];
            let x_distance = (x.0.0-x.1.0).abs() + (x.0.1-x.1.1).abs();
            let y_distance = (y.0.0-y.1.0).abs() + (y.0.1-y.1.1).abs();
            let between_distance = (x.0.0-y.0.0).abs() + (x.0.1-y.0.1).abs();
            if between_distance-x_distance-y_distance == 2{
                if x.0.0 < y.0.0{
                    if x.0.1 < y.0.1{
                        left_plus_right = x.0.0+x.0.1+x_distance+1;
                    }
                    else{
                        left_minus_right = x.0.0-x.0.1+x_distance+1;
                    }
                }
                else{
                    if x.0.1 < y.0.1{
                        left_minus_right = x.0.0-x.0.1-x_distance-1;
                    }
                    else{
                        left_plus_right = x.0.0+x.0.1-x_distance-1;
                    }
                }
            }
        }
    }
    let part2:i64 = ((left_minus_right + left_plus_right) as i64 * 2000000)+(left_plus_right-left_minus_right) as i64/2;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{}",part1);
    println!("{}",part2);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}