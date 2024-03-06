#![warn(unused_variables)]
#![warn(dead_code)]
#![allow(non_snake_case)]

use num_complex::{Complex, Complex64};
use ndarray::*;

use crate::matrix::CircuitMatrix;

type TuppleArray1Array2 = (Array2<Complex64>, Array1<Complex64>);

#[derive(Debug, Copy, Clone)]
pub enum ElementType {
    V,
    I,
    C,
    R,
    L,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub pos: String,
    pub neg: String,
    pub value: f64,
    pub kind: ElementType,
}

pub trait VoltageSource {
    fn gen_mat_vec_V(&mut self, elem: Element) -> TuppleArray1Array2;
    fn ac_mat_vec_V(&mut self, elem_mat_vec: TuppleArray1Array2,  omega: f64) -> TuppleArray1Array2;
}

pub trait CurrentSource {
    fn gen_mat_vec_I(&mut self, elem: Element) -> TuppleArray1Array2;
    fn ac_mat_vec_I(&mut self, elem_mat_vec: TuppleArray1Array2,  omega: f64) -> TuppleArray1Array2;
}

pub trait Resistor {
    fn gen_mat_vec_R(&mut self, elem: Element) -> TuppleArray1Array2;
    fn ac_mat_vec_R(&mut self, elem_mat_vec: TuppleArray1Array2,  omega: f64) -> TuppleArray1Array2;
}

pub trait Capacitor {
    fn gen_mat_vec_C(&mut self, elem: Element) -> TuppleArray1Array2;
    fn ac_mat_vec_C(&mut self, elem_mat_vec: TuppleArray1Array2,  omega: f64) -> TuppleArray1Array2;
}

pub trait Inductor {
    fn gen_mat_vec_L(&mut self, elem: Element) -> TuppleArray1Array2;
    fn ac_mat_vec_L(&mut self, elem_mat_vec: TuppleArray1Array2,  omega: f64) -> TuppleArray1Array2;
}

impl VoltageSource for CircuitMatrix {
    fn gen_mat_vec_V(&mut self, elem: Element) -> TuppleArray1Array2 {
        let E = elem.value;
        let a = array![
            [Complex64::new(0., 0.), Complex64::new(0., 0.), Complex64::new(1., 0.)],
            [Complex64::new(0., 0.), Complex64::new(0., 0.), Complex64::new(-1., 0.)],
            [Complex64::new(1., 0.), Complex64::new(-1., 0.), Complex64::new(0., 0.)]
        ];
        let b = arr1(&[Complex64::new(0., 0.), Complex64::new(0., 0.), Complex::new(-E, 0.)]);
        (a, b)
    }

    fn ac_mat_vec_V(&mut self, mut elem_mat_vec: TuppleArray1Array2, omega: f64) -> TuppleArray1Array2 {
        elem_mat_vec.1[2] = Complex64::new(-1.0, 0.);
        (elem_mat_vec.0, elem_mat_vec.1)
    }
}

impl CurrentSource for CircuitMatrix {
    fn gen_mat_vec_I(&mut self, elem: Element) -> TuppleArray1Array2 {
        let I = elem.value;
        let a = array![
            [Complex64::new(0., 0.), Complex::new(0., 0.)],
            [Complex64::new(0., 0.), Complex::new(0., 0.)]];
        let b = arr1(&[Complex64::new(I, 0.), Complex64::new(-I, 0.)]);
        (a, b)
    }

    fn ac_mat_vec_I(&mut self, mut elem_mat_vec: TuppleArray1Array2, omega: f64) -> TuppleArray1Array2 {
        elem_mat_vec.1[0] = Complex64::new(1.0, 0.);
        elem_mat_vec.1[1] = Complex64::new(-1.0, 0.);
        (elem_mat_vec.0, elem_mat_vec.1)
    }
}

impl Resistor for CircuitMatrix {
    fn gen_mat_vec_R(&mut self, elem: Element) -> TuppleArray1Array2 {
        let G = 1.0 / elem.value;
        let a = array![
            [Complex64::new(G, 0.), Complex::new(-G, 0.)],
            [Complex64::new(-G, 0.), Complex::new(G, 0.)]];
        let b = arr1(&[Complex64::new(0., 0.), Complex64::new(0., 0.)]);
        (a, b)
    }

    fn ac_mat_vec_R(&mut self, mut elem_mat_vec: TuppleArray1Array2, omega: f64) -> TuppleArray1Array2 {
        (elem_mat_vec.0, elem_mat_vec.1)
    }
}

impl Capacitor for CircuitMatrix {
    fn gen_mat_vec_C(&mut self, elem: Element) -> TuppleArray1Array2 {
        let C = elem.value;
        let a = array![
            [Complex64::new(C, 0.), Complex::new(-C, 0.)],
            [Complex64::new(-C, 0.), Complex::new(C, 0.)]];
        let b = arr1(&[Complex64::new(0., 0.), Complex64::new(0., 0.)]);
        (a, b)
    }

    fn ac_mat_vec_C(&mut self, mut elem_mat_vec: TuppleArray1Array2, omega: f64) -> TuppleArray1Array2 {
        elem_mat_vec.0 = elem_mat_vec.0.map_mut(|x| *x * Complex64::new(0., omega));
        (elem_mat_vec.0, elem_mat_vec.1)
    }
}

impl Inductor for CircuitMatrix {
    fn gen_mat_vec_L(&mut self, elem: Element) -> TuppleArray1Array2 {
        let L = elem.value;
        let a = array![
            [Complex64::new(0., 0.), Complex64::new(0., 0.), Complex64::new(1., 0.)],
            [Complex64::new(0., 0.), Complex64::new(0., 0.), Complex64::new(-1., 0.)],
            [Complex64::new(1., 0.), Complex64::new(-1., 0.), Complex64::new(-L, 0.)]
        ];
        let b = arr1(&[Complex64::new(0., 0.), Complex64::new(0., 0.), Complex::new(0., 0.)]);
        (a, b)
    }

    fn ac_mat_vec_L(&mut self, mut elem_mat_vec: TuppleArray1Array2, omega: f64) -> TuppleArray1Array2 {
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
