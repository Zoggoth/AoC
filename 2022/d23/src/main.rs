use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn print_grid(elves:&HashSet<(i32,i32)>){
    let mut minmaxs = (0i32,0i32,0i32,0i32);
    for i in elves{
        minmaxs.0 = min(minmaxs.0, i.0);
        minmaxs.1 = max(minmaxs.1, i.0);
        minmaxs.2 = min(minmaxs.2, i.1);
        minmaxs.3 = max(minmaxs.3, i.1);
    }
    for x in minmaxs.0..minmaxs.1+1{
        for y in minmaxs.2..minmaxs.3+1{
            if elves.contains(&(x, y)){
                print!("#");
            }
            else{
                print!(".");
            }
        }
        println!();
    }
    println!("{:?}",(minmaxs.1-minmaxs.0+1)*(minmaxs.3-minmaxs.2+1)-elves.len() as i32)
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let now = std::time::Instant::now();
    let mut xpos = 0;
    let mut elves:HashSet<(i32,i32)> = HashSet::new();
    for line in input.lines(){
        let mut ypos = 0;
        for position in line.chars(){
            if position == '#'{
                elves.insert((xpos, ypos));
            }
            ypos += 1;
        }
        xpos += 1;
    }
    let mut time = 0;
    for _ in 0..10 {
        let mut proposal: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut proposal_collisions: HashMap<(i32, i32), bool> = HashMap::new();
        for (x, y) in &elves {
            let mut udlr = (true, true, true, true);
            if elves.contains(&(x - 1, y - 1)) {
                udlr.0 = false;
                udlr.2 = false;
            }
            if elves.contains(&(x - 1, *y)) {
                udlr.0 = false;
            }
            if elves.contains(&(x - 1, y + 1)) {
                udlr.0 = false;
                udlr.3 = false;
            }
            if elves.contains(&(*x, y + 1)) {
                udlr.3 = false;
            }
            if elves.contains(&(x + 1, y + 1)) {
                udlr.1 = false;
                udlr.3 = false;
            }
            if elves.contains(&(x + 1, *y)) {
                udlr.1 = false;
            }
            if elves.contains(&(x + 1, y - 1)) {
                udlr.1 = false;
                udlr.2 = false;
            }
            if elves.contains(&(*x, y - 1)) {
                udlr.2 = false;
            }
            if udlr == (true, true, true, true) {
                continue;
            } else {
                for i in time..time + 4 {
                    match i % 4 {
                        0 => {
                            if udlr.0 {
                                proposal.insert((*x, *y), (x - 1, *y));
                                if proposal_collisions.contains_key(&(x - 1, *y)) {
                                    proposal_collisions.insert((x - 1, *y), true);
                                } else {
                                    proposal_collisions.insert((x - 1, *y), false);
                                }
                                break;
                            }
                        }
                        1 => {
                            if udlr.1 {
                                proposal.insert((*x, *y), (x + 1, *y));
                                if proposal_collisions.contains_key(&(x + 1, *y)) {
                                    proposal_collisions.insert((x + 1, *y), true);
                                } else {
                                    proposal_collisions.insert((x + 1, *y), false);
                                }
                                break;
                            }
                        }
                        2 => {
                            if udlr.2 {
                                proposal.insert((*x, *y), (*x, y - 1));
                                if proposal_collisions.contains_key(&(*x, y - 1)) {
                                    proposal_collisions.insert((*x, y - 1), true);
                                } else {
                                    proposal_collisions.insert((*x, y - 1), false);
                                }
                                break;
                            }
                        }
                        3 => {
                            if udlr.3 {
                                proposal.insert((*x, *y), (*x, y + 1));
                                if proposal_collisions.contains_key(&(*x, y + 1)) {
                                    proposal_collisions.insert((*x, y + 1), true);
                                } else {
                                    proposal_collisions.insert((*x, y + 1), false);
                                }
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        for (old, new) in proposal {
            if !proposal_collisions.get(&new).unwrap() {
                elves.remove(&old);
                elves.insert(new);
            }
        }
        time += 1;
    }
    let mut minmaxs = (0i32,0i32,0i32,0i32);
    for i in &elves{
        minmaxs.0 = min(minmaxs.0, i.0);
        minmaxs.1 = max(minmaxs.1, i.0);
        minmaxs.2 = min(minmaxs.2, i.1);
        minmaxs.3 = max(minmaxs.3, i.1);
    }
    let elapsed = now.elapsed();
    println!("{:.2?}",elapsed);
    println!("{:?}",(minmaxs.1-minmaxs.0+1)*(minmaxs.3-minmaxs.2+1)-elves.len() as i32);
    let mut updated = true;
    while updated{
        updated = false;
        let mut proposal: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut proposal_collisions: HashMap<(i32, i32), bool> = HashMap::new();
        for (x, y) in &elves {
            let mut udlr = (true, true, true, true);
            if elves.contains(&(x - 1, y - 1)) {
                udlr.0 = false;
                udlr.2 = false;
            }
            if elves.contains(&(x - 1, *y)) {
                udlr.0 = false;
            }
            if elves.contains(&(x - 1, y + 1)) {
                udlr.0 = false;
                udlr.3 = false;
            }
            if elves.contains(&(*x, y + 1)) {
                udlr.3 = false;
            }
            if elves.contains(&(x + 1, y + 1)) {
                udlr.1 = false;
                udlr.3 = false;
            }
            if elves.contains(&(x + 1, *y)) {
                udlr.1 = false;
            }
            if elves.contains(&(x + 1, y - 1)) {
                udlr.1 = false;
                udlr.2 = false;
            }
            if elves.contains(&(*x, y - 1)) {
                udlr.2 = false;
            }
            if udlr == (true, true, true, true) {
                continue;
            } else {
                for i in time..time + 4 {
                    match i % 4 {
                        0 => {
                            if udlr.0 {
                                proposal.insert((*x, *y), (x - 1, *y));
                                if proposal_collisions.contains_key(&(x - 1, *y)) {
                                    proposal_collisions.insert((x - 1, *y), true);
                                } else {
                                    proposal_collisions.insert((x - 1, *y), false);
                                }
                                break;
                            }
                        }
                        1 => {
                            if udlr.1 {
                                proposal.insert((*x, *y), (x + 1, *y));
                                if proposal_collisions.contains_key(&(x + 1, *y)) {
                                    proposal_collisions.insert((x + 1, *y), true);
                                } else {
                                    proposal_collisions.insert((x + 1, *y), false);
                                }
                                break;
                            }
                        }
                        2 => {
                            if udlr.2 {
                                proposal.insert((*x, *y), (*x, y - 1));
                                if proposal_collisions.contains_key(&(*x, y - 1)) {
                                    proposal_collisions.insert((*x, y - 1), true);
                                } else {
                                    proposal_collisions.insert((*x, y - 1), false);
                                }
                                break;
                            }
                        }
                        3 => {
                            if udlr.3 {
                                proposal.insert((*x, *y), (*x, y + 1));
                                if proposal_collisions.contains_key(&(*x, y + 1)) {
                                    proposal_collisions.insert((*x, y + 1), true);
                                } else {
                                    proposal_collisions.insert((*x, y + 1), false);
                                }
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        for (old, new) in proposal {
            if !proposal_collisions.get(&new).unwrap() {
                elves.remove(&old);
                elves.insert(new);
                updated = true;
            }
        }
        time += 1;
    }
    let elapsed = now.elapsed();
    println!("{:.2?}",elapsed);
    println!("{:?}",time);
}
