fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No File");
    let mut count1 = 0;
    let mut count2 = 0;
    for line in input.lines(){
        let low1 = line.split(",").nth(0).unwrap().split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let high1 = line.split(",").nth(0).unwrap().split("-").nth(1).unwrap().parse::<i32>().unwrap();
        let low2 = line.split(",").nth(1).unwrap().split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let high2 = line.split(",").nth(1).unwrap().split("-").nth(1).unwrap().parse::<i32>().unwrap();
        if low1 <= low2 && low2 <= high2 && high2 <= high1{
            count1 += 1;
        }
        else if low2 <= low1 && low1 <= high1 && high1 <= high2{
            count1 += 1;
        }
        if low1 <= high2 && low2 <= high1{
            count2 += 1;
        }
        else if low2 <= high1 && low1 <= high2{
            count2 += 1;
        }
    }
    println!("{}",count1);
    println!("{}",count2);
}
