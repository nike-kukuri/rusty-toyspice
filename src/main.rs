#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]
mod elements;
mod matrix;
mod netlist;

use crate::elements::Element;
use crate::netlist::Netlist;
use crate::matrix::CircuitMatrix;
use crate::matrix::Analysis;

use std::f64::consts::PI as PI;
use std::collections::HashMap;

use ndarray::*;

fn main() {
    let c1_element = Element { pos: 2, neg: 0, value: 3.0 };
    let c2_element = Element { pos: 3, neg: 0, value: 3.0 };
    let r1_element = Element { pos: 0, neg: 1, value: 3.0 };
    let r2_element = Element { pos: 1, neg: 2, value: 3.0 };
    let mut netlist = Netlist { 
            v: HashMap::new(),
            r: HashMap::new(),
            c: HashMap::new(),
            l: HashMap::new(),
    };
    netlist.c.insert(String::from("c1"), c1_element);
    netlist.c.insert(String::from("c2"), c2_element);
    netlist.r.insert(String::from("r1"), r1_element);
    netlist.r.insert(String::from("r2"), r2_element);

    println!("");
    println!("----- Runnning Netlist -----");
    println!("{:?}", &netlist);
    println!("----- Runnning Netlist -----");
    println!("");

    // ログスケールの周波数ベクトル生成
    let frequencies_arr: Array1<_> = Array1::logspace(10.0, 0.0, 9.0, 10);
    let omega_arr = 2f64 * PI * frequencies_arr;

    println!("----- Arg freq for analysis -----");
    println!("{:?}", omega_arr);
    println!("----- Arg freq for analysis -----");
    println!("");
    println!("----- initialize matrix -----");
    let matrix = CircuitMatrix::new();
    let (mat, vec) = matrix.get_current_mat_vec();
    println!("mat: {}", mat);
    println!("vec: {}", vec);
    println!("----- initialize matrix -----");
    println!("");
    println!("----- solve  -----");
    for omega in omega_arr.iter() {
        let mut matrix = CircuitMatrix::new();
        matrix.create_mat_vec_from_netlist(&netlist, Analysis::AC, *omega);
        let (mat, vec) = matrix.get_current_mat_vec();
        let _result = matrix.solve();
        println!("mat: \n{:?}", mat);
        println!("vec: \n{:?}", vec);
    }
    println!("----- solve  -----");
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
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
    */
}
