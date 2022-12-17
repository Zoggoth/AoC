use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn print_tower(collision_list:HashSet<(i128,i128)>,highest_point:i128){
    for x in (0..highest_point).rev(){
        print!("|");
        for y in 0..7{
            if collision_list.contains(&(x,y)){
                print!("#");
            }
            else{
                print!(".");
            }
        }
        println!("|");
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("No File").chars().collect::<Vec<char>>();
    let mut now = std::time::Instant::now();
    let wind_loop = input.len();
    let mut wind_time = 0;
    let mut block_time: i128 = 0;
    let mut highest_point: i128 = 0;
    let mut collision_list:HashSet<(i128,i128)> = HashSet::new();
    for i in 0..7{
        collision_list.insert((0,i));
    }
    let mut bonus_high_point = 0;
    let mut memoise_loop_detection:HashMap<(i128, usize),(i128, i128)> = HashMap::new();
    let mut memoise_all_high_points:HashMap<i128,i128> = HashMap::new();
    let loop_window = 2;
    let mut state_match = 1;
    let mut loop_info = 0i128;
    while block_time < 1000000000000{
        let mut falling_blocks:Vec<(i128,i128)> = vec![];
        match block_time % 5 {
            0 => {
                falling_blocks.push((highest_point+4,2));
                falling_blocks.push((highest_point+4,3));
                falling_blocks.push((highest_point+4,4));
                falling_blocks.push((highest_point+4,5));
            }
            1 => {
                falling_blocks.push((highest_point+5,2));
                falling_blocks.push((highest_point+5,3));
                falling_blocks.push((highest_point+5,4));
                falling_blocks.push((highest_point+6,3));
                falling_blocks.push((highest_point+4,3));
            }
            2 => {
                falling_blocks.push((highest_point+4,2));
                falling_blocks.push((highest_point+4,3));
                falling_blocks.push((highest_point+4,4));
                falling_blocks.push((highest_point+5,4));
                falling_blocks.push((highest_point+6,4));
            }
            3 => {
                falling_blocks.push((highest_point+4,2));
                falling_blocks.push((highest_point+5,2));
                falling_blocks.push((highest_point+6,2));
                falling_blocks.push((highest_point+7,2));
            }
            4 => {
                falling_blocks.push((highest_point+4,2));
                falling_blocks.push((highest_point+4,3));
                falling_blocks.push((highest_point+5,2));
                falling_blocks.push((highest_point+5,3));
            }
            _ => {println!("Woo! the secret 6th block")}
        }
        let mut falling = true;
        while falling{
            let mut new_position = falling_blocks.clone();
            let mut possible = true;
            for x in 0..new_position.len(){
                if input[wind_time%wind_loop] == '>'{
                    new_position[x].1 += 1;
                    if new_position[x].1 == 7{
                        possible = false;
                    }
                }
                else{
                    new_position[x].1 -= 1;
                    if new_position[x].1 == -1{
                        possible = false;
                    }
                }
                if collision_list.contains(&(new_position[x])){
                    possible = false;
                }
            }
            if possible{
                falling_blocks = new_position.clone();
            }
            new_position = falling_blocks.clone();
            possible = true;
            for x in 0..new_position.len(){
                new_position[x].0 -= 1;
                if collision_list.contains(&(new_position[x])){
                    possible = false;
                }
            }
            if possible{
                falling_blocks = new_position.clone();
            }
            else{
                falling = false;
                for x in 0..falling_blocks.len(){
                    collision_list.insert(falling_blocks[x]);
                    highest_point = max(highest_point,falling_blocks[x].0);
                }
            }
            wind_time += 1;
        }
        block_time += 1;
        if block_time == 2022{
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
            println!("{:?}",highest_point);
            now = std::time::Instant::now();
        }
        if block_time > 2022{
            if memoise_loop_detection.contains_key(&(block_time % 5, wind_time % wind_loop)) {
                let last_state = memoise_loop_detection[&(block_time % 5, wind_time % wind_loop)];
                let high_point_loop = highest_point - last_state.1;
                if high_point_loop == loop_info{
                    state_match += 1;
                }
                else{
                    state_match = 1;
                    loop_info = high_point_loop;
                }
                if state_match >= loop_window {
                    let block_loop = block_time - last_state.0;
                    let big_loops_to_do = (1000000000000 - block_time) / block_loop;
                    block_time += block_loop * big_loops_to_do;
                    bonus_high_point = high_point_loop * big_loops_to_do;
                    let loops_to_do = 1000000000000 - block_time;
                    highest_point += memoise_all_high_points[&(loops_to_do + last_state.0)] - memoise_all_high_points[&last_state.0];
                    break;
                }
            }
            else{
                state_match = 1;
            }
        }
        memoise_loop_detection.insert((block_time%5, wind_time%wind_loop), (block_time, highest_point));
        memoise_all_high_points.insert(block_time,highest_point);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:?}",highest_point+bonus_high_point);
    // if wind_time == input.len(){
    //     wind_time = 0;
    // }
}
