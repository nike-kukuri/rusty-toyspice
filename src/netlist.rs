#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]
use crate::elements::Element;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Netlist {
    pub v: HashMap<String, Element>,
    pub r: HashMap<String, Element>,
    pub c: HashMap<String, Element>,
    pub l: HashMap<String, Element>,
}

impl Netlist {
    fn new() -> Self {
        Self {
            v: HashMap::new(),
            r: HashMap::new(),
            c: HashMap::new(),
            l: HashMap::new(),
        }
    }
    fn total_elements(&self) -> usize {
       self.v.len() + self.r.len() + self.c.len() + self.l.len()
    }
}

// TODO 空行スキップ
pub fn parse_netlist() -> Result<Netlist, Box<dyn std::error::Error>> {
    let mut netlist: Netlist = Netlist::new();
    let args: Vec<String> = env::args().collect();
    let filepath_netlist = &args[1];
    let mut elem_ins = Vec::new();
    let mut elem_val = Vec::new();
    let mut node_pos = Vec::new();
    let mut node_neg = Vec::new();
    let mut line_vec = Vec::new();
    for line in BufReader::new(File::open(filepath_netlist)?).lines() {
        {
            let line_temp = line.unwrap().to_string();
            line_vec.push(line_temp);
        }
        let l: Vec<&str> = line_vec.last().unwrap().split_whitespace().collect();
        elem_ins.push(l[0].to_string());
        node_pos.push(l[1].parse::<usize>().unwrap());
        node_neg.push(l[2].parse::<usize>().unwrap());
        elem_val.push(l[3].parse::<f64>().unwrap()); 
    }

    let mut elem_vec: Vec<Element> = Vec::new();
    for (i, _) in elem_ins.iter().enumerate() {
        elem_vec.push(
            Element {
                pos: node_pos[i],
                neg: node_neg[i],
                value: elem_val[i],
            });
    }

    for (i, elem_i) in elem_ins.iter().enumerate() {
        if elem_i.chars().next().unwrap() == 'v' {
            netlist.v.insert(elem_i.to_string(), elem_vec[i]);
        }
        if elem_i.chars().next().unwrap() == 'r' {
            netlist.r.insert(elem_i.to_string(), elem_vec[i]);
        }
        if elem_i.chars().next().unwrap() == 'c' {
            netlist.c.insert(elem_i.to_string(), elem_vec[i]);
        }
    }

    Ok(netlist)
}
