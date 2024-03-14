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
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    foo: String,
    spectre_node: Vec<String>,
    gnd: String,
}

fn main() -> Result<()> {
    let netlist: Netlist = parse_netlist().unwrap();

    println!("----- Runnning Netlist -----");
    println!("{:?}", &netlist);
    println!("----- Runnning Netlist -----");

    // ログスケールの周波数ベクトル生成
    let frequencies_arr: Array1<_> = Array1::logspace(10.0, 0.0, 9.0, 100);
    let omega_arr = 2f64 * PI * frequencies_arr;

    println!("----- Arg freq for analysis -----");
    println!("{:?}", omega_arr);
    println!("----- Arg freq for analysis -----");
    println!("----- solve  -----");
    let mut results_z: Vec<Result<Array1<Complex64>>> = vec![];
    let gnd = "0"; // temporary
    for omega in omega_arr.iter() {
        let mut matrix = CircuitMatrix::new();
        matrix.create_mat_vec(&netlist, Analysis::AC, *omega, gnd)?;
        matrix.remove_ground(gnd);
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
