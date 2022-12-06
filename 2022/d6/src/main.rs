fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No File");
    let mut i = 0;
    loop{
        if input.chars().nth(i+2).unwrap() == input.chars().nth(i+3).unwrap() {
            i += 3;
        }
        else if (input.chars().nth(i+1).unwrap() == input.chars().nth(i+2).unwrap())|(input.chars().nth(i+1).unwrap() == input.chars().nth(i+3).unwrap()){
            i += 2;
        }
        else if (input.chars().nth(i).unwrap() == input.chars().nth(i+1).unwrap())|(input.chars().nth(i).unwrap() == input.chars().nth(i+2).unwrap())|(input.chars().nth(i).unwrap() == input.chars().nth(i+3).unwrap()){
            i += 1;
        }
        else{
            break;
        }
    }
    println!("{:?}",i+4);
    i = 0;
    loop{
        let mut jump = 0;
        for x in (0..14).rev(){
            for y in x+1..14{
                if input.chars().nth(i+x).unwrap() == input.chars().nth(i+y).unwrap(){
                    jump = x+1;
                    break;
                }
            }
            if jump != 0{
                break;
            }
        }
        if jump == 0{
            break;
        }
        else{
            i += jump;
        }
    }
    println!("{:?}",i+14);
}
