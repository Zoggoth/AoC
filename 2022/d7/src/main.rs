#[derive(Debug)]
pub struct Node{
    pub parent: usize,
    pub children: Vec<usize>,
    pub name: String,
    pub size: i32,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No File");
    let mut current_node: usize = 0;
    let mut node_list = Vec::<Node>::new();
    for x in input.lines(){
        if x != "$ ls"{
            let split : Vec::<&str> = x.split(" ").collect();
            if split[0] == "$"{
                if split[1] == "cd"{
                    if split[2] == ".."{
                        let mut size = 0;
                        for y in &node_list[current_node].children{
                            size += node_list[*y].size;
                        }
                        node_list[current_node].size = size;
                        current_node = node_list[current_node].parent;
                    }
                    else{
                        let new_node : Node = Node {
                            parent: current_node,
                            children: vec![],
                            name: split[2].to_string(),
                            size: 0
                        };
                        let next_node = node_list.len();
                        node_list.push(new_node);
                        if split[2] != "/"{
                            node_list[current_node].children.push(next_node);
                        }
                        current_node = next_node
                    }
                }
            }
            else if split[0] != "dir"{
                let new_node : Node = Node {
                    parent: current_node,
                    children: vec![],
                    name: split[1].to_string(),
                    size: split[0].parse::<i32>().unwrap()
                };
                let next_node = node_list.len();
                node_list.push(new_node);
                node_list[current_node].children.push(next_node);
            }
        }
    }
    while node_list[current_node].size == 0{
        let mut size = 0;
        for y in &node_list[current_node].children{
            size += node_list[*y].size;
        }
        node_list[current_node].size = size;
        current_node = node_list[current_node].parent;
    }
    let space_needed = node_list[0].size - 40000000;
    let mut best_delete = node_list[0].size;
    let mut part1 = 0;
    for x in node_list{
        if x.children.len() != 0{
            if x.size <= 100000{
                part1 += x.size;
            }
            if x.size >= space_needed{
                if x.size < best_delete{
                    best_delete = x.size;
                }
            }
        }
    }
    println!("{:?}",part1);
    println!("{:?}",best_delete);
}
