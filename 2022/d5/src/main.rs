fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No File");
    let mut i = 0;
    for line in input.lines(){
        if let Some(value) = line.chars().nth(1) {
            if value == '1'{
                break;
            }
        }
        i += 1;
    }
    let initial= &input.lines().collect::<Vec<_>>()[0..i];
    let instructions= &input.lines().collect::<Vec<_>>()[i+2..];
    let stack_count = (initial[0].len()+1)/4;
    let mut stacks : Vec<Vec<char>> = Vec::new();
    let mut stacks2 : Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_count{
        let empty_stack : Vec<char> = Vec::new();
        let empty_stack2 : Vec<char> = Vec::new();
        stacks.push(empty_stack);
        stacks2.push(empty_stack2);
    }
    for line in initial.iter().rev(){
        for stack in 0..stack_count{
            let item = line.chars().nth(stack*4+1).unwrap();
            if item != ' '{
                stacks[stack].push(item);
                stacks2[stack].push(item);
            }
        }
    }
    for line in instructions{
        let words = line.split(" ").collect::<Vec<_>>();
        let mut holding: Vec<char> = Vec::new();
        for _ in 0..words[1].parse::<i32>().unwrap(){
            let item = stacks[words[3].parse::<usize>().unwrap()-1].pop().unwrap();
            let item2 = stacks2[words[3].parse::<usize>().unwrap()-1].pop().unwrap();
            holding.push(item2);
            stacks[words[5].parse::<usize>().unwrap()-1].push(item)
        }
        for _ in 0..words[1].parse::<i32>().unwrap(){
            let item2 = holding.pop().unwrap();
            stacks2[words[5].parse::<usize>().unwrap()-1].push(item2);
        }
    }
    for mut stack in stacks{
        print!("{}",stack.pop().unwrap())
    }
    println!();
    for mut stack in stacks2{
        print!("{}",stack.pop().unwrap())
    }
}
