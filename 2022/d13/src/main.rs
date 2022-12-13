extern crate core;

use std::fs::read_to_string;
use std::cmp::{min, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug)]
#[derive(Clone)]
struct Packet {
    value: Option<i32>,
    list: Option<Vec<Packet>>
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
        let self_is_none = self.list.is_none() & self.value.is_none();
        let other_is_none = other.list.is_none() & other.value.is_none();
        if self_is_none & other_is_none{
            return Equal;
        }
        if self_is_none{
            return Less;
        }
        if other_is_none{
            return Greater;
        }
        if self.value.is_some() & other.value.is_some(){
            if self.value.unwrap() < other.value.unwrap(){
                return Less;
            }
            if self.value.unwrap() > other.value.unwrap(){
                return Greater;
            }
            return Equal;
        }
        let self_list = if self.value.is_some() {
            vec![Packet { value: self.value, list: None }]
        }
        else{
            self.list.clone().unwrap()
        };
        let other_list = if other.value.is_some() {
            vec![Packet { value: other.value, list: None }]
        }
        else{
            other.list.clone().unwrap()
        };
        let xmax = min(self_list.len(),other_list.len());
        for x in 0..xmax{
            let self_x = &self_list[x];
            let other_x = &other_list[x];
            match self_x.cmp(other_x){
                Less => {return Less;}
                Greater => {return Greater;}
                Equal => {}
            };
        }
        if self_list.len() < other_list.len(){
            return Less;
        }
        if self_list.len() > other_list.len(){
            return Greater;
        }
        return Equal;
    }
}

fn parse(description: &str) -> Packet {
    return if description == ""{
        return Packet { value: None, list: None }
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
        let output = Packet { value: None, list: Some(list) };
        output
    } else {
        let output = Packet { value: Some(description.parse::<i32>().unwrap()), list: None };
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
