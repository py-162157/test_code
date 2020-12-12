use core::cmp::max;
use rand::Rng;
use AffinityClustering::affinity_clustering;
use std::fmt::{Display, Debug};
use std::cmp::{Eq};
use ndarray::{Array3, ArrayBase, Array2};
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::marker::Copy;
use affinity_clustering::Edge;
use std::cmp::{PartialEq, Ord};
extern crate stopwatch;
use stopwatch::{Stopwatch};

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

fn make_fat_tree(k: usize) -> Vec::<Edge<Node>> {
    let mut edges = Vec::<Edge<Node>>::new();
    let mut kernel_node = Vec::<Node>::new();
    let mut pods = Vec::<(Vec::<Node>, Vec::<Node>)>::new();
    let mut server_node = Vec::<Vec::<Vec<Node>>>::new();
    let mut count = 0;
    //生成节点
    for _ in 0..k*k/4 {
        let r_w = rand::thread_rng().gen_range(3, 6);
        let node = Node::new(count, r_w);
        kernel_node.push(node);
        count += 1;
    }
    for _ in 0..k {
        let mut pod = (Vec::<Node>::new(), Vec::<Node>::new());
        for _ in 0..k/2 {
            let r_w = rand::thread_rng().gen_range(3, 6);
            let node = Node::new(count, r_w);
            pod.0.push(node);
            count += 1;
        }
        for _ in 0..k/2 {
            let r_w = rand::thread_rng().gen_range(3, 6);
            let node = Node::new(count, r_w);
            pod.1.push(node);
            count += 1;
        }
        pods.push(pod);
    }
    for _ in 0..k {
        let mut pod_servers = Vec::<Vec::<Node>>::new();
        for _ in 0..k/2 {
            let mut access_servers = Vec::<Node>::new();
            for _ in 0..k/2 {
                let r_w = rand::thread_rng().gen_range(3, 6);
                let node = Node::new(count, r_w);
                access_servers.push(node);
                count += 1;
            }
            pod_servers.push(access_servers);
        }
        server_node.push(pod_servers);
    }
    //生成节点完毕
    //开始生成edge
    for i in 0..k*k/4{
        for j in 0..k {
            let edge = Edge {
                start: kernel_node[i],
                end: pods[j].0[i*2/k],
                weight: 1,
            };
            edges.push(edge);
            let edge = Edge {
                start: pods[j].0[i*2/k],
                end: kernel_node[i],
                weight: 1,
            };
            edges.push(edge);
        }
    }
    for i in 0..k {
        for j in 0..k/2 {
            for m in 0..k/2 {
                let edge = Edge {
                    start: pods[i].0[j],
                    end: pods[i].1[m],
                    weight: 1,
                };
                edges.push(edge);
                let edge = Edge {
                    start: pods[i].1[m],
                    end: pods[i].0[j],
                    weight: 1,
                };
                edges.push(edge);
            }
        }
    }
    for i in 0..k {
        for j in 0..k/2 {
            for m in 0..k/2 {
                let edge = Edge {
                    start: pods[i].1[j],
                    end: server_node[i][j][m],
                    weight: 1,
                };
                edges.push(edge);
                let edge = Edge {
                    start: server_node[i][j][m],
                    end: pods[i].1[j],
                    weight: 1,
                };
                edges.push(edge);
            }
        }
    }
    println!("The count is: {}", count);
    edges
}

fn make_random_graph (verticle: usize) ->  Vec::<Edge<Node>> {//随机生成一个图,矩阵中1表示存在
    //let mut data = Array2::<Node>::zeros((verticle, verticle));
    let mut i = 0;
    let mut j = 0;
    let mut edges = Vec::<Edge<Node>>::new();
    let mut random_weight = Vec::<usize>::new();
    for i in 0..verticle/10 {
        let r_w = rand::thread_rng().gen_range(30, 51);
        random_weight.push(r_w);
    }
    for i in verticle/10..verticle {
        let r_w = rand::thread_rng().gen_range(1, 6);
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

fn get_hash_edges(edges: &Vec::<Edge<Node>>) -> HashMap::<(Node, Node), usize> {
    let mut hash_edges = HashMap::<(Node, Node), usize>::new();
    for edge in edges {
        hash_edges.insert((edge.start, edge.end), edge.weight);
    }
    hash_edges
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
    for (_, vertexs) in common_neighbor_reverse {
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
    //println!("the common neighbor edges is {}", new_edges.len());
    //affinity_clustering::print_edges(&new_edges);
    new_edges
}

fn RankSwap(line: Vec::<Node>, k: usize, r: usize, q: Vec::<usize>) -> Vec::<Node> {
    let mut divided_line = Vec::<Vec::<Vec::<Node>>>::new();
    let mut cut_size = Vec::<usize>::new();
    //line划分为k个partitions， partition划分为r个intervals，并计算每个partition的cut size
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
    //println!("Present max cut size is: {}", find_max_size(&cut_size));
    println!("Begin to optimize the cut size");
    //划分完毕，开始配对
    /*let mut pairs = Vec::<HashMap::<usize, usize>>::new();
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
    }*/
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
    println!("randomly pair intervals completed!");
    //配对interval完毕
    //配对partition开始
    let mut cut_size_copy = cut_size.clone();
    let mut partition_size_rank = Vec::<usize>::new();
    for i in 0..k {
        let mut max = 0;
        let mut max_position = 0;
        for j in 0..k {
            if cut_size_copy[j] > max {
                max = cut_size_copy[j];
                max_position = j;
            }
        }
        partition_size_rank.push(max_position);
        cut_size_copy[max_position] = 0;
    }
    let mut partition_pairs = HashMap::<usize, usize>::new();
    for i in 0..k/2 {
        partition_pairs.insert(partition_size_rank[i], partition_size_rank[k-i-1]);
    }
    //目前对partition进行排序，最大与最小配对
    //如对partition进行相邻配对，使用下段
    /*let mut partition_pairs = HashMap::<usize, usize>::new();
    for i in 0..k/2 {
        partition_pairs.insert(i*2, i*2+1);
    }*/
    

    let mut count = 0;
    loop {
        count += 1;
        println!("After {} rounds swap ", count);
        let mut flag = 0;//标志变量，为0表示此轮没有进行交换，终止循环
        for (partition1, partition2) in &partition_pairs {
            for (interval1, interval2) in &hash_pair {
                for j in 0..divided_line[*partition1][*interval1].len() {
                    let mut best_pair = Option::<usize>::None;
                    let mut present_small_max_size = max(cut_size[*partition1], cut_size[*partition2]);
                    for k in 0..divided_line[*partition2][*interval2].len() {                                                                               
                        let imaginaty_max_size = max(cut_size[*partition1] - divided_line[*partition1][*interval1][j].weight + divided_line[*partition2][*interval2][k].weight, 
                            cut_size[*partition2] - divided_line[*partition2][*interval2][k].weight + divided_line[*partition1][*interval1][j].weight);
                        if imaginaty_max_size < present_small_max_size {//交换后能获得更小的maxsize
                            best_pair = Some(k);
                            present_small_max_size = imaginaty_max_size;
                        }
                    }
                    if let Some(real_best_pair) = best_pair {//有可供交换的best pair, 交换两点
                        flag = 1;
                        let pre_weight = divided_line[*partition2][*interval2][real_best_pair].weight as i32;
                        let post_weight = divided_line[*partition1][*interval1][j].weight as i32;
                        let difference:usize = (pre_weight - post_weight).abs() as usize;
                        if pre_weight > post_weight {//更新两个partition的cutsize
                            cut_size[*partition1] += difference;
                            cut_size[*partition2] -= difference;
                        } else {
                            cut_size[*partition1] -= difference;
                            cut_size[*partition2] += difference;
                        }
                        let temp = divided_line[*partition1][*interval1][j];
                        divided_line[*partition1][*interval1][j] = divided_line[*partition2][*interval2][real_best_pair];
                        divided_line[*partition2][*interval2][real_best_pair] = temp;
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
    adjusted_line
}

fn q_list(k: usize) -> Vec::<usize>{
    let mut hash_qs = HashSet::<usize>::new();
    let mut present_k1 = k;
    let mut present_k2 = k;
    hash_qs.insert(k);
    while present_k1 != 1 && present_k2 != 1 {
        let mut hash_number = HashSet::<usize>::new();
        hash_number.insert(present_k1/2);
        hash_number.insert(present_k1 - present_k1/2);
        hash_number.insert(present_k2/2);
        hash_number.insert(present_k2 - present_k2/2);
        if hash_number.len() == 1 {
            present_k1 = *hash_number.iter().next().unwrap();
            present_k2 = present_k1;
            hash_qs.insert(present_k1);
        } else {
            let mut iters = hash_number.iter();
            present_k1 = *iters.next().unwrap();
            present_k2 = *iters.next().unwrap();
            hash_qs.insert(present_k1);
            hash_qs.insert(present_k2);
        }
    }
    let mut qs:Vec::<usize> = hash_qs.into_iter().collect();
    qs.sort();
    println!("The q_list is {:?}", qs);
    qs
}

fn DynamicProgram(line: Vec::<Node>, k: usize, alpha: f32, edges: HashMap::<(Node, Node), usize>) {
    let mut node_position = HashMap::<Node, usize>::new();
    for i in 0..line.len() {
        node_position.insert(line[i], i);
    }
    let vertex_num = line.len();
    let mut total_node_weight = 0;
    let mut total_edge_weight = 0;
    let mut J = Array3::<usize>::zeros((vertex_num, vertex_num, vertex_num));
    //J存储从区间[i, j]到点k的边长度总和,以便后续计算使用
    //初始化J
    for i in 0..vertex_num {
        total_node_weight += line[i].weight;
    }
    for ((start, end), edge_weight) in &edges {
        total_edge_weight += edge_weight;
        J[[node_position[start], node_position[start], node_position[end]]] = 1;
    }
    //J计算公式为J[i, j, k] = J[i, j-1, k] + J[j, j, k]
    for length in 2..vertex_num {//length为该区间包括的点个数
        for i in 0..vertex_num-length {
            let j = i + length - 1;
            for k in i+length..vertex_num {
                J[[i, j, k]] = J[[i, j-1, k]] + J[[j, j, k]];
            }
        }
    }
    println!("table J has been completed");

    let mut D = Array2::<usize>::zeros((vertex_num, vertex_num));
    //D存储在区间[i, j]内的边权总和，计算公式为D(i, j+1) = D(i, j) + J(i, j, j+1)
    for i in 0..vertex_num-1 {
        for j in i+1..vertex_num {
            D[[i, j]] = D[[i, j-1]] + J[[i, j-1, j]];
        }
    }
    println!("table D has been completed");
    let mut B = Array2::<usize>::zeros((vertex_num, vertex_num));
    //B存储在区间[i, j]内的点权总和，计算公式为B(i, j+1) = B(i, j) + w[j+1]
    for i in 0..vertex_num {
        B[[i, i]] = line[i].weight;
    }//初始化B
    for i in 0..vertex_num-1 {
        for j in i+1..vertex_num {
            B[[i, j]] = B[[i, j-1]] + line[j].weight;
        }
    }
    println!("table B has been completed");

    let mut C = Array3::<usize>::zeros((vertex_num, vertex_num, vertex_num));
    //C[i, j, k]存储从区间[i, k]到区间[k, j]的边长度总和
    //初始化C
    for i in 0..vertex_num-1 {
        C[[i, i+1, i]] = J[[i, i, i+1]];
    }
    for i in 0..vertex_num-2 {
        for j in i+2..vertex_num {
            for cut_point in i..j {
                C[[i, j, cut_point]] = C[[i, j-1, cut_point]] + J[[i, cut_point, j]];
            }
        }
    }
    println!("table C has been completed");
    let mut A = Array3::<usize>::zeros((vertex_num, vertex_num, k+1));
    let mut Ap = ArrayBase::<ndarray::OwnedRepr<Vec::<usize>>, ndarray::Dim<[usize; 3]>>::default((vertex_num, vertex_num, k+1));
    //存储A(i, j, q)，即中间解, Ap存储切割位置
    //初始化A
    println!("The average weight of cluster is: {}", total_node_weight as f32/k as f32 + total_edge_weight as f32/k as f32);
    for i in 0..vertex_num {
        for j in i.. vertex_num {
            A[[i, j, 1]] = B[[i, j]] + D[[i, j]] + C[[0, j, i]] + C[[i, vertex_num-1, j]];
            /*if sum as f32 <= (1.0+alpha) * (total_node_weight as f32/k as f32 + (j-i+1) as f32 * edge_weight_per_node) {
                A[[i, j, 1]] = sum;
            } else {
                A[[i, j, 1]] = 1000000;//infinity
            }*/
        }
    }
    println!("begin to dynamic programing");
    //动态规划序列准备
    let mut qs = q_list(k);
    qs.remove(0);
    //开始动态规划
    //for q in 2..k+1 {
    for q in qs {
        println!("now running in q = {}", q);
        let left = q/2;
        let right = q-q/2;
        for i in 0..vertex_num-1 {
            for j in i+1..vertex_num {
                if q <= j-i+1 {
                    let mut min_cut_point = i;
                    let mut min_cut_size = 1000000;
                    for cut_point in i..j {
                        if cut_point-i+1 >= left && j-cut_point+1 >= right {
                            let cut_size_temp = max(A[[i, cut_point, left]], A[[cut_point+1, j, right]]);
                            if cut_size_temp < min_cut_size {
                                min_cut_size = cut_size_temp;
                                min_cut_point = cut_point;
                            }
                        }
                    }
                    A[[i, j, q]] = min_cut_size;
                    Ap[[i, j, q]] = Ap[[i, min_cut_point, left]].clone();
                    Ap[[i, j, q]].push(min_cut_point);
                    let mut right_part = Ap[[min_cut_point+1, j, right]].clone();
                    Ap[[i, j, q]].append(&mut right_part);
                }
            }
        }
    }
    println!("dynamic programing finished!");
    //输出动态规划结果
    println!("These cut point is: {:?}", Ap[[0, vertex_num-1, k]]);
    let mut cluster_size = Vec::<usize>::new();
    let mut cut_index = Vec::<usize>::new();
    cut_index.push(0);
    let mut temp = Ap[[0, vertex_num-1, k]].clone();
    cut_index.append(&mut temp);
    cut_index.push(vertex_num-1);
    println!("The cut_index is: {:?}", cut_index);
    let mut max_cluster_size = 0;
    for i in 0..cut_index.len()-1 {
        let mut present_cluster_size = 0;
        if i == 0 {
            present_cluster_size += C[[0, vertex_num-1, cut_index[1]]];
        } else if i==cut_index.len()-2 {
            present_cluster_size += C[[0, vertex_num-1, cut_index[i]]];
        } else {
            present_cluster_size += C[[0, cut_index[i+1], cut_index[i]]];
            present_cluster_size += C[[cut_index[i], vertex_num-1, cut_index[i+1]]];
        }
        present_cluster_size += D[[cut_index[i], cut_index[i+1]]];
        present_cluster_size += B[[cut_index[i], cut_index[i+1]]];
        cluster_size.push(present_cluster_size);
        if present_cluster_size > max_cluster_size {
            max_cluster_size = present_cluster_size;
        }
    }
    let mut mean:f32 = 0.0;
    let mut variance:f32= 0.0;
    let mut standard_deviation:f32 = 0.0;
    for i in 0..cluster_size.len() {
        mean += cluster_size[i] as f32 / cluster_size.len() as f32;
    }
    for i in 0..cluster_size.len() {
        variance += (cluster_size[i] as f32 - mean).powf(2.0);
    }
    variance /= cluster_size.len() as f32;
    standard_deviation = variance.sqrt();
    let coefficient_of_variation = standard_deviation / mean;
    println!("The cut sizes is: {:?}", cluster_size);
    println!("The mean cut size is: {}", mean);
    println!("The max cut size is: {}", max_cluster_size);
    println!("The coefficient of variance is: {}", coefficient_of_variation);
}

fn Combination(af: affinity_clustering::Affinity::<Node>, partition_number: usize, interval_len: usize) -> Vec::<Node> {
    let mut q = Vec::<usize>::new();
    let mut line = af.linear_embed();
    for i in 0..partition_number+1 {
        q.push(((i*line.len()) as f32 / partition_number as f32).floor() as usize);
    }
    //let adjusted_line = RankSwap(line, partition_number, interval_len, q);
    //adjusted_line
    line
}

fn main() {
    let graph_scale = 12;
    let partition_numbers = 64;
    let edges = make_fat_tree(graph_scale);
    let mut vertexs = HashSet::<Node>::new();
    for edge in edges.iter() {
        vertexs.insert(edge.start);
        vertexs.insert(edge.end);
    }
    println!("vertexs number is: {}", vertexs.len());
    let sw = Stopwatch::start_new();
    let hash_edges = get_hash_edges(&edges);    
    let new_edges = find_common_neighbors(&edges);
    let af = affinity_clustering::make_cluster(0.4, new_edges, 10, false, true);
    let line_after_swap = Combination(af, partition_numbers, ((graph_scale/partition_numbers)as f32).sqrt() as usize);
    DynamicProgram(line_after_swap, partition_numbers, 0.2, hash_edges);
    println!("The running time is:{}", sw.elapsed_ms());
}
