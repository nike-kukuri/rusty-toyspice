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

use anyhow::Result;
use ndarray::*;
use num_complex::Complex64;

fn main() {
    let c1_element = Element { pos: 2, neg: 0, value: 3.0e-6 };
    let c2_element = Element { pos: 3, neg: 0, value: 3.0e-6 };
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
    let mut results: Vec<Result<Array1<Complex64>>> = vec![];
    for omega in omega_arr.iter() {
        let mut matrix = CircuitMatrix::new();
        matrix.create_mat_vec_from_netlist(&netlist, Analysis::AC, *omega);
        println!("---- before remove GND ----");
        let (mat, vec) = matrix.get_current_mat_vec();
        println!("mat: \n{:?}", mat);
        println!("vec: \n{:?}", vec);
        matrix.remove_ground();
        println!("---- after remove GND ----");
        let (mat, vec) = matrix.get_current_mat_vec();
        println!("mat: \n{:?}", mat);
        println!("vec: \n{:?}", vec);
        println!("---- GND ----");
        results.push(matrix.solve());
    }
    println!("----- solve  -----");
    println!("");

    
    for result in results.iter() {
        println!("result: \n{:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netlist() {
    }
}
