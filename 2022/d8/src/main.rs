fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No File");
    let grid: Vec<&str> = input.lines().collect();
    let depth = grid.len();
    let width = grid[0].len();
    let mut bool_grid = vec!(vec!(false;depth);width);
    let mut scenic_score_left = vec!(vec!(0;depth);width);
    let mut scenic_score_right = vec!(vec!(0;depth);width);
    let mut scenic_score_up = vec!(vec!(0;depth);width);
    let mut scenic_score_down = vec!(vec!(0;depth);width);
    for x in 0..depth{
        let mut height = '/';
        let mut height_to_scenic_score = [0;10];
        for y in 0..width{
            let tree = grid[x].chars().nth(y).unwrap();
            scenic_score_left[x][y] = height_to_scenic_score[tree as usize - 48];
            for z in 0..10{
                if z > tree as usize - 48{
                    height_to_scenic_score[z] += 1;
                }
                else{
                    height_to_scenic_score[z] = 1;
                }
            }
            if tree > height{
                height = tree;
                bool_grid[x][y] = true;
            }
        }
    }
    for x in 0..depth{
        let mut height = '/';
        let mut height_to_scenic_score = [0;10];
        for y in (0..width).rev(){
            let tree = grid[x].chars().nth(y).unwrap();
            scenic_score_right[x][y] = height_to_scenic_score[tree as usize - 48];
            for z in 0..10{
                if z > tree as usize - 48{
                    height_to_scenic_score[z] += 1;
                }
                else{
                    height_to_scenic_score[z] = 1;
                }
            }
            if tree > height{
                height = tree;
                bool_grid[x][y] = true;
            }
        }
    }
    for y in 0..width{
        let mut height = '/';
        let mut height_to_scenic_score = [0;10];
        for x in 0..depth{
            let tree = grid[x].chars().nth(y).unwrap();
            scenic_score_up[x][y] = height_to_scenic_score[tree as usize - 48];
            for z in 0..10{
                if z > tree as usize - 48{
                    height_to_scenic_score[z] += 1;
                }
                else{
                    height_to_scenic_score[z] = 1;
                }
            }
            if tree > height{
                height = tree;
                bool_grid[x][y] = true;
            }
        }
    }
    for y in 0..width{
        let mut height = '/';
        let mut height_to_scenic_score = [0;10];
        for x in (0..depth).rev(){
            let tree = grid[x].chars().nth(y).unwrap();
            scenic_score_down[x][y] = height_to_scenic_score[tree as usize - 48];
            for z in 0..10{
                if z > tree as usize - 48{
                    height_to_scenic_score[z] += 1;
                }
                else{
                    height_to_scenic_score[z] = 1;
                }
            }
            if tree > height{
                height = tree;
                bool_grid[x][y] = true;
            }
        }
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for x in 0..depth{
        for y in 0..width{
            if bool_grid[x][y]{
                part1 += 1;
            }
            let score = scenic_score_left[x][y]*scenic_score_right[x][y]*scenic_score_up[x][y]*scenic_score_down[x][y];
            part2 = std::cmp::max(part2,score);
        }
    }
    println!("{:?}",part1);
    println!("{:?}",part2);
}
