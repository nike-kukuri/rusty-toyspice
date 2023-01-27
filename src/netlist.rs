#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]
use crate::elements::Element;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Netlist {
    pub v: HashMap<String, Element>,
    pub r: HashMap<String, Element>,
    pub c: HashMap<String, Element>,
    pub l: HashMap<String, Element>,
}