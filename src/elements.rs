#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]

use crate::matrix::CircuitMatrix;

use ndarray::*;
use ndarray_linalg::*;

#[derive(Debug)]
pub struct Element {
    pub pos: usize,
    pub neg: usize,
    pub value: f64,
}

pub trait VoltageSource {
    fn gen_mat_vec(elem: Element, params: f64) -> (Array2<f64>, Array1<f64>);
}
pub trait Resistor {
    fn gen_mat_vec(elem: Element, params: f64) -> (Array2<f64>, Array1<f64>);
}
pub trait Capacitor {
    fn gen_mat_vec(elem: Element, params: f64) -> (Array2<f64>, Array1<f64>);
}
pub trait Inductor {
    fn gen_mat_vec(elem: Element, params: f64) -> (Array2<f64>, Array1<f64>);
}

impl VoltageSource for CircuitMatrix {
    fn gen_mat_vec(elem: Element, params: f64) -> (Array2<f64>, Array1<f64>) {
        let a: Array2<f64> = random((3, 3));
        let b: Array1<f64> = random(3);
        (a, b)
    }
}
impl Resistor for CircuitMatrix {
    fn gen_mat_vec(elem: Element, params: f64) -> (Array2<f64>, Array1<f64>) {
        let a: Array2<f64> = random((3, 3));
        let b: Array1<f64> = random(3);
        (a, b)
    }
}
impl Capacitor for CircuitMatrix {
    fn gen_mat_vec(elem: Element, params: f64) -> (Array2<f64>, Array1<f64>) {
        let a: Array2<f64> = random((3, 3));
        let b: Array1<f64> = random(3);
        (a, b)
    }
}
impl Inductor for CircuitMatrix {
    fn gen_mat_vec(elem: Element, params: f64) -> (Array2<f64>, Array1<f64>) {
        let a: Array2<f64> = random((3, 3));
        let b: Array1<f64> = random(3);
        (a, b)
    }
}