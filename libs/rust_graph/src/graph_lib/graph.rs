// Implementar lista de adjacência em grafos

use super::busca::*;
use scan_fmt::scan_fmt;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::{collections::HashMap, fs};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    destiny_key: i32,
    origin_key: i32,
    id: usize,
}

impl Edge {
    pub fn new(origin_vertice: i32, destiny_vertice: i32) -> Edge {
        static mut EDGE_COUNTER: i32 = 0;
        let mut id = 0;
        unsafe {
            id = EDGE_COUNTER;
            EDGE_COUNTER += 1;
        }
        Edge {
            id: id as usize,
            destiny_key: destiny_vertice,
            origin_key: origin_vertice,
        }
    }

    pub fn get_destiny_key(&self) -> i32 {
        self.destiny_key
    }
    pub fn get_origin_key(&self) -> i32 {
        self.origin_key
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.destiny_key.cmp(&other.destiny_key)
    }
}
impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.origin_key, self.destiny_key)
    }
}
///# Vertice
/// Estrutura destinada a representar vertices em um grafo
///
/// contém campos como `key` e `edges`
pub struct Vertice {
    key: i32,
    edges: Vec<Edge>,
}
impl Vertice {
    pub fn new(vertice_key: i32) -> Vertice {
        Vertice {
            key: vertice_key,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, destiny_key: i32) {
        self.edges
            .insert(self.edges.len(), Edge::new(self.key, destiny_key));
    }
    ///Clona o vetor de arestas do vértice
    ///
    pub fn get_Edges_clone(&self) -> Vec<Edge> {
        self.edges.clone()
    }
    pub fn get_Edges_ref(&self) -> &Vec<Edge> {
        &self.edges
    }
}

/// # DiGraph
/// Grafo direcionado representado em lista de Adjacência
///
/// `vertices_num` quantidade de vértices em um grafo
///
/// `edges_num` quantidade de arestas em um grafo
///
/// `Vertices` HashSet para encontrar vértices usando sua key em O(1)
pub struct DiGraph {
    vertices_num: u32,
    edges_num: u32,
    vertices: HashMap<i32, Rc<RefCell<Vertice>>>,
}

impl DiGraph {
    pub fn new(vertice_num: u32, edge_num: u32) -> DiGraph {
        DiGraph {
            vertices_num: vertice_num,
            edges_num: edge_num,
            vertices: HashMap::new(),
        }
    }

    pub fn get_vertices_length(&self) -> u32 {
        return self.vertices_num;
    }

    pub fn get_edges_lenght(&self) -> u32 {
        return self.edges_num;
    }

    pub fn get_vertice_key_array(&self) -> Vec<i32> {
        let mut vertice_array: Vec<i32> = Vec::with_capacity(self.vertices.len());
        for (vert_key, _) in self.vertices.iter() {
            vertice_array.insert(vertice_array.len(), *vert_key);
        }
        return vertice_array;
    }

    pub fn get_vertice_cloneRef(&self, vertice_key: i32) -> Option<Rc<RefCell<Vertice>>> {
        let vertice = self.vertices.get(&vertice_key);
        if (vertice.is_none()) {
            return None;
        }
        Some(vertice.unwrap().clone())
    }

    /// ## Verifica existência de um vértice no grafo
    ///
    /// `true` se existe
    ///
    /// `false` se não existe
    pub fn vertice_exists(&self, vert_key: i32) -> bool {
        !self.vertices.get(&vert_key).is_none()
    }

    pub fn add_vertice(&mut self, vertice_key: i32) {
        let vertice = Vertice::new(vertice_key);
        let vertice = Rc::new(RefCell::new(vertice));
        self.vertices.insert(vertice_key, vertice);
    }

    pub fn add_edge(&mut self, origin_vert: i32, destiny_vert: i32) {
        if !self.vertice_exists(origin_vert) {
            self.add_vertice(origin_vert);
        }
        if !self.vertice_exists(destiny_vert) {
            self.add_vertice(destiny_vert);
        }

        // Obtém o `Rc<RefCell<Vertice>>` referente ao vértice de origem
        let mut vertice_origem = self
            .vertices
            .get(&origin_vert)
            .unwrap()
            .try_borrow_mut()
            .unwrap();
        vertice_origem.add_edge(destiny_vert); // precisa ser mutavel
    }

    pub fn from_file(file_path: &str) -> DiGraph {
        let file_content = fs::read_to_string(file_path).unwrap();

        let mut lines = file_content.lines(); //iterador do arquivo

        let (vert_num, edge_num) = scan_fmt!(lines.next().unwrap(), "{} {}", u32, u32).unwrap();
        let mut graph = DiGraph::new(vert_num, edge_num);

        for line in lines {
            let (orig, dest) = scan_fmt!(line, "{} {}", i32, i32).unwrap();
            graph.add_edge(orig, dest);
        }
        graph
    }

    /// retorna as chaves dos sucessores do vértice
    ///
    pub fn get_sucessor(&self, vertice_key: i32) -> Option<Vec<i32>> {
        let vertice: Option<Rc<RefCell<Vertice>>> = self.get_vertice_cloneRef(vertice_key);
        if vertice.is_none() {
            return None;
        }

        let vert_ref = vertice.unwrap();

        let vertice = vert_ref.borrow();
        let arestas: &Vec<Edge> = vertice.get_Edges_ref();

        let mut sucessors: Vec<i32> = Vec::new();
        for aresta in arestas.iter() {
            sucessors.insert(sucessors.len(), aresta.destiny_key);
        }
        return Some(sucessors);
    }

    /// retorna um conjunto clonado de arestas do vértice
    pub fn get_edges(&self, vertice_key: i32) -> Option<Vec<Edge>> {
        let vertice: Option<Rc<RefCell<Vertice>>> = self.get_vertice_cloneRef(vertice_key);
        if vertice.is_none() {
            return None;
        }

        let vert_ref = vertice.unwrap();

        let vertice = vert_ref.borrow();
        Some(vertice.get_Edges_clone())
    }

    // retorna sa chaves dos predecessores do vértice
    ///
    pub fn get_predecessor(&self, vertice_key: i32) -> Option<Vec<i32>> {
        let mut vertice: Option<Rc<RefCell<Vertice>>> = self.get_vertice_cloneRef(vertice_key);
        if vertice.is_none() {
            return None;
        }
        let mut list: Vec<i32> = Vec::new();
        for (vert_key, vertice_ref) in self.vertices.iter() {
            let vertice_ref = vertice_ref.borrow();
            for aresta in vertice_ref.edges.iter() {
                if aresta.destiny_key == vertice_key {
                    list.insert(list.len(), aresta.origin_key);
                }
            }
        }

        return Some(list);
    }

    pub fn dfs_search(&self, mut search_key: i32) -> DfsStruct {
        let mut dfs_data = DfsStruct::new(self);
        while search_key != -1 {
            self.explore_dfs_vertice(search_key, &mut dfs_data);
            search_key = dfs_data.get_unexplored_vertice(self);
        }
        return dfs_data;
    }
    fn explore_dfs_vertice(
        &self,
        search_key: i32,
        dfs_data: &mut DfsStruct,
    ) {
        let mut stack: Vec<i32> = Vec::new();

        stack.push(search_key);

        while stack.len() > 0 {
            let vertice_key = stack.pop().unwrap();

            if !dfs_data.already_visited(vertice_key) {
                dfs_data.start_exploring(vertice_key);
            }
            let mut arestas: Option<Vec<Edge>> = self.get_edges(vertice_key);

            let Some(mut arestas) = arestas else {
                dfs_data.finish_exploring(vertice_key);
                stack.pop();
                continue;
            };
            arestas.sort();
            let mut descobriu_vertice = false;

            for aresta in arestas {
                if dfs_data.is_aresta_marked(aresta.id as i32) {
                    continue; //aresta ja classificada
                }
                dfs_data.arestas_marked.insert(aresta.id as i32, true); // marca a aresta que está sendo explorada
                if !dfs_data.already_visited(aresta.destiny_key) {
                    // não foi descoberto ainda, arvore
                    dfs_data.fathers.insert(aresta.destiny_key, vertice_key);
                    stack.push(aresta.destiny_key); // empilha o vertice
                    dfs_data.classificate_aresta(&aresta, DfsClassification::Arvore);
                    descobriu_vertice = true;
                    break;
                }
                if !dfs_data.already_explored(aresta.destiny_key) {
                    //se ainda n finalizou, é retorno
                    dfs_data.classificate_aresta(&aresta, DfsClassification::Retorno);
                } else if dfs_data.tempo_descoberta.get(&vertice_key).unwrap()
                    < dfs_data.tempo_descoberta.get(&aresta.destiny_key).unwrap()
                {
                    //se já finalizou a busca, mas ele é mais novo q o vertice_key, é avanço
                    dfs_data.classificate_aresta(&aresta, DfsClassification::Avanco);
                } else {
                    //se já finalizou a busca, mas ele é mais velho q o vertice_key, é cruzamento
                    dfs_data.classificate_aresta(&aresta, DfsClassification::Cruzamento);
                }
            }
            if !descobriu_vertice {
                dfs_data.finish_exploring(vertice_key);
                stack.pop();
            }
        }
    }
}
