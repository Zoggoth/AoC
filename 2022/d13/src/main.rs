extern crate core;

use std::fs::read_to_string;
use std::cmp::{min, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use crate::Packet::{List, Value};

#[derive(Debug)]
enum Packet {
    Value(i32),
    List(Vec<Packet>),
    None
}

impl Eq for Packet {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Equal;
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        return match (self, other) {
            (Packet::None, Packet::None) => { Equal }
            (Packet::None, _) => { Less }
            (_, Packet::None) => { Greater }
            (Value(x), Value(y)) => { x.cmp(y) }
            (Value(x), y@ List(_)) => { List(vec![Value(*x)]).cmp(y) }
            (x@ List(_), Value(y)) => { x.cmp(&List(vec![Value(*y)])) }
            (List(x), List(y)) => {
                let zmax = min(x.len(),y.len());
                for z in 0..zmax{
                    let self_z = &x[z];
                    let other_z = &y[z];
                    match self_z.cmp(other_z){
                        Less => {return Less;}
                        Greater => {return Greater;}
                        Equal => {}
                    };
                }
                if x.len() < y.len(){
                    return Less;
                }
                if x.len() > y.len(){
                    return Greater;
                }
                Equal
            }
        }
    }
}

fn parse(description: &str) -> Packet {
    return if description == ""{
        Packet::None
    }
    else if description[0..1] == *"[" {
        let mut items: Vec<&str> = vec![];
        let mut depth = 0;
        let mut last = 1;
        for x in 1..description.len() - 1{
            match &description[x..x+1] {
                "[" => {depth+=1},
                "]" => {depth-=1},
                "," => {
                    if depth==0{
                        items.push(&description[last..x]);
                        last = x+1;
                    }
                }
                _ => {}
            }
        }
        if description != "" {
            items.push(&description[last..description.len() - 1]);
        }
        let mut list: Vec<Packet> = vec![];
        for x in items {
            list.push(parse(x));
        }
        let output = Packet::List(list);
        output
    } else {
        let output = Packet::Value(description.parse::<i32>().unwrap());
        output
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("No File");
    let pairs:Vec<&str> = input.split("\r\n\r\n").collect();
    let mut index = 1;
    let mut part1 = 0;
    let mut part2_list:Vec<Packet> = vec![];
    for x in pairs{
        let packets:Vec<&str> = x.split("\r\n").collect();
        let left_packet = parse(packets[0]);
        let right_packet = parse(packets[1]);
        if left_packet < right_packet{
            part1+=index;
        }
        index += 1;
        part2_list.push(left_packet);
        part2_list.push(right_packet);
    }
    println!("{:?}",part1);
    part2_list.push(parse("[[2]]"));
    part2_list.push(parse("[[6]]"));
    let decoder1 = parse("[[2]]");
    let decoder2 = parse("[[6]]");
    part2_list.sort();
    index = 1;
    let mut part2a = 0;
    let mut part2b = 0;
    for x in part2_list{
        if part2a == 0 {
            if x == decoder1 {
                part2a = index;
            }
        }
        else{
            if x == decoder2{
                part2b = index;
                break;
            }
        }
        index += 1;
    }
    println!("{:?}",part2a*part2b);
}
