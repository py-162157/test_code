use rand::Rng;
use AffinityClustering::affinity_clustering;
use std::fmt::{Display, Debug};
use std::cmp::Eq;
use ndarray::Array2;
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::marker::Copy;
use affinity_clustering::Edge;


fn make_random_graph_matrix (verticle: usize) -> (Array2::<usize>, Vec::<Edge<usize>>) {//随机生成一个图,矩阵中1表示存在
    let mut data = Array2::<usize>::zeros((verticle, verticle));
    let mut i = 0;
    let mut j = 0;
    let mut edges = Vec::<Edge<usize>>::new();
    while i < verticle {
        j = 0;
        while j < verticle {
            if i == j {
                data.row_mut(i)[j] = 0;
                j += 1;
            } else if j < i {
                data.row_mut(i)[j] = data.row(j)[i];
                j += 1;
            } else {
                let rand_number1 = rand::thread_rng().gen_range(1, j-i+1);
                if rand_number1 != 1 {
                    data.row_mut(i)[j] = 0;
                } else {
                    edges.push(Edge {
                        start: i,
                        end: j,
                        weight: 1
                    });
                    edges.push(Edge {
                        start: j,
                        end: i,
                        weight: 1
                    })
                    }
                j += 1;
            }
        }
        i += 1;
    }
    //println!("the origional edges is");
    affinity_clustering::print_edges(&edges);
    (data, edges)
}

fn find_common_neighbors<T: Eq + Hash + Copy + Debug + Display>(edges: Vec<Edge<T>>) -> Vec::<Edge<T>>{//转换为以common neighbor为边权的图
    let mut neighbors_of_vertex = HashMap::<T, Vec::<T>>::new();
    let mut common_neighbor_reverse = HashMap::<T, Vec::<T>>::new();
    let mut edges_of_common_neighbors = HashMap::<(T, T), usize>::new();
    let mut new_edges = Vec::<Edge<T>>::new();
    let mut hash_edges = HashSet::<(T, T)>::new();
    for e in edges.iter() {
        if neighbors_of_vertex.contains_key(&e.start) {
            neighbors_of_vertex.get_mut(&e.start).unwrap().push(e.end);
        } else {
            neighbors_of_vertex.insert(e.start, vec!(e.end));
        }
    }
    for (vertex, neighbors) in neighbors_of_vertex {
        for neighbor in neighbors {
            if common_neighbor_reverse.contains_key(&neighbor) {
                common_neighbor_reverse.get_mut(&neighbor).unwrap().push(vertex);
            } else {
                common_neighbor_reverse.insert(neighbor, vec!(vertex));
            }
        }
    }
    for (beighbor, vertexs) in common_neighbor_reverse {
        for vertex1 in vertexs.iter() {
            for vertex2 in vertexs.iter() {
                if vertex1 != vertex2 {
                    if edges_of_common_neighbors.contains_key(&(*vertex1, *vertex2)) {
                        *edges_of_common_neighbors.get_mut(&(*vertex1, *vertex2)).unwrap() += 1;
                    } else {
                        edges_of_common_neighbors.insert((*vertex1, *vertex2), 1);
                    }
                }
            }
        }
    }
    for (pair, weight) in edges_of_common_neighbors {
        new_edges.push(Edge {
            start: pair.0,
            end: pair.1,
            weight: weight,
        });
        hash_edges.insert((pair));
    }

    //println!("the common neighbor edges is");
    affinity_clustering::print_edges(&new_edges);
    new_edges
}

fn RankSwap<T>(line: Vec::<T>, edges: HashSet::<(T, T)>, k: usize, q: Vec::<usize>) {
    
}

fn Combination<T>(af: affinity_clustering::Affinity::<T>, k: usize, matrix: Array2::<T>)
    where T: Eq + Hash + Copy + Debug + Display {
    let mut q = Vec::<usize>::new();
    for i in 0..k+1 {
        q.push(((i*af.V.len()) as f32 / k as f32).floor() as usize);
    }
    let mut line = af.linear_embed();
    loop {

    }
}



fn main() {
    let (matrix, mut edges) = make_random_graph_matrix(100);
    let new_edges = find_common_neighbors(edges);
    let af = affinity_clustering::make_cluster(0.4, new_edges, 10, true, true);

}
