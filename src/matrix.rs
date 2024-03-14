#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]

use anyhow::Result;
use num_complex::Complex64;
use ndarray::*;
use ndarray_linalg::*;

use crate::netlist::Netlist;
use crate::elements::{Element, ElementType};
use crate::elements::{VoltageSource, CurrentSource, Capacitor, Resistor, Inductor};

pub enum Analysis {
    _DC, //TODO
    AC,
    _Tran, //TODO
}

trait ExtendWith0 {
    fn extend_with0(&mut self) -> Result<()> ;
}

impl ExtendWith0 for ArrayBase<OwnedRepr<Complex64>, Dim<[usize; 2]>> {
    fn extend_with0(&mut self) -> Result<()> {
        let shape = self.shape();
        let (m, n) = (shape[0], shape[1]);
        self.push_column(ArrayView::from(&vec![Complex64::new(0., 0.); m])).ok();
        self.push_row(ArrayView::from(&vec![Complex64::new(0., 0.); n+1])).ok();
        Ok(())
    }
}

impl ExtendWith0 for ArrayBase<OwnedRepr<Complex64>, Dim<[usize; 1]>> {
    fn extend_with0(&mut self) -> Result<()> {
        self.append(Axis(0), ArrayView::from(&vec![Complex64::new(0., 0.); 1]))?;
        Ok(())
    }
}

pub fn netlist_serialize(netlist: &Netlist) -> Vec<&Element> {
    let mut ret_vec = Vec::new();
    let _: Vec<_> = netlist.v.iter().map(|x| ret_vec.push(x)).collect();
    let _: Vec<_> = netlist.i.iter().map(|x| ret_vec.push(x)).collect();
    let _: Vec<_> = netlist.r.iter().map(|x| ret_vec.push(x)).collect();
    let _: Vec<_> = netlist.c.iter().map(|x| ret_vec.push(x)).collect();
    let _: Vec<_> = netlist.l.iter().map(|x| ret_vec.push(x)).collect();

    ret_vec
}

// TODO specific GND remove by user setting
fn remove_ground_from_array(matrix: &Array2<Complex64>, idx_gnd: usize) -> Array2<Complex64> {
    let mut new_matrix: Array2<Complex64> = Array2::from_elem((matrix.nrows() - 1, matrix.ncols() - 1), Complex64::new(0., 0.));
    for row in 0..idx_gnd {
        for col in 0..idx_gnd {
            new_matrix[[row , col]] = matrix[[row, col]];
        }
    }
    for row in idx_gnd+1..matrix.nrows() {
        for col in idx_gnd+1..matrix.ncols() {
            new_matrix[[row-1 , col-1]] = matrix[[row, col]];
        }
    }
    new_matrix
}

fn remove_ground_from_vector(vector: &Array1<Complex64>, idx_gnd: usize) -> Array1<Complex64> {
    let mut new_vector: Array1<Complex64> = Array1::from_elem(vector.len() - 1, Complex64::new(0., 0.));
    for i in 0..idx_gnd {
        new_vector[i] = vector[i].clone();
    }
    for i in idx_gnd+1..vector.len() {
        new_vector[i-1] = vector[i].clone();
    }
    new_vector
}

fn remove_ground_from_nodes(nodes: &Array1<String>, gnd: String) -> (Array1<String>, usize) {
    let mut idx_gnd: usize = 0;
    for (i, n) in nodes.iter().enumerate() {
        if n == &gnd {
            idx_gnd == i;
        }
    }
    let empty_value: String = "0".to_string();
    let mut new_nodes: Array1<String> = Array1::from_elem(1, empty_value);
    let empty_value: String = "0".to_string();
    if !nodes.is_empty() {
        new_nodes = Array1::from_elem(nodes.len() - 1, empty_value);
    } else {
        println!("[matrix.rs]: nodes: {:?}", nodes);
    }
    for i in 0..idx_gnd {
        new_nodes[i] = nodes[i].clone();
    }
    for i in idx_gnd+1..nodes.len() {
        new_nodes[i-1] = nodes[i].clone();
    }
    (new_nodes, idx_gnd)
}

#[derive(Debug, Clone, PartialEq)]
pub struct CircuitMatrix {
    pub mat: Array2<Complex64>,
    pub vec: Array1<Complex64>,
    pub nodes: Array1<String>,
}

impl CircuitMatrix {
    pub fn new() -> Self {
        // 初期化された最小の２行２列のマトリックスと２要素の配列を返す
        let a = array![
            [Complex64::new(0., 0.), Complex64::new(0., 0.)], 
            [Complex64::new(0., 0.), Complex64::new(0., 0.)]
        ];
        let b = array![Complex64::new(0., 0.), Complex64::new(0., 0.)];
        let c = array!["initial_node0".to_string(), "initial_node1".to_string()];
        CircuitMatrix { mat: a, vec: b, nodes: c }
    }

    // main method
    pub fn create_mat_vec(&mut self, netlist: &Netlist, analysis: Analysis, omega: f64, gnd: &str) -> Result<()> {
        let elements = netlist_serialize(netlist);
        for elem in elements.iter() {
            self.update_nodes(elem.pos.clone(), elem.neg.clone(), elem.kind)?;

            let mut elem_mat_vec = self.gen_mat_vec((*elem).clone());
            match analysis {
                Analysis::_DC => (),
                Analysis::AC => { elem_mat_vec = self.ac_mat_vec(elem_mat_vec, elem.kind, omega) },
                Analysis::_Tran => unimplemented!(),
            }

            let extended_elem_mat_vec = self.extend_elem_mat_vec(
                elem_mat_vec,
                elem.pos.clone(),
                elem.neg.clone()
            );
            self.add_mat_vec(&extended_elem_mat_vec);
        }
        self.remove_ground(gnd);
        Ok(())
    }

    pub fn gen_mat_vec(&mut self, elem: Element) -> (Array2<Complex64>, Array1<Complex64>){
        match elem.kind {
            ElementType::V => self.gen_mat_vec_V(elem),
            ElementType::I => self.gen_mat_vec_I(elem),
            ElementType::C => self.gen_mat_vec_C(elem),
            ElementType::R => self.gen_mat_vec_R(elem),
            ElementType::L => self.gen_mat_vec_L(elem),
        }
    }

    fn extend_elem_mat_vec(&mut self, elem_mat_vec: (Array2<Complex64>, Array1<Complex64>), pos: String, neg: String) -> (Array2<Complex64>, Array1<Complex64>) {
        // 要素０の行列とベクトルを生成
        let mut mat: Array2<Complex64> = Array::zeros((self.nodes.len(), self.nodes.len()));
        let mut vec: Array1<Complex64> = Array::zeros(self.nodes.len());
        // nodes から素子の接続されている要素インデックスを取得
        let mut node_i1: usize = 0;
        let mut node_i2: usize = 0;
        for (i, node) in self.nodes.iter().enumerate() {
            if node == &pos {
               node_i1 += i;
            } else if node == &neg{
               node_i2 += i;
            }
        }
        // 要素に加算
        if elem_mat_vec.1.len() == 2 {
        // 2行2列の場合
            let elem_mat = elem_mat_vec.0;
            mat[[node_i1, node_i1]] += elem_mat[[0, 0]];
            mat[[node_i1, node_i2]] += elem_mat[[0, 1]];
            mat[[node_i2, node_i1]] += elem_mat[[1, 0]];
            mat[[node_i2, node_i2]] += elem_mat[[1, 1]];

            let elem_vec = elem_mat_vec.1;
            vec[node_i1] += elem_vec[0];
            vec[node_i2] += elem_vec[1];
        } else if elem_mat_vec.1.len() == 3 {
        // 3行3列の場合
            let elem_mat = elem_mat_vec.0;
            let node_i3 = self.nodes.len() - 1;
            mat[[node_i1, node_i1]] += elem_mat[[0, 0]];
            mat[[node_i1, node_i2]] += elem_mat[[0, 1]];
            mat[[node_i1, node_i3]] += elem_mat[[0, 2]];
            mat[[node_i2, node_i1]] += elem_mat[[1, 0]];
            mat[[node_i2, node_i2]] += elem_mat[[1, 1]];
            mat[[node_i2, node_i3]] += elem_mat[[1, 2]];
            mat[[node_i3, node_i1]] += elem_mat[[2, 0]];
            mat[[node_i3, node_i2]] += elem_mat[[2, 1]];
            mat[[node_i3, node_i3]] += elem_mat[[2, 2]];

            let elem_vec = elem_mat_vec.1;
            vec[node_i1] += elem_vec[0];
            vec[node_i2] += elem_vec[1];
            vec[node_i3] += elem_vec[2];
        }
        (mat, vec)
    }

    fn update_nodes(&mut self, pos: String, neg: String, etype: ElementType) -> Result<()> {
        // self.nodesを素子のつながっているノードを見て更新する
        let is_pos = self.nodes.iter().any(|node| node==&pos);
        let is_neg = self.nodes.iter().any(|node| node==&neg);
        if !is_pos {
            self.nodes.append(
                Axis(0),
                ArrayView1::from(&[pos])
            )?;
            self.mat.extend_with0()?;
            self.vec.extend_with0()?;
        }
        if !is_neg {
            self.nodes.append(
                Axis(0),
                ArrayView1::from(&[neg])
            )?;
            self.mat.extend_with0()?;
            self.vec.extend_with0()?;
        }
        // V or L は電流ノードの追加が必要
        if !is_pos || !is_neg {
            match etype {
                ElementType::V => { 
                    self.nodes.append(
                        Axis(0),
                        ArrayView1::from(&[String::from("i999")])
                    )?;
                    self.mat.extend_with0()?;
                    self.vec.extend_with0()?;
                },
                ElementType::L => { 
                    self.nodes.append(
                        Axis(0),
                        ArrayView1::from(&[String::from("i999")])
                    )?;
                    self.mat.extend_with0()?;
                    self.vec.extend_with0()?;
                },
                _ => (),
            }
        }

        // if not eliminated initial node
        let is_initial = self.nodes.iter().any(|node| node==&"initial_node0".to_string()) && self.nodes.iter().any(|node| node==&"initial_node1".to_string());
        if is_initial {
            let mut idx;
            (self.nodes, idx) = remove_ground_from_nodes(&self.nodes.clone(), "initial_node0".to_string());
            self.vec = remove_ground_from_vector(&self.vec.clone(), idx);
            self.mat = remove_ground_from_array(&self.mat.clone(), idx);
            (self.nodes, idx) = remove_ground_from_nodes(&self.nodes.clone(), "initial_node1".to_string());
            self.vec = remove_ground_from_vector(&self.vec.clone(), idx);
            self.mat = remove_ground_from_array(&self.mat.clone(), idx);
        }
        Ok(())
    }

    fn ac_mat_vec(&mut self, elem_mat_vec: (Array2<Complex64>, Array1<Complex64>), etype: ElementType, omega: f64) -> (Array2<Complex64>, Array1<Complex64>) {
        let (mat, vec) = match etype {
            ElementType::V => self.ac_mat_vec_V(elem_mat_vec, omega),
            ElementType::I => self.ac_mat_vec_I(elem_mat_vec, omega),
            ElementType::C => self.ac_mat_vec_C(elem_mat_vec, omega),
            ElementType::R => self.ac_mat_vec_R(elem_mat_vec, omega),
            ElementType::L => self.ac_mat_vec_L(elem_mat_vec, omega),
        };
        (mat, vec)
    }

    pub fn remove_ground(&mut self, gnd: &str) {
        let idx_gnd: usize;
        (self.nodes, idx_gnd) = remove_ground_from_nodes(&self.nodes, gnd.to_string());
        self.mat = remove_ground_from_array(&self.mat, idx_gnd);
        self.vec = remove_ground_from_vector(&self.vec, idx_gnd);
    }

    fn add_mat_vec(&mut self, mat_vec: &(Array2<Complex64>, Array1<Complex64>)) -> () {
        self.mat = &self.mat + mat_vec.0.clone();
        self.vec = &self.vec + mat_vec.1.clone();
    }

    pub fn get_current_mat_vec(&self) -> (Array2<Complex64>, Array1<Complex64>) {
        (self.mat.clone(), self.vec.clone())
    }

    pub fn get_current_nodes(&self) -> Array1<String> {
        self.nodes.clone()
    }

    pub fn get_number_of_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn solve(&self) -> Result<Array1<Complex64>> {
        let result: Array1<Complex64> = self.mat.solve_into(self.vec.clone())?;
        Ok(result)
    }
  }

#[cfg(test)]
mod tests {
    use super::*;

    use crate::netlist::Netlist;
    use crate::elements::Element;

    #[test]
    fn nodes() {
        let v1_element = Element {
            name: "v1".to_string(),
            pos: "0".to_string(),
            neg: "1".to_string(),
            value: 3.0,
            kind: ElementType::V,
        };
        let c1_element = Element {
            name: "c1".to_string(),
            pos: "1".to_string(),
            neg: "2".to_string(),
            value: 1.0e-6,
            kind: ElementType::C,
        };
        let r1_element = Element {
            name: "r1".to_string(),
            pos: "2".to_string(),
            neg: "0".to_string(),
            value: 1.0,
            kind: ElementType::R,
        };
        let mut netlist = Netlist {
            v: Vec::new(),
            i: Vec::new(),
            r: Vec::new(),
            c: Vec::new(),
            l: Vec::new(),
        };
        netlist.v.push(v1_element);
        netlist.c.push(c1_element);
        netlist.r.push(r1_element);

        let mut matrix = CircuitMatrix::new();
        let freq: f64 = 1.0;
        let gnd = "0";
        let _ = matrix.create_mat_vec(&netlist, Analysis::AC, freq, gnd);

        assert_eq!(
            array!["1".to_string(), "i999".to_string(), "2".to_string()],
            matrix.get_current_nodes()
        );

    }
}
