#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]
mod elements;
mod matrix;
mod netlist;

use crate::elements::Element;
use crate::netlist::Netlist;

use std::f64::consts::PI as PI;
use std::collections::HashMap;

use ndarray::*;

fn main() {
    let mut v = HashMap::new();
    let mut r = HashMap::new();
    let mut c = HashMap::new();
    let mut l = HashMap::new();

    v.insert("v1".to_string(), Element{ pos: 1, neg: 2, value: 3.0 });
    r.insert("r1".to_string(), Element{ pos: 2, neg: 3, value: 0.3 });
    l.insert("l1".to_string(), Element{ pos: 3, neg: 4, value: 1.0e-9 });
    c.insert("c1".to_string(), Element{ pos: 4, neg: 1, value: 3.0e-6 });
    c.insert("l1".to_string(), Element{ pos: 4, neg: 1, value: 3.0e-6 });
    let netlist = Netlist { v, r, c, l };

    println!("");
    println!("----- Runnning Netlist -----");
    println!("{:?}", &netlist);
    println!("----- Runnning Netlist -----");
    println!("");

    // ログスケールの周波数ベクトル生成
    let frequencies_arr: Array1<_> = Array1::logspace(10.0, 0.0, 9.0, 10);
    let ang_freq_arr = 2f64 * PI * frequencies_arr;

    println!("----- Arg freq for analysis -----");
    println!("{:?}", ang_freq_arr);
    println!("----- Arg freq for analysis -----");
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netlist() {
        let mut v = HashMap::new();
        let mut r = HashMap::new();
        let mut c = HashMap::new();
        let mut l = HashMap::new();

        v.insert("v1".to_string(), Element{ pos: 1, neg: 2, value: 3.0 });
        r.insert("r1".to_string(), Element{ pos: 2, neg: 3, value: 0.3 });
        l.insert("l1".to_string(), Element{ pos: 3, neg: 4, value: 1.0e-9 });
        c.insert("c1".to_string(), Element{ pos: 4, neg: 1, value: 3.0e-6 });
        c.insert("c2".to_string(), Element{ pos: 4, neg: 1, value: 3.0e-6 });
        let netlist = Netlist { v, r, c, l };

        assert_eq!(vec!["v1"], netlist.v.keys().collect::<Vec<_>>());
        assert_eq!(vec!["r1"], netlist.r.keys().collect::<Vec<_>>());
        assert_eq!(vec!["l1"], netlist.l.keys().collect::<Vec<_>>());
        assert_eq!(vec!["c1", "c2"], netlist.c.keys().collect::<Vec<_>>());

        netlist.v.values().into_iter().for_each(|x| println!("first elem = {}, seconde elem = {}" , x.pos, x.neg));
    }
}