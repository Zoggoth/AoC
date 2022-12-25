use std::cmp::max;
use std::fs::read_to_string;
use regex::Regex;

fn tick(mut state: [i32;8]) -> [i32;8]{
    state[1] += state[0];
    state[3] += state[2];
    state[5] += state[4];
    state[7] += state[6];
    return state;
}

fn calculate(blueprint: [i32;6], time: i32, state: [i32;8]) -> i32{
    if time < 0{
        return 0;
    }
    if time == 0{
        return state[7];
    }
    let mut best = 0;
    // Decide to build Geode robot
    if state[4] > 0{ // need to be generating Obsidian
        let mut new_time = time;
        let mut new_state = state;
        while (blueprint[4] > new_state[1])|(blueprint[5] > new_state[5]){
            new_state = tick(new_state);
            new_time -= 1;
        }
        new_state = tick(new_state);
        new_time -= 1;
        new_state[1] -= blueprint[4];
        new_state[5] -= blueprint[5];
        new_state[6] += 1;
        best = max(best,calculate(blueprint,new_time,new_state));
    }
    if (blueprint[4] <= state[1])&(blueprint[5] <= state[5]){
        return best;
    }
    // Decide to build an Obsidian robot
    if state[2] > 0{ // Need to be generating Clay
        if state[4] < blueprint[5]{ // Don't bother if we already have enough Obsidian
            let mut new_time = time;
            let mut new_state = state;
            while (blueprint[2] > new_state[1])|(blueprint[3] > new_state[3]){
                new_state = tick(new_state);
                new_time -= 1;
            }
            new_state = tick(new_state);
            new_time -= 1;
            new_state[1] -= blueprint[2];
            new_state[3] -= blueprint[3];
            new_state[4] += 1;
            best = max(best, calculate(blueprint,new_time,new_state));
        }
    }
    // Decide to build a Clay robot
    if state[6] == 0 {// Clay is never built after Geode (no idea if this is 100% true)
        if state[2] < blueprint[3] { // Don't bother if we already have enough Clay
            let mut new_time = time;
            let mut new_state = state;
            while blueprint[1] > new_state[1] {
                new_state = tick(new_state);
                new_time -= 1;
            }
            new_state = tick(new_state);
            new_time -= 1;
            new_state[1] -= blueprint[1];
            new_state[2] += 1;
            best = max(best, calculate(blueprint, new_time, new_state));
        }
    }
    // Decide to build an Ore robot
    if state[6] == 0 { // Ore is never built after Geode (no idea if this is 100% true)
        if state[0] < max(max(blueprint[1], blueprint[2]), blueprint[4]) { // Don't bother if we already have enough Ore
            let mut new_time = time;
            let mut new_state = state;
            while blueprint[0] > new_state[1] {
                new_state = tick(new_state);
                new_time -= 1;
            }
            new_state = tick(new_state);
            new_time -= 1;
            new_state[1] -= blueprint[0];
            new_state[0] += 1;
            best = max(best, calculate(blueprint, new_time, new_state));
        }
    }
    // Wait for death
    if best == 0 {
        let mut new_time = time;
        let mut new_state = state;
        while new_time > 0 {
            new_state = tick(new_state);
            new_time -= 1;
        }
        best = max(best, new_state[7]);
    }
    return best;
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let now = std::time::Instant::now();
    let re = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();
    let mut blueprints:Vec<[i32;6]> = vec![];
    for capture in re.captures_iter(&*input){
        let mut blueprint:[i32;6] = [0;6];
        for i in 1..7{
            blueprint[i-1]=capture[i].parse::<i32>().unwrap();
        }
        blueprints.push(blueprint);
    }
    let mut index = 1;
    let mut part1 = 0;
    let state = [1, 0, 0, 0, 0, 0, 0, 0];
    for x in &blueprints{
        part1 += index*calculate(*x,24,state);
        index += 1;
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let part2 = calculate(blueprints[0],32,state)*calculate(blueprints[1],32,state)*calculate(blueprints[2],32,state);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:?}",part1);
    println!("{:?}",part2);
}