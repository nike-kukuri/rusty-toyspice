use crate::netlist::Netlist;

use ndarray::*;
use ndarray_linalg::*;

#[derive(Debug)]
pub struct CircuitMatrix {
    mat: Array2<f64>,
    vec: Array1<f64>,
}

impl CircuitMatrix {
    fn new(a: Array2<f64>, b: Array1<f64>) -> Self {
        CircuitMatrix { mat: a, vec: b }
    }

    fn extend_mat(&mut self, netlist: Netlist) -> Self {
        todo!();
    }

    fn solve(a: Array2<f64>, b: Array1<f64>) -> Result<Array1<f64>, error::LinalgError> {
        let result: Array1<f64> = a.solve(&b)?;
        Ok(result)
    }
  }