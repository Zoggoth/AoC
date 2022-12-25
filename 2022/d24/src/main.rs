use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
#[derive(Clone)]
struct Blizzard {
    position: (i32,i32),
    direction: (i32,i32)
}

impl Blizzard {
    fn tick(&mut self, height:i32, width:i32){
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
        if self.position.0 <= 0 { self.position.0 = height;}
        if self.position.0 > height { self.position.0 = 1;}
        if self.position.1 <= 0 { self.position.1 = width;}
        if self.position.1 > width { self.position.1 = 1;}
    }
}

fn tick(blizzard_list: &mut Vec<Blizzard>, height:i32, width:i32){
    for x in blizzard_list{
        x.tick(height,width);
    }
}

fn spread(possible_list: &mut HashSet<(i32,i32)>, height:i32, width:i32){
    let fake_possible_list = possible_list.clone();
    for (x,y) in fake_possible_list.iter(){
        possible_list.insert((x+1,*y));
        possible_list.insert((x-1,*y));
        possible_list.insert((*x,y+1));
        possible_list.insert((*x,y-1));
    }
    let fake_possible_list = possible_list.clone();
    for (x,y) in fake_possible_list.iter(){
        if (*x <= 0) & ((*x,*y) != (0,1)) {
            possible_list.remove(&(*x, *y));
        }
        if (*x > height) & ((*x,*y) != (height+1,width)) {
            possible_list.remove(&(*x, *y));
        }
        if *y <= 0{
            possible_list.remove(&(*x, *y));
        }
        if *y > width{
            possible_list.remove(&(*x, *y));
        }
    }
}

fn prune(possible_list: &mut HashSet<(i32,i32)>, blizzard_list: &Vec<Blizzard>){
    for x in blizzard_list{
        possible_list.remove(&x.position);
    }
}

fn print_map(blizzard_list: &Vec<Blizzard>, possible_list: &HashSet<(i32,i32)>, height:i32, width:i32){
    let mut position_set: HashSet<(i32,i32)> = HashSet::new();
    for x in blizzard_list{
        position_set.insert(x.position);
    }
    for x in 0..height {
        for y in 0..width{
            if possible_list.contains(&(x + 1, y + 1)){
                print!("X");
            }
            else if position_set.contains(&(x + 1, y + 1)){
                print!("#");
            }
            else{
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut width= 0;
    let mut height = 0;
    let mut blizzard_list: Vec<Blizzard> = vec![];
    for line in input.lines(){
        width = 0;
        for x in line.chars(){
            match x {
                '>' => {blizzard_list.push(Blizzard{ position: (height, width), direction: (0, 1) })}
                '<' => {blizzard_list.push(Blizzard{ position: (height, width), direction: (0, -1) })}
                'v' => {blizzard_list.push(Blizzard{ position: (height, width), direction: (1, 0) })}
                '^' => {blizzard_list.push(Blizzard{ position: (height, width), direction: (-1, 0) })}
                _ => {}
            }
            width += 1;
        }
        height += 1;
    }
    height -= 2;
    width -= 2;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut possible_list: HashSet<(i32,i32)> = HashSet::new();
    possible_list.insert((0,1));
    while !possible_list.contains(&(height + 1, width)){
        tick(&mut blizzard_list,height,width);
        spread(&mut possible_list,height,width);
        prune(&mut possible_list,&blizzard_list);
        // print_map(&blizzard_list, &possible_list, height,width);
        // println!();
        part1 += 1;
        part2 += 1;
    }
    possible_list = HashSet::new();
    possible_list.insert((height + 1, width));
    while !possible_list.contains(&(0,1)){
        tick(&mut blizzard_list,height,width);
        spread(&mut possible_list,height,width);
        prune(&mut possible_list,&blizzard_list);
        // print_map(&blizzard_list, &possible_list, height,width);
        // println!();
        part2 += 1;
    }
    possible_list= HashSet::new();
    possible_list.insert((0,1));
    while !possible_list.contains(&(height + 1, width)){
        tick(&mut blizzard_list,height,width);
        spread(&mut possible_list,height,width);
        prune(&mut possible_list,&blizzard_list);
        // print_map(&blizzard_list, &possible_list, height,width);
        // println!();
        part2 += 1;
    }
    println!("{:?}",part1);
    println!("{:?}",part2);
}