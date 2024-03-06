#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]
use crate::elements::{Element, ElementType};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

#[derive(Debug)]
pub struct Netlist {
    pub v: Vec<Element>,
    pub i: Vec<Element>,
    pub r: Vec<Element>,
    pub c: Vec<Element>,
    pub l: Vec<Element>,
}

impl Netlist {
    fn new() -> Self {
        Self {
            v: Vec::new(),
            i: Vec::new(),
            r: Vec::new(),
            c: Vec::new(),
            l: Vec::new(),
        }
    }
    fn total_elements(&self) -> usize {
       self.v.len() + self.r.len() + self.c.len() + self.l.len()
    }
}

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
        if let Ok(line) = &line {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
        }
        {
            let line_temp = line.unwrap().to_string();
            line_vec.push(line_temp);
        }
        let l: Vec<&str> = line_vec.last().unwrap().split_whitespace().collect();
        if l.len() != 4 {
            panic!("Please, check netlist format: at one line, 4columns split with white spaces")
        }
        elem_ins.push(l[0].to_string());
        node_pos.push(l[1].to_string());
        node_neg.push(l[2].to_string());
        elem_val.push(l[3].parse::<f64>().unwrap()); 
    }

    let mut elem_vec: Vec<Element> = Vec::new();
    for (i, name) in elem_ins.iter().enumerate() {
        let kind = classification_kind(name.to_lowercase().chars().nth(0).unwrap());
        let elem = Element {
            name: name.clone(),
            pos: node_pos[i].clone(),
            neg: node_neg[i].clone(),
            value: elem_val[i],
            kind: kind
        };
        match kind {
            ElementType::V => netlist.v.push(elem),
            ElementType::I => netlist.i.push(elem),
            ElementType::R => netlist.r.push(elem),
            ElementType::C => netlist.c.push(elem),
            ElementType::L => netlist.l.push(elem),
            _ => panic!("In netlist, exist NO defined element")
        }
    }

    Ok(netlist)
}

fn classification_kind(ins: char) -> ElementType {
    match ins {
        'v' => ElementType::V,
        'i' => ElementType::I,
        'r' => ElementType::R,
        'c' => ElementType::C,
        'l' => ElementType::L,
        _ => panic!("In netlist, exist NO defined element.")
    }
}
