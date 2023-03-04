#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]
mod elements;
mod matrix;
mod netlist;

use crate::elements::Element;
use crate::netlist::Netlist;
use crate::netlist::parse_netlist;
use crate::matrix::CircuitMatrix;
use crate::matrix::Analysis;

use std::f64::consts::PI as PI;
use std::collections::HashMap;

use anyhow::Result;
use ndarray::*;
use num_complex::Complex64;

fn main() -> Result<()> {
    let netlist: Netlist = parse_netlist().unwrap();

    /*
    let c1_element = Element { pos: 2, neg: 0, value: 1.0e-3 };
    let c2_element = Element { pos: 3, neg: 0, value: 1.0e-3 };
    let r1_element = Element { pos: 1, neg: 2, value: 1.0e3 };
    let r2_element = Element { pos: 2, neg: 3, value: 1.0e3 };
    let v1_element = Element { pos: 1, neg: 0, value: 1.0 };
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
    netlist.v.insert(String::from("v1"), v1_element);
    */

    println!("");
    println!("----- Runnning Netlist -----");
    println!("{:?}", &netlist);
    println!("----- Runnning Netlist -----");
    println!("");

    // ログスケールの周波数ベクトル生成
    let frequencies_arr: Array1<_> = Array1::logspace(10.0, 0.0, 9.0, 100);
    let omega_arr = 2f64 * PI * frequencies_arr;

    println!("----- Arg freq for analysis -----");
    println!("{:?}", omega_arr);
    println!("----- Arg freq for analysis -----");
    println!("");
    println!("----- solve  -----");
    let mut results_z: Vec<Result<Array1<Complex64>>> = vec![];
    for omega in omega_arr.iter() {
        let mut matrix = CircuitMatrix::new();
        matrix.create_mat_vec_from_netlist(&netlist, Analysis::AC, *omega)?;
        matrix.remove_ground();
        //results_z.push(matrix.solve());
    }
    /*
    println!("----- solve  -----");
    println!("");

    let mut results_arg = vec![];
    let mut results_norm = vec![];
    let spectre_node = 3; //観測するノード番号

    for result in results_z.iter() {
        for (node_i, value) in result.as_ref().unwrap().iter().enumerate() {
            if node_i == spectre_node {
                results_arg.push(value.arg());
                results_norm.push(value.norm());
            }
        }
    }
    println!("----- norm  -----");
    for norm in results_norm {
        println!("{norm}");
    }
    println!("----- norm  -----");
    println!("----- arg  -----");
    for arg in results_arg {
        println!("{arg}");
    }
    println!("----- arg  -----");
    */

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netlist() {
    }
}
