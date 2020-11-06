use core::cmp::max;
use rand::Rng;
use AffinityClustering::affinity_clustering;
use std::fmt::{Display, Debug};
use std::cmp::Eq;
use ndarray::Array2;
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::marker::Copy;
use affinity_clustering::Edge;
use std::cmp::{PartialEq, Ord};

#[derive(Debug, Copy, Clone, Hash)]
struct Node {
    name: usize,
    weight: usize,
}

impl PartialEq for Node {
    fn eq(&self, others: &Self) -> bool {
        self.name == others.name && self.weight == others.weight
    }
}

impl Eq for Node { }
impl Display for Node { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.weight)
    }
 }

impl Node {
    fn new(Name: usize, random_weight: usize) -> Self {
        Node {
            name: Name,
            weight: random_weight,
        }
    }
}

fn make_random_graph (verticle: usize) ->  Vec::<Edge<Node>> {//随机生成一个图,矩阵中1表示存在
    //let mut data = Array2::<Node>::zeros((verticle, verticle));
    let mut i = 0;
    let mut j = 0;
    let mut edges = Vec::<Edge<Node>>::new();
    let mut random_weight = Vec::<usize>::new();
    for i in 0..verticle {
        let r_w = rand::thread_rng().gen_range(10, 20);
        random_weight.push(r_w);
    }
    while i < verticle {
        j = 0;
        while j < verticle {
            if i == j {
                //data.row_mut(i)[j] = 0;
                j += 1;
            } else if j < i {
                //data.row_mut(i)[j] = data.row(j)[i];
                j += 1;
            } else {
                let rand_number1 = rand::thread_rng().gen_range(1, j-i+1);
                if rand_number1 != 1 {
                    //data.row_mut(i)[j] = 0;
                } else {
                    edges.push(Edge {
                        start: Node::new(i, random_weight[i]),
                        end: Node::new(j, random_weight[j]),
                        weight: 1
                    });
                    edges.push(Edge {
                        start: Node::new(j, random_weight[j]),
                        end: Node::new(i, random_weight[i]),
                        weight: 1
                    })
                    }
                j += 1;
            }
        }
        i += 1;
    }
    //println!("the origional edges is {}", edges.len());
    //affinity_clustering::print_edges(&edges);
    edges
}

fn find_common_neighbors<T: Eq + Hash + Copy + Debug + Display>(edges: &Vec<Edge<T>>) -> Vec::<Edge<T>>{//转换为以common neighbor为边权的图
    let mut neighbors_of_vertex = HashMap::<T, Vec::<T>>::new();
    let mut common_neighbor_reverse = HashMap::<T, Vec::<T>>::new();
    let mut edges_of_common_neighbors = HashMap::<(T, T), usize>::new();
    let mut new_edges = Vec::<Edge<T>>::new();
    let mut hash_edges = HashSet::<(T, T)>::new();
    for e in edges {
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
        hash_edges.insert(pair);
    }

    println!("the common neighbor edges is {}", new_edges.len());
    affinity_clustering::print_edges(&new_edges);
    new_edges
}

fn find_max_size(cut_size: &Vec::<usize>) -> usize {
    let mut max_size = 0;
    for i in cut_size {
        if *i > max_size {
            max_size = *i;
        }
    }
    max_size
}

fn RankSwap(line: Vec::<Node>, k: usize, r: usize, q: Vec::<usize>) -> Vec::<Node> {
    let mut divided_line = Vec::<Vec::<Vec::<Node>>>::new();
    let mut cut_size = Vec::<usize>::new();
    //line划分为partitions， partition划分为intervals，并计算每个partition的cut size
    for i in 0..q.len()-1 {
        let mut partition_size = 0;
        let mut partition = Vec::<Vec::<Node>>::new();
        let mut interval_index = Vec::<usize>::new();
        for j in 0..r+1 {
            interval_index.push(q[i] + ((j*(q[i+1] - q[i])) as f32 / r as f32).floor() as usize);
        }
        for j in 0..r {
            let mut interval = Vec::<Node>::new();
            for k in interval_index[j]..interval_index[j+1] {
                interval.push(line[k].clone());
                partition_size += line[k].weight;
            }
            interval.sort_by_key(|x| x.weight);
            interval.reverse();
            partition.push(interval);
        }
        divided_line.push(partition);
        cut_size.push(partition_size);
    }
    println!("Present max cut size is: {}", find_max_size(&cut_size));
    println!("Begin to optimize the cut size");
    //划分完毕，开始配对
    let mut pairs = Vec::<HashMap::<usize, usize>>::new();
    for i in 0..divided_line.len()/2 {
        //随机配对相邻partition的intervals
        let mut hash_pair = HashMap::<usize, usize>::new();//intervals的配对表
        for j in 0..r {
            loop {
                let random_pair = rand::thread_rng().gen_range(0, r);
                if !hash_pair.contains_key(&random_pair) {
                    hash_pair.insert(random_pair, j);
                    break;
                }
            }
        }
        pairs.push(hash_pair);
    }
    println!("randomly pair intervals completed!");
    //配对完毕，开始计算
    let mut count = 0;
    loop {
        count += 1;
        println!("now in rounds {}", count);
        let mut flag = 0;//标志变量，为0表示此轮没有进行交换，终止循环
        for i in 0..divided_line.len()/2 {
            for (interval1, interval2) in &pairs[i] {
                for j in 0..divided_line[i*2][*interval1].len() {
                    let mut best_pair = Option::<usize>::None;
                    let mut present_small_max_size = max(cut_size[i*2], cut_size[i*2 + 1]);
                    for k in 0..divided_line[i*2+1][*interval2].len() {                                                                               
                        let imaginaty_max_size = max(cut_size[i*2] - divided_line[i*2][*interval1][j].weight + divided_line[i*2+1][*interval2][k].weight, 
                            cut_size[i*2+1] - divided_line[i*2+1][*interval2][k].weight + divided_line[i*2][*interval1][j].weight);
                        if imaginaty_max_size < present_small_max_size {//交换后能获得更小的maxsize
                            best_pair = Some(k);
                            present_small_max_size = imaginaty_max_size;
                        }
                    }
                    if let Some(real_best_pair) = best_pair {//有可供交换的best pair, 交换两点
                        flag = 1;
                        let pre_weight = divided_line[i*2+1][*interval2][real_best_pair].weight as i32;
                        let post_weight = divided_line[i*2][*interval1][j].weight as i32;
                        let difference:usize = (pre_weight - post_weight).abs() as usize;
                        if pre_weight > post_weight {//更新两个partition的cutsize
                            cut_size[i*2] += difference;
                            cut_size[i*2+1] -= difference;
                        } else {
                            cut_size[i*2] -= difference;
                            cut_size[i*2+1] += difference;
                        }
                        let temp = divided_line[i*2][*interval1][j];
                        divided_line[i*2][*interval1][j] = divided_line[i*2+1][*interval2][real_best_pair];
                        divided_line[i*2+1][*interval2][real_best_pair] = temp;
                    }
                }
            }
        }
        if flag == 0 {
            break;
        }
    }
    //输出RankSwap处理后的序列
    let mut adjusted_line = Vec::<Node>::new();
    for partition in divided_line {
        for interval in partition {
            adjusted_line.append(&mut interval.clone());
        }
    }
    println!("max cut size after optimize is: {}", find_max_size(&cut_size));
    adjusted_line
}

fn Combination(af: affinity_clustering::Affinity::<Node>, partition_number: usize, interval_len: usize) {
    let mut q = Vec::<usize>::new();
    for i in 0..partition_number+1 {
        q.push(((i*af.V.len()) as f32 / partition_number as f32).floor() as usize);
    }
    let mut line = af.linear_embed();
    let adjusted_line = RankSwap(line, partition_number, interval_len, q);
}

fn random_cluster_max_size(af: affinity_clustering::Affinity<Node>, partition_number: usize) {
    let mut line = af.linear_embed();
    let mut q = Vec::<usize>::new();
    for i in 0..partition_number+1 {
        q.push(((i*af.V.len()) as f32 / partition_number as f32).floor() as usize);
    }
    let mut divided_line_size = Vec::<usize>::new();
    for i in 0..partition_number {
        let mut partition_size = 0;
        for j in q[i]..q[i+1] {
            partition_size += line[j].weight;
        }
        divided_line_size.push(partition_size);
    }
    println!("The max cut size of ramdom divide is: {}", find_max_size(&divided_line_size));
}

fn main() {
    let mut edges = make_random_graph(100);
    let new_edges = find_common_neighbors(&edges);
    let af = affinity_clustering::make_cluster(0.4, new_edges, 30, true, true);
    let unclustering_af = affinity_clustering::Affinity::<Node>::new_and_init(edges, 10);
    random_cluster_max_size(unclustering_af, 10);
    Combination(af,10, 3);
}
