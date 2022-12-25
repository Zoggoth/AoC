use std::fs::read_to_string;

#[derive(Debug)]
#[derive(Clone)]
struct Linked {
    value: i128,
    forward: usize,
    backward: usize
}

fn print_list(linked_list: Vec<Linked>, start: usize){
    let mut current = start;
    print!("{:?}, ",linked_list[current].value);
    current = linked_list[current].forward;
    while current != start{
        print!("{:?}, ",linked_list[current].value);
        current = linked_list[current].forward;
    }
    println!();
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut now = std::time::Instant::now();
    let mut part1_list: Vec<Linked> = vec![];
    let mut zero_position = 0;
    for line in input.lines(){
        let element = Linked{
            value: line.parse::<i128>().unwrap(),
            forward: part1_list.len()+1,
            backward: part1_list.len()-1
        };
        if line == "0"{
            zero_position = part1_list.len();
        }
        part1_list.push(element);
    }
    let length = part1_list.len();
    part1_list[0].backward = length-1;
    part1_list[length-1].forward = 0;
    let mut part2_list = part1_list.clone();
    for x in 0..length{
        let mut shift = part1_list[x].value%(length-1) as i128;
        if shift > length as i128/2{
            shift -= length as i128-1;
        }
        if shift < -(length as i128/2){
            shift += length as i128-1;
        }
        if shift > 0{
            shift -= 1;
            let mut current_position = part1_list[x].forward;
            let element = part1_list[x].clone();
            part1_list[element.forward].backward = element.backward;
            part1_list[element.backward].forward = element.forward;
            while shift > 0 {
                current_position = part1_list[current_position].forward;
                shift -= 1;
            }
            let next_position = part1_list[current_position].forward;
            part1_list[x].forward = next_position;
            part1_list[x].backward = current_position;
            part1_list[next_position].backward = x;
            part1_list[current_position].forward = x;
        }
        else if shift < 0 {
            let mut current_position = part1_list[x].forward;
            let element = part1_list[x].clone();
            part1_list[element.forward].backward = element.backward;
            part1_list[element.backward].forward = element.forward;
            while shift < 0 {
                current_position = part1_list[current_position].backward;
                shift += 1;
            }
            let previous_position = part1_list[current_position].backward;
            part1_list[x].forward = current_position;
            part1_list[x].backward = previous_position;
            part1_list[previous_position].forward = x;
            part1_list[current_position].backward = x;
        }
    }
    let mut current_position = zero_position;
    let mut part1 = 0;
    for _ in 0..1000{
        current_position = part1_list[current_position].forward;
    }
    part1 += part1_list[current_position].value;
    for _ in 0..1000{
        current_position = part1_list[current_position].forward;
    }
    part1 += part1_list[current_position].value;
    for _ in 0..1000{
        current_position = part1_list[current_position].forward;
    }
    part1 += part1_list[current_position].value;
    let elapsed = now.elapsed();
    println!("{:.2?}",elapsed);
    println!("{:?}",part1);
    now = std::time::Instant::now();
    for x in 0..length{
        part2_list[x].value *= 811589153;
    }
    for _ in 0..10 {
        for x in 0..length {
            let mut shift = part2_list[x].value % (length - 1) as i128;
            if shift > length as i128 / 2 {
                shift -= length as i128 - 1;
            }
            if shift < -(length as i128 / 2) {
                shift += length as i128 - 1;
            }
            if shift > 0 {
                shift -= 1;
                let mut current_position = part2_list[x].forward;
                let element = part2_list[x].clone();
                part2_list[element.forward].backward = element.backward;
                part2_list[element.backward].forward = element.forward;
                while shift > 0 {
                    current_position = part2_list[current_position].forward;
                    shift -= 1;
                }
                let next_position = part2_list[current_position].forward;
                part2_list[x].forward = next_position;
                part2_list[x].backward = current_position;
                part2_list[next_position].backward = x;
                part2_list[current_position].forward = x;
            } else if shift < 0 {
                let mut current_position = part2_list[x].forward;
                let element = part2_list[x].clone();
                part2_list[element.forward].backward = element.backward;
                part2_list[element.backward].forward = element.forward;
                while shift < 0 {
                    current_position = part2_list[current_position].backward;
                    shift += 1;
                }
                let previous_position = part2_list[current_position].backward;
                part2_list[x].forward = current_position;
                part2_list[x].backward = previous_position;
                part2_list[previous_position].forward = x;
                part2_list[current_position].backward = x;
            }
        }
    }
    current_position = zero_position;
    let mut part2 = 0;
    for _ in 0..1000{
        current_position = part2_list[current_position].forward;
    }
    part2 += part2_list[current_position].value;
    for _ in 0..1000{
        current_position = part2_list[current_position].forward;
    }
    part2 += part2_list[current_position].value;
    for _ in 0..1000{
        current_position = part2_list[current_position].forward;
    }
    part2 += part2_list[current_position].value;
    let elapsed = now.elapsed();
    println!("{:.2?}",elapsed);
    println!("{:?}",part2);
    // let input = read_to_string("input.txt").expect("No File");
    // let mut now = std::time::Instant::now();
    // let mut index = 0;
    // let mut coordinates:Vec<(i32,i32)> = vec![];
    // let mut part2:Vec<(i128,i32)> = vec![];
    // for line in input.lines(){
    //     coordinates.push((line.parse().unwrap(),index));
    //     part2.push((line.parse::<i128>().unwrap()*811589153,index));
    //     index += 1;
    // }
    // let max_index = index-1;
    // index = 0;
    // while index <= max_index{
    //     let mut search_index = 0;
    //     loop{
    //         if coordinates[search_index].1 == index{
    //             break;
    //         }
    //         search_index += 1;
    //     }
    //     let current_coordinate = coordinates.remove(search_index);
    //     coordinates.insert((((search_index as i32+current_coordinate.0)%max_index+max_index)%max_index) as usize,current_coordinate);
    //     index += 1;
    // }
    // let mut search_index = 0;
    // loop{
    //     if coordinates[search_index].0 == 0{
    //         break;
    //     }
    //     search_index += 1;
    // }
    // for _ in 0..10{
    //     index = 0;
    //     while index <= max_index{
    //         let mut search_index = 0;
    //         loop{
    //             if part2[search_index].1 == index{
    //                 break;
    //             }
    //             search_index += 1;
    //         }
    //         let current_coordinate = part2.remove(search_index);
    //         part2.insert((((search_index as i128+current_coordinate.0)%max_index as i128+max_index as i128)%max_index as i128) as usize,current_coordinate);
    //         index += 1;
    //     }
    // }
    // let elapsed = now.elapsed();
    // println!("{:.2?}",elapsed);
    // let mut search_index = 0;
    // loop{
    //     if coordinates[search_index].0 == 0{
    //         break;
    //     }
    //     search_index += 1;
    // }
    // println!("{:?}",coordinates[(search_index+1000)%coordinates.len()].0+coordinates[(search_index+2000)%coordinates.len()].0+coordinates[(search_index+3000)%coordinates.len()].0);
    // let mut search_index = 0;
    // loop{
    //     if part2[search_index].0 == 0{
    //         break;
    //     }
    //     search_index += 1;
    // }
    // println!("{:?}",part2[(search_index+1000)%part2.len()].0+part2[(search_index+2000)%part2.len()].0+part2[(search_index+3000)%part2.len()].0);
}
