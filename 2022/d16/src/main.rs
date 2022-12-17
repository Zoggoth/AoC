use std::collections::HashMap;
use std::fs::read_to_string;
use regex::Regex;
use bit_set::BitSet;

#[derive(Debug)]
#[derive(Clone)]
struct Location {
    name: String,
    flow_rate: i32,
    neighbour_string: String,
    neighbour_list: Vec<usize>
}

fn best_route(current_location: usize, available: &BitSet, time_left: i32, location_list: &Vec<Location>, pair_to_distance: &HashMap<(usize,usize),i32>, memoise: &mut HashMap<(usize,BitSet,i32),(i32,String)>)-> (i32,String){
    let mut new_available = available.clone();
    if memoise.contains_key(&(current_location,new_available,time_left)){
        return memoise.get(&(current_location,available.to_owned(),time_left)).unwrap().clone()
    }
    let mut best = 0;
    let mut best_name = "".to_string();
    for x in available{
        new_available = available.clone();
        new_available.remove(x);
        let new_time = time_left - pair_to_distance.get(&(current_location, x)).unwrap()-1;
        if new_time > 0{
            let best_continuation = best_route(x,&new_available,new_time,location_list,pair_to_distance,memoise);
            let score = location_list[x].flow_rate*new_time + best_continuation.0;
            if score > best{
                best = score;
                best_name = location_list[x].name.to_string() + " " + &*best_continuation.1;
            }
        }
    }
    memoise.insert((current_location,available.to_owned(),time_left),(best,best_name.to_owned()));
    return (best,best_name);
}

fn distance(x: usize, location_list: Vec<Location>, pair_to_distance: &mut HashMap<(usize, usize), i32>){
    pair_to_distance.insert((x,x),0);
    let mut todo: Vec<(usize,i32)> = vec![];
    for neighbour in 0..location_list[x].neighbour_list.len(){
        todo.push((location_list[x].neighbour_list[neighbour],1));
    }
    while !todo.is_empty(){
        let current = todo.drain(0..1).nth(0).unwrap();
        if !pair_to_distance.contains_key(&(x,current.0)){
            pair_to_distance.insert((x,current.0),current.1);
            for neighbour in 0..location_list[current.0].neighbour_list.len(){
                todo.push(((location_list[current.0].neighbour_list[neighbour]),current.1+1));
            }
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = read_to_string("input.txt").expect("No File");
    let re = Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? ([^\r]*)").unwrap();
    let mut name_to_location: HashMap<String,usize> = HashMap::new();
    let mut location_list: Vec<Location> = vec![];
    for capture in re.captures_iter(&input){
        let current_location = Location{
            name: capture[1].to_string(),
            flow_rate: capture[2].parse::<i32>().unwrap(),
            neighbour_string: capture[3].to_string(),
            neighbour_list: vec![]
        };
        name_to_location.insert(capture[1].to_string(), location_list.len());
        location_list.push(current_location);
    }
    for x in 0..location_list.len(){
        for neighbour in location_list[x].neighbour_string.clone().split(", "){
            location_list[x].neighbour_list.push(name_to_location[neighbour]);
        }
    }
    let mut pair_to_distance: HashMap<(usize,usize),i32> = HashMap::new();
    let mut available = BitSet::with_capacity(location_list.len());
    for x in 0..location_list.len(){
        if (&location_list[x].name == "AA") | (&location_list[x].flow_rate > &0) {
            available.insert(x);
            let temp_list = location_list.clone();
            distance(x,temp_list,&mut pair_to_distance);
        }
    }
    let mut best_route_memoise:HashMap<(usize,BitSet,i32),(i32,String)> = HashMap::new();
    let part1 = best_route(name_to_location["AA"],&available,30,&location_list,&pair_to_distance,& mut best_route_memoise);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = std::time::Instant::now();

    let mut me_available = available.clone();
    let mut part2_me = best_route(name_to_location["AA"],&me_available,26,&location_list,&pair_to_distance,& mut best_route_memoise);
    let mut elephant_available = available.clone();
    for x in part2_me.1[0..part2_me.1.len()-1].split(" "){
        elephant_available.remove(name_to_location[x]);
    }
    let mut part2_elephant = best_route(name_to_location["AA"],&elephant_available,26,&location_list,&pair_to_distance,& mut best_route_memoise);
    let mut best_score = part2_me.0+part2_elephant.0;
    let mut best_route_found = part2_me.1;
    loop{
        let mut best_new_score = 0;
        let mut best_removal = "";
        let mut best_new_route = "".to_string();
        for x in best_route_found[0..best_route_found.len()-1].split(" "){
            let mut me_available_test = me_available.clone();
            elephant_available = available.clone();
            me_available_test.remove(name_to_location[x]);
            part2_me = best_route(name_to_location["AA"],&me_available_test,26,&location_list,&pair_to_distance,& mut best_route_memoise);
            for x in part2_me.1[0..part2_me.1.len()-1].split(" "){
                elephant_available.remove(name_to_location[x]);
            }
            part2_elephant = best_route(name_to_location["AA"],&elephant_available,26,&location_list,&pair_to_distance,& mut best_route_memoise);
            let this_score = part2_me.0+part2_elephant.0;
            if this_score > best_new_score{
                best_new_score = this_score;
                best_removal = x;
                best_new_route = part2_me.1;
                // println!("{:?}",best_new_route);
                // println!("{:?}",part2_elephant.1);
            }
        }
        if best_new_score > best_score{
            me_available.remove(name_to_location[best_removal]);
            best_score = best_new_score;
            best_route_found = best_new_route;
        }
        else{
            break;
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:?}",part1.0);
    println!("{:?}",best_score);
    // for x in 0..location_list.len(){
    //     for y in 0..location_list[x].neighbour_list.len(){
    //         if x < location_list[x].neighbour_list[y] {
    //             println!("{} {}", location_list[x].name, location_list[location_list[x].neighbour_list[y]].name);
    //         }
    //     }
    // }
    // for x in 0..location_list.len(){
    //     let temp_list = location_list.clone();
    //     distance(x,temp_list,&mut pair_to_distance);
    //     for y in 0..location_list.len(){
    //         println!("{} {} {}",location_list[x].name,location_list[y].name,pair_to_distance[&(x,y)])
    //     }
    // }
}
