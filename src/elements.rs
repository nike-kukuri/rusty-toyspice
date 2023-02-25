#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]

use num_complex::{Complex, Complex64};
use ndarray::*;

use crate::matrix::CircuitMatrix;

type tuple_Array2_Array1 = (Array2<Complex64>, Array1<Complex64>);

#[derive(Debug, Copy, Clone)]
pub enum ElementType {
    V,
    C,
    R,
    L,
}

#[derive(Debug, Copy, Clone)]
pub struct Element {
    pub pos: usize,
    pub neg: usize,
    pub value: f64,
}

pub trait VoltageSource {
    fn gen_mat_vec_V(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>);
    fn ac_mat_vec_V(&mut self, elem_mat_vec: tuple_Array2_Array1,  omega: f64) -> tuple_Array2_Array1;
}

pub trait Resistor {
    fn gen_mat_vec_R(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>);
    fn ac_mat_vec_R(&mut self, elem_mat_vec: tuple_Array2_Array1,  omega: f64) -> tuple_Array2_Array1;
}

pub trait Capacitor {
    fn gen_mat_vec_C(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>);
    fn ac_mat_vec_C(&mut self, elem_mat_vec: tuple_Array2_Array1,  omega: f64) -> tuple_Array2_Array1;
}

pub trait Inductor {
    fn gen_mat_vec_L(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>);
    fn ac_mat_vec_L(&mut self, elem_mat_vec: tuple_Array2_Array1,  omega: f64) -> tuple_Array2_Array1;
}

impl VoltageSource for CircuitMatrix {
    fn gen_mat_vec_V(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>) {
        let E = elem.value;
        let a = array![
            [Complex64::new(0., 0.), Complex64::new(0., 0.), Complex64::new(1., 0.)],
            [Complex64::new(0., 0.), Complex64::new(0., 0.), Complex64::new(-1., 0.)],
            [Complex64::new(1., 0.), Complex64::new(-1., 0.), Complex64::new(0., 0.)]
        ];
        let b = arr1(&[Complex64::new(0., 0.), Complex64::new(0., 0.), Complex::new(-E, 0.)]);
        (a, b)
    }
    fn ac_mat_vec_V(&mut self, mut elem_mat_vec: tuple_Array2_Array1, omega: f64) -> tuple_Array2_Array1 {
        elem_mat_vec.1[2] = Complex64::new(-1.0, 0.);
        (elem_mat_vec.0, elem_mat_vec.1)
    }
}

impl Resistor for CircuitMatrix {
    fn gen_mat_vec_R(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>) {
        let G = elem.value;
        let a = array![
            [Complex64::new(G, 0.), Complex::new(-G, 0.)],
            [Complex64::new(-G, 0.), Complex::new(G, 0.)]];
        let b = arr1(&[Complex64::new(0., 0.), Complex64::new(0., 0.)]);
        (a, b)
    }
    fn ac_mat_vec_R(&mut self, mut elem_mat_vec: tuple_Array2_Array1, omega: f64) -> tuple_Array2_Array1 {
        (elem_mat_vec.0, elem_mat_vec.1)
    }
}

impl Capacitor for CircuitMatrix {
    fn gen_mat_vec_C(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>) {
        let C = elem.value;
        let a = array![
            [Complex64::new(C, 0.), Complex::new(-C, 0.)],
            [Complex64::new(-C, 0.), Complex::new(C, 0.)]];
        let b = arr1(&[Complex64::new(0., 0.), Complex64::new(0., 0.)]);
        (a, b)
    }
    fn ac_mat_vec_C(&mut self, mut elem_mat_vec: tuple_Array2_Array1, omega: f64) -> tuple_Array2_Array1 {
        elem_mat_vec.0 = elem_mat_vec.0.map_mut(|x| *x * Complex64::new(0., omega));
        (elem_mat_vec.0, elem_mat_vec.1)
    }
}

impl Inductor for CircuitMatrix {
    fn gen_mat_vec_L(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>) {
        let L = elem.value;
        let a = array![
            [Complex64::new(0., 0.), Complex64::new(0., 0.), Complex64::new(1., 0.)],
            [Complex64::new(0., 0.), Complex64::new(0., 0.), Complex64::new(-1., 0.)],
            [Complex64::new(1., 0.), Complex64::new(-1., 0.), Complex64::new(-L, 0.)]
        ];
        let b = arr1(&[Complex64::new(0., 0.), Complex64::new(0., 0.), Complex::new(0., 0.)]);
        (a, b)
    }
    fn ac_mat_vec_L(&mut self, mut elem_mat_vec: tuple_Array2_Array1, omega: f64) -> tuple_Array2_Array1 {
        elem_mat_vec.0[[2, 2]] *= Complex::new(0., omega);
        (elem_mat_vec.0, elem_mat_vec.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::CircuitMatrix;

    #[test]
    fn matrix() {
        unimplemented!();
    }
}
