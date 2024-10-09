use std::collections::HashMap;

use crate::DiGraph;


use super::super::tools::heap::HeapMin;

use super::vertice::Vertice;
use super::super::tools::inifinity::Infinity;

pub struct Bellman {
    pred: HashMap<i32, i32>,
    pot: HashMap<i32, Infinity>,
}
#[allow(unused)]
impl Bellman {
    pub fn new() -> Bellman {
        Bellman {
            pred: HashMap::new(),
            pot: HashMap::new(),
        }
    }

    pub fn pred(&self) -> &HashMap<i32, i32> {
        &self.pred
    }
    pub fn pot(&self) -> &HashMap<i32, Infinity> {
        &self.pot
    }

   
}


use Infinity::{Infinite, Number};
#[allow(unused)]
pub fn find_shortest_path(graph: &DiGraph, start: i32) -> Bellman {
    let mut data = Bellman::new();

    for v in graph.iter_vertices() {
        let v = v.read().unwrap();
        data.pot.insert(v.key(), Infinite);
        data.pred.insert(v.key(), -1);
    }
 
    data.pot.insert(start, Number(0));
    for _ in 0..graph.get_vertices_length() {
        let mut change = false;
        for v in graph.iter_vertices() {
            let v = v.read().unwrap();
            for e in v.edges_borrow() {
                let w = e.destiny_key();
                let v = v.key();
                let v_d = *data.pot.get(&v).unwrap();
                let w_d = *data.pot.get(&w).unwrap();
                if w_d > (v_d + Number(e.weight())) {
                    data.pot.insert(w, Number(v_d.unwrap() + e.weight()));
                    data.pred.insert(w, v);
                    change = true;
                }
            }
        }
        if !change {
            break;
        }
    }

    data
}


