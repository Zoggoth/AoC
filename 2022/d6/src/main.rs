use fancy_regex::Regex;

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
    let now = std::time::Instant::now();
    // loop{
    //     let mut jump = 0;
    //     for x in (0..14).rev(){
    //         for y in x+1..14{
    //             if input.chars().nth(i+x).unwrap() == input.chars().nth(i+y).unwrap(){
    //                 jump = x+1;
    //                 break;
    //             }
    //         }
    //         if jump != 0{
    //             break;
    //         }
    //     }
    //     if jump == 0{
    //         break;
    //     }
    //     else{
    //         i += jump;
    //     }
    // }
    // loop{
    //     let mut jump = 0;
    //     let mut already = std::collections::HashSet::new();
    //     already.insert(input.chars().nth(i+14).unwrap());
    //     for x in (0..14).rev(){
    //         if already.contains(&input.chars().nth(i+x).unwrap()){
    //             jump = x;
    //             break;
    //         }
    //         else{
    //             already.insert(input.chars().nth(i+x).unwrap());
    //         }
    //     }
    //     if jump == 0{
    //         break;
    //     }
    //     else{
    //         i += jump;
    //     }
    // }
    let new_input: &[u8]= input.as_ref();
    loop{
        let mut jump = 0;
        let mut already = bit_set::BitSet::with_capacity(26);
        already.insert((new_input[i+14] - 97) as usize);
        for x in (0..14).rev(){
            if already.contains((new_input[i + x] - 97) as usize){
                jump = x;
                break;
            }
            else{
                already.insert((new_input[i + x] - 97) as usize);
            }
        }
        if jump == 0{
            break;
        }
        else{
            i += jump;
        }
    }
    // let mut regex_text : String = "(.)".to_string();
    // let mut regex_part : String = "".to_string();
    // for i in 1..14{
    //     regex_part.push_str(&*("(?!\\".to_owned() + &*i.to_string() + ")"));
    //     regex_text.push_str(&*(regex_part.to_owned() + "(.)"));
    // }
    // let re = Regex::new(&*regex_text).unwrap();
    // i = re.find(&input).unwrap().unwrap().start();
    let elapsed = now.elapsed();
    println!("{:?}",i+14);
    println!("Elapsed: {:.2?}", elapsed);
}
