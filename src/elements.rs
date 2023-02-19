#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]

use num_complex::{Complex, Complex64};
use ndarray::*;

use crate::matrix::CircuitMatrix;

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
    fn ac_mat_vec_V(&mut self, elem_mat: Array2<Complex64>,  omega: f64) -> (Array2<Complex64>);
}

pub trait Resistor {
    fn gen_mat_vec_R(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>);
    fn ac_mat_vec_R(&mut self, elem_mat: Array2<Complex64>,  _omega: f64) -> (Array2<Complex64>);
}

pub trait Capacitor {
    fn gen_mat_vec_C(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>);
    fn ac_mat_vec_C(&mut self, elem_mat: Array2<Complex64>,  omega: f64) -> (Array2<Complex64>);
}

pub trait Inductor {
    fn gen_mat_vec_L(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>);
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
    fn ac_mat_vec_V(&mut self, mut elem_mat: Array2<Complex64>, omega: f64) -> Array2<Complex64> {
        elem_mat.map_mut(|x| *x * Complex64::new(0., omega)) //TODO
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
    fn ac_mat_vec_R(&mut self, mut elem_mat: Array2<Complex64>, omega: f64) -> Array2<Complex64> {
        elem_mat.map_mut(|x| *x * Complex64::new(0., omega)) //TODO
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
    fn ac_mat_vec_C(&mut self, mut elem_mat: Array2<Complex64>, omega: f64) -> Array2<Complex64> {
        elem_mat.map_mut(|x| *x * Complex64::new(0., omega)) //TODO
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::CircuitMatrix;

    #[test]
    fn matrix() {
        let c1_element = Element { pos: 1, neg: 2, value: 3.0 };
        let mut matrix = CircuitMatrix::new();
        let (c_matrix, c_vector) = matrix.gen_mat_vec_C(c1_element);
        assert_eq!(matrix.nodes, array![1, 2]);
        let c2_element = Element { pos: 3, neg: 4, value: 3.0 };
        let (c_matrix, c_vector) = matrix.gen_mat_vec_C(c2_element);
        assert_eq!(matrix.nodes, array![1, 2, 3, 4]);
    }
}
