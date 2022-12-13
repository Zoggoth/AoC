use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut height_map: Vec<Vec<i16>> = vec![];
    let mut x_pos: usize = 0;
    let mut y_pos: usize;
    let mut start = (0,0);
    let mut end = (0,0);
    for x in input.lines(){
        y_pos = 0;
        let mut height_line: Vec<i16> = vec![];
        for y in x.chars(){
            if y == 'S'{
                height_line.push(0);
                start = (x_pos,y_pos);
            }
            else if y == 'E'{
                height_line.push(25);
                end = (x_pos,y_pos);
            }
            else{
                height_line.push(y as i16 - 97);
            }
            y_pos += 1;
        }
        x_pos += 1;
        height_map.push(height_line);
    }
    let mut distance_map = height_map.clone();
    for x in 0..distance_map.len(){
        for y in 0..distance_map[x].len(){
            distance_map[x][y] = 0;
        }
    }
    let distance_map2 = distance_map.clone();
    let mut todo: Vec<((usize,usize),i16)> = vec![];
    todo.push((start,1));
    while !todo.is_empty(){
        let location = todo.drain(0..1).nth(0).unwrap();
        if distance_map[location.0.0][location.0.1] == 0{
            distance_map[location.0.0][location.0.1] = location.1;
            let current_distance = distance_map[location.0.0][location.0.1];
            let current_height = height_map[location.0.0][location.0.1];
            let mut checking = location.0;
            checking.0 += 1;
            if checking.0 < distance_map.len() {
                if distance_map[checking.0][checking.1] == 0 {
                    if current_height + 1 >= height_map[checking.0][checking.1] {
                        todo.push((checking, current_distance + 1));
                    }
                }
            }
            if checking.0 > 1 {
                checking.0 -= 2;
                if distance_map[checking.0][checking.1] == 0 {
                    if current_height + 1 >= height_map[checking.0][checking.1] {
                        todo.push((checking, current_distance + 1));
                    }
                }
                checking.0 += 1;
            }
            else{
                checking.0 -= 1;
            }
            checking.1 += 1;
            if checking.1 < distance_map[checking.0].len() {
                if distance_map[checking.0][checking.1] == 0 {
                    if current_height + 1 >= height_map[checking.0][checking.1] {
                        todo.push((checking, current_distance + 1));
                    }
                }
            }
            if checking.1 > 1 {
                checking.1 -= 2;
                if distance_map[checking.0][checking.1] == 0 {
                    if current_height + 1 >= height_map[checking.0][checking.1] {
                        todo.push((checking, current_distance + 1));
                    }
                }
            }
        }
    }
    println!("{:?}",distance_map[end.0][end.1]-1);
    todo.push((end,1));
    distance_map = distance_map2;
    while !todo.is_empty(){
        let location = todo.drain(0..1).nth(0).unwrap();
        if distance_map[location.0.0][location.0.1] == 0{
            distance_map[location.0.0][location.0.1] = location.1;
            let current_distance = distance_map[location.0.0][location.0.1];
            let current_height = height_map[location.0.0][location.0.1];
            if current_height == 0{
                println!("{:?}",current_distance-1);
                break;
            }
            let mut checking = location.0;
            checking.0 += 1;
            if checking.0 < distance_map.len() {
                if distance_map[checking.0][checking.1] == 0 {
                    if current_height - 1 <= height_map[checking.0][checking.1] {
                        todo.push((checking, current_distance + 1));
                    }
                }
            }
            if checking.0 > 1 {
                checking.0 -= 2;
                if distance_map[checking.0][checking.1] == 0 {
                    if current_height - 1 <= height_map[checking.0][checking.1] {
                        todo.push((checking, current_distance + 1));
                    }
                }
                checking.0 += 1;
            }
            else{
                checking.0 -= 1;
            }
            checking.1 += 1;
            if checking.1 < distance_map[checking.0].len() {
                if distance_map[checking.0][checking.1] == 0 {
                    if current_height - 1 <= height_map[checking.0][checking.1] {
                        todo.push((checking, current_distance + 1));
                    }
                }
            }
            if checking.1 > 1 {
                checking.1 -= 2;
                if distance_map[checking.0][checking.1] == 0 {
                    if current_height - 1 <= height_map[checking.0][checking.1] {
                        todo.push((checking, current_distance + 1));
                    }
                }
            }
        }
    }
    // println!("{:?}",input);
    // for x in &distance_map{
    //     for y in x{
    //         print!("{:03} ",y);
    //     }
    //     println!();
    // }
    // for x in &height_map{
    //     for y in x{
    //         print!("{:02} ",y);
    //     }
    //     println!();
    // }
    // println!("{:?}",start);
    // println!("{:?}",end);
}
