use std::collections::HashSet;

fn printState(bounds: [i32;4], positions: [(i32,i32);10]){
    for x in bounds[0]..bounds[1]+1{
        for y in bounds[2]..bounds[3]+1{
            let mut printed = false;
            for z in 0..10{
                if positions[z] == (x,y){
                    print!("{:?}",z);
                    printed = true;
                    break;
                }
            }
            if !printed{
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let mut total_bounds = [0i32;4];
    let input = std::fs::read_to_string("input.txt").expect("No File");
    let mut positions = [(0i32,0i32);10];
    let mut part_1_visited:HashSet<(i32,i32)> = HashSet::new();
    let mut part_2_visited:HashSet<(i32,i32)> = HashSet::new();
    for x in input.lines(){
        let instruction:Vec<&str> = x.split(" ").collect();
        let direction = instruction[0];
        let amount = instruction[1].parse::<i32>().unwrap();
        let mut up = 0;
        let mut right = 0;
        if direction == "R"{
            right = 1;
        }
        if direction == "L"{
            right = -1;
        }
        if direction == "U"{
            up = 1;
        }
        if direction == "D"{
            up = -1;
        }
        for _ in 0..amount{
            positions[0].0 += up;
            positions[0].1 += right;
            if positions[0].0 < total_bounds[0]{
                total_bounds[0]=positions[0].0
            }
            if positions[0].0 > total_bounds[1]{
                total_bounds[1]=positions[0].0
            }
            if positions[0].1 < total_bounds[2]{
                total_bounds[2]=positions[0].1
            }
            if positions[0].1 > total_bounds[3]{
                total_bounds[3]=positions[0].1
            }
            let mut bounds = [positions[0].0,positions[0].0,positions[0].1,positions[0].1];
            for y in 1..10{
                if positions[y-1].0 - positions[y].0 == 2{
                    positions[y].0 += 1;
                    if positions[y].1 < positions[y-1].1{
                        positions[y].1 += 1;
                    }
                    if positions[y].1 > positions[y-1].1{
                        positions[y].1 -= 1;
                    }
                }
                if positions[y-1].0 - positions[y].0 == -2{
                    positions[y].0 -= 1;
                    if positions[y].1 < positions[y-1].1{
                        positions[y].1 += 1;
                    }
                    if positions[y].1 > positions[y-1].1{
                        positions[y].1 -= 1;
                    }
                }
                if positions[y-1].1 - positions[y].1 == 2{
                    positions[y].1 += 1;
                    if positions[y].0 < positions[y-1].0{
                        positions[y].0 += 1;
                    }
                    if positions[y].0 > positions[y-1].0{
                        positions[y].0 -= 1;
                    }
                }
                if positions[y-1].1 - positions[y].1 == -2{
                    positions[y].1 -= 1;
                    if positions[y].0 < positions[y-1].0{
                        positions[y].0 += 1;
                    }
                    if positions[y].0 > positions[y-1].0{
                        positions[y].0 -= 1;
                    }
                }
                if positions[y].0 < bounds[0]{
                    bounds[0]=positions[y].0
                }
                if positions[y].0 > bounds[1]{
                    bounds[1]=positions[y].0
                }
                if positions[y].1 < bounds[2]{
                    bounds[2]=positions[y].1
                }
                if positions[y].1 > bounds[3]{
                    bounds[3]=positions[y].1
                }
            }
            // printState(total_bounds,positions);
            part_1_visited.insert(positions[1]);
            part_2_visited.insert(positions[9]);
        }
    }
    println!("{:?}",part_1_visited.len());
    println!("{:?}",part_2_visited.len());
}
