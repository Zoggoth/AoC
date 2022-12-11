use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let mut register: i32 = 1;
    let mut time: i32 = 0;
    let mut part1 = 0;
    for x in input.lines(){
        let instruction:Vec<&str> = x.split(" ").collect();
        if instruction[0] == "noop"{
            if time % 40 == 20{
                part1 += register*time;
            }
            if ((time % 40) - register).abs() <=1{
                print!("#");
            }
            else{
                print!(".");
            }
            time += 1;
            if time % 40 == 0{
                println!();
            }
        }
        else{
            if time % 40 == 20{
                part1 += register*(time);
            }
            if ((time % 40) - register).abs() <=1{
                print!("#");
            }
            else{
                print!(".");
            }
            time += 1;
            if time % 40 == 0{
                println!();
            }
            if time % 40 == 20{
                part1 += register*(time);
            }
            if ((time % 40) - register).abs() <=1{
                print!("#");
            }
            else{
                print!(".");
            }
            time += 1;
            if time % 40 == 0{
                println!();
            }
            register += instruction[1].parse::<i32>().unwrap();
        }
    }
    println!("{:?}",part1)
}
