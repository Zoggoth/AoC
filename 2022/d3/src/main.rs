use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No File");
    let mut score = 0;
    for line in input.lines(){
        let len = line.len()/2;
        let first = &line[..len];
        let second = &line[len..];
        let mut contains = HashSet::new();
        for char in first.chars(){
            contains.insert(char);
        }
        for char in second.chars(){
            if contains.contains(&char){
                let mut score_increase = char as i32 - 38;
                if score_increase > 52{
                    score_increase -= 58;
                }
                score += score_increase;
                break;
            }
        }
    }
    println!("{}",score);
    score = 0;
    let mut elf1 = "";
    let mut elf2 = "";
    for line in input.lines(){
        if elf1 == ""{
            elf1 = line;
        }
        else if elf2 == ""{
            elf2 = line;
        }
        else{
            let mut set1 = HashSet::new();
            for char in elf1.chars(){
                set1.insert(char);
            }
            let mut set2 = HashSet::new();
            for char in elf2.chars(){
                if set1.contains(&char){
                    set2.insert(char);
                }
            }
            for char in line.chars(){
                if set2.contains(&char){
                    let mut score_increase = char as i32 - 38;
                    if score_increase > 52{
                        score_increase -= 58;
                    }
                    score += score_increase;
                    break;
                }
            }
            elf1 = "";
            elf2 = "";
        }
    }
    println!("{}",score)
}
