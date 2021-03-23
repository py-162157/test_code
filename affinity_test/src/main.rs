use core::cmp::max;
use rand::{Rng, thread_rng};
use std::fmt::{Display, Debug};
use std::cmp::{Eq};
use ndarray::{Array3, ArrayBase, Array2};
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::marker::Copy;
use std::cmp::{PartialEq, Ord};
extern crate stopwatch;
use stopwatch::{Stopwatch};

pub struct ArrayUnion<T> {
    group: HashMap::<T, T>,
    size: HashMap::<T, usize>,
    items: HashMap::<T, Vec::<T>>,
}

pub struct Affinity<T> {
    k: usize,
    pub E: Vec::<Edge_py<T>>,
    pub V: Vec::<T>,//目前所有节点均为已知，故不需要集合类型
    uf: ArrayUnion::<T>,
    clost_neighbors: HashMap::<T, T>,
    merged: HashMap::<T, Option::<T>>//使用没有value的hashmap作为集合类型
}

impl<T:Debug + Display + Copy + Hash + Eq> Affinity<T> {
    pub fn new_and_init(edges: &Vec<Edge_py<T>>, k: usize) -> Self {
        let mut v_set = HashMap::<T, Option<T>>::new();
        for e in edges.iter() {
            v_set.insert(e.start, None);
            v_set.insert(e.end, None);
        }
        let v: Vec::<T> = v_set.keys().into_iter().map(|&x| x).collect();
        //println!("number of vertexs is:{}", v.len());
        Affinity {
            k: k,
            E: edges.clone(),
            V: v.clone(),
            uf: ArrayUnion::new_and_init(v),
            clost_neighbors: HashMap::new(),
            merged: HashMap::new()
        }
    }

    pub fn print_all_clusters(&self) {
        for (name, clusters) in self.uf.items.iter() {
            println!("cluster{}: {:?}", name, clusters);
            println!();
        }
    }

    fn merge_with_cloest_neighbors(&mut self, v: T, mut v_stack: HashMap::<T, Option<T>>) -> HashMap::<T, Option<T>>{
        if self.merged.contains_key(&v) {
            return v_stack;//v_stack记录该函数的栈里目前都寻找了哪些v，避免死循环
        }
        if let Some(self_closet) = self.clost_neighbors.get(&v) {
            if let Some(self_cloest_cloest) = self.clost_neighbors.get(self_closet) {
                if v == *self_cloest_cloest {
                    let findv = self.uf.find(v);
                    let find_clost_neighbor = self.uf.find(*self_closet);
                    self.uf.union(findv, find_clost_neighbor);//出现所有权问题，考虑将调用的函数设为&mut self
                    //可直接写为self.uf.union(self.uf.find(v), self.uf.find(*self.clost_neighbors.get(&v).unwrap()))， 考虑到对find函数的多次调用
                    self.merged.insert(v, None);
                    self.merged.insert(*self_closet, None);
                    return v_stack;
                } else {
                    let findv = self.uf.find(v);
                    let find_clost_neighbor = self.uf.find(*self_closet);
                    if v_stack.contains_key(&v) {
                        //v_stack.remove(&v);
                        return v_stack;
                    } else {
                        v_stack.insert(v, None);
                        v_stack = self.merge_with_cloest_neighbors(*self_closet, v_stack);
                        v_stack.remove(&v);
                        self.uf.union(findv, find_clost_neighbor);
                        self.merged.insert(v, None);
                        return  v_stack;
                    }
                }
            }
        }
        v_stack
    }

    fn fragment_process(&mut self, round: u32) {
        let init_group: Vec::<T> = self.uf.get_items();
        for group_name in init_group {
            if self.uf.items.contains_key(&group_name) {
                if *self.uf.size.get(&group_name).unwrap() < 2usize.pow(round) {
                    let mut edges_of_group = Vec::<Edge_py<T>>::new();
                    for e in &self.E {
                        if e.start == group_name {
                            edges_of_group.push(e.clone());
                        }
                    }
                    edges_of_group.sort_by_key(|x| x.weight);
                    for e in &edges_of_group {
                        if self.uf.find(e.end) != group_name {
                            self.uf.union(group_name, e.end);
                        }
                    }
                }
            }
        }
    }

    fn edges_update(&mut self) {
        let mut new_edges = Vec::<Edge_py<T>>::new();
            for e in &self.E {
                if self.uf.find(e.start) != self.uf.find(e.end) {
                    new_edges.push(Edge_py {
                        start: self.uf.find(e.start),
                        end: self.uf.find(e.end),
                        weight: e.weight,
                    })
                }
            }
        self.E = new_edges;
    }

    fn clustering(&mut self, FragmentProcess: bool, CommonNeighborCluster: bool) {
        let mut number_of_clusters = self.V.len();
        let mut vertexs = self.V.clone();
        let mut count = 0;
        while number_of_clusters > self.k && count < 5 {
            count += 1;
            println!("-----------------------------------after {} rounds clustering-------------------------------", count);
            self.clost_neighbors = HashMap::new();
            self.merged = HashMap::new();
            let selfe = &self.E;
            let mut EEV = edges_of_every_vertexs(selfe);
            let mut min_edges = Vec::<Edge_py<T>>::new();
            let mut v_stack = HashMap::<T, Option<T>>::new();

            for value in EEV.values_mut() {
                value.sort_by_key(|x| x.weight);
                if CommonNeighborCluster == true {
                    value.reverse();
                }
                min_edges.push(value[0].clone());
            }

            for edge in min_edges {
                self.clost_neighbors.insert(edge.start, edge.end);
            }
            
            for v in &vertexs {
                if !self.merged.contains_key(&v) {
                    v_stack = self.merge_with_cloest_neighbors(*v, v_stack);
                }
            }

            //更新两个cluster之间的边
            self.edges_update();

            //更新点
            vertexs = self.uf.get_items();
            number_of_clusters = vertexs.len();

            if FragmentProcess == true {
                //处理碎片，可以避免极端不平衡的碎片，但会降低性能
                self.fragment_process(count);
                self.edges_update();
                //self.print_all_clusters();
            }
            //print_edges(&self.E);
            //self.print_all_clusters();
            println!("present number of clusters is: {}", number_of_clusters);
        }
    }

    pub fn linear_embed(&self) -> Vec::<T> {
        let mut line = Vec::<T>::new();
        for (_, cluster) in &self.uf.items {
            line.append(&mut cluster.clone());
        }
        line
    }
}

pub fn edges_of_every_vertexs<T:Debug + Display + Copy + Hash + Eq> (edges: &Vec::<Edge_py<T>>) -> HashMap::<T, Vec::<Edge_py<T>>> {//找到每个点的所有边
    let mut edges_of_v = HashMap::<T, Vec::<Edge_py<T>>>::new();
    for edge in edges {
        if edges_of_v.contains_key(&edge.start) {
            edges_of_v.get_mut(&edge.start).unwrap().push(edge.clone());
        } else {
            edges_of_v.insert(edge.start, vec!(edge.clone()));
        }
    }
    edges_of_v
}

impl<T:Debug + Display + Copy + Hash + Eq> ArrayUnion<T> {
    fn new_and_init(V: Vec::<T>) -> Self {
        let mut ArrUni = ArrayUnion {
            group: HashMap::new(),
            size: HashMap::new(),
            items: HashMap::new(),
        };
        for v in V {
            ArrUni.group.insert(v, v);
            ArrUni.size.insert(v, 1);
            ArrUni.items.insert(v, vec!(v));
        }
        ArrUni
    }

    fn find(&self, target: T) -> T {//返回包含target的group id
            if let Some(gp) = self.group.get(&target) {
                *gp
            } else {
                panic!("Error happened when finding the target's group!");
            }
    }

    fn union(&mut self, mut a: T, mut b: T) {
        if !(self.items.contains_key(&a) && self.items.contains_key(&b)) {
            //panic!("Error: a and b are not both in items");
            return ;
        }

        if self.size.get(&a).unwrap() > self.size.get(&b).unwrap() {
            let temp = a;
            a = b;
            b = temp;
        }

        for s in self.items.clone().get_mut(&a) {
            if let Some(x) = self.group.get_mut(&a) {
                *x = b
            } else {
                panic!("Failed to get a from items a");
            }

            if let Some(x) = self.items.get_mut(&b) {
                (*x).append(s);
            } else {
                panic!("Failed to get a from items b");
            }
        }

        *self.size.get_mut(&b).unwrap() += *self.size.get(&a).unwrap();
        self.size.remove(&a);
        self.items.remove(&a);
    }

    fn get_items(&self) -> Vec::<T> {
        self.items.keys().into_iter().map(|&x| x).collect()//将hashmap中所有key或者value输出的方法
    }
}

pub struct Edge_py<T> {
    pub start: T,
    pub end: T,
    pub weight: usize
}

/*impl<T:Debug + Display + Copy + Hash + Eq> Edge_py<T> {
    fn clone(&self) -> Self {
        Edge_py {
            start: self.start,
            end: self.end,
            weight: self.weight,
        }
    }
}*/

impl<T:Debug + Display + Copy + Hash + Eq> Clone for Edge_py<T> {
    fn clone(&self) -> Self {
        Edge_py {
            start: self.start,
            end: self.end,
            weight: self.weight,
        }
    }
}

fn MST<T:Debug + Display + Copy + Hash + Eq> (edges: &mut Vec::<Edge_py<T>>) -> Vec::<Edge_py<T>> {
    let mut mst = Vec::<Edge_py<T>>::new();
    edges.sort_by_key(|x| x.weight);
    let mut v_set = HashMap::<T, Option<usize>>::new();//使用空value的hashmap作为集合
    for e in edges.iter() {//直接使用edge调用into_iter，发生所有权转移
        v_set.insert(e.start, None);
        v_set.insert(e.end, None);
    }
    let V = v_set.keys().into_iter().map(|&x| x).collect();
    let mut UF = ArrayUnion::new_and_init(V);
    for e in edges.iter() {
        let u_group = UF.find(e.start);
        let v_group = UF.find(e.end);
        
        if u_group != v_group {
            mst.push(e.clone());
            UF.union(u_group, v_group)
        }
    }
    mst
}

fn partition1<T:Debug + Display + Copy + Hash + Eq> (v_edges: &HashMap::<T, Vec::<Edge_py<T>>>, k: usize) 
                    ->Vec::<(T, (usize, Edge_py<T>))> {
    //对相同起点的边进行随机划分，并分配partition_key
    let mut out = Vec::<(T, (usize, Edge_py<T>))>::new();
    let mut rng = thread_rng();
    let partition_key =  rng.gen_range(0, k);
    for (v, edges) in v_edges.iter() {
        for e in edges {
            out.push((e.end, (partition_key, e.clone())));
        }
    }
    out
}

fn group_by_end<T:Debug + Display + Copy + Hash + Eq> (edges: Vec::<(T, (usize, Edge_py<T>))>) -> HashMap::<T, Vec::<(usize, Edge_py<T>)>> {
    let mut out = HashMap::<T, Vec::<(usize, Edge_py<T>)>>::new();
    for edge in edges {
        if out.contains_key(&edge.0) {
            out.get_mut(&edge.0).unwrap().push((edge.1.0, edge.1.1));
        } else {
            out.insert(edge.0, vec!((edge.1.0, edge.1.1)));
        }
    }
    out
}

fn group_and_MST<T:Debug + Display + Copy + Hash + Eq> (edges: Vec::<((usize, usize), Edge_py<T>)>) -> Vec::<Edge_py<T>> {
    let mut cluster_edges = HashMap::<(usize, usize), Vec::<Edge_py<T>>>::new();
    let mut mst = Vec::<Edge_py<T>>::new();
    for e in edges {
        if cluster_edges.contains_key(&e.0) {
            cluster_edges.get_mut(&e.0).unwrap().push(e.1);
        } else {
            cluster_edges.insert(e.0, vec!(e.1));
        }
    }

    for (_, mut ed) in cluster_edges {
        //println!("before MST is: {}", e.len());
        let mut mst_output = MST(&mut ed);

        //println!("after MST is: {}", mst_output.len());
        mst.append(&mut mst_output);
    }
    mst
}

fn partition2<T:Debug + Display + Copy + Hash + Eq> (v_edges: &HashMap::<T, Vec::<(usize, Edge_py<T>)>>, k: usize) 
                    -> Vec::<((usize, usize), Edge_py<T>)> {
    //对相同终点的边进行随机划分，并分配first_partition
    let mut out = Vec::<((usize, usize), Edge_py<T>)>::new();
    for (v, edges) in v_edges.iter() {
        let mut rng = thread_rng();
        let partition_key =  rng.gen_range(0, k);
        for e in edges {
            let first_partition = e.0;
            let edge = e.1.clone();
            out.push(((first_partition, partition_key), edge));
        }
    }
    out
}

pub fn make_random_graph_matrix (verticle: usize) -> (Array2::<usize>, Vec::<Edge_py<usize>>) {//随机生成一个图，usize类型不满足ndarray::IntoDimension特征，故verticle用usize
    let mut data = Array2::<usize>::zeros((verticle, verticle));
    let mut i = 0;
    let mut j = 0;
    let mut edges = Vec::<Edge_py<usize>>::new();
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
                    data.row_mut(i)[j] = 10000;
                } else {
                    let rand_number2 = rand::thread_rng().gen_range(1, j-i+1);
                    data.row_mut(i)[j] = rand_number2;
                    edges.push(Edge_py {
                        start: i,
                        end: j,
                        weight: rand_number2 as usize
                    });
                    edges.push(Edge_py {
                        start: j,
                        end: i,
                        weight: rand_number2 as usize
                    })
                    }
                j += 1;
            }
        }
        i += 1;
    }
    //print_edges(&edges);
    (data, edges)
}

pub fn print_edges<T:Debug + Display + Copy + Hash + Eq> (edges: &Vec::<Edge_py<T>>) {
    for edge in edges {
        println!("start:{}, end:{}, weight:{}" ,edge.start, edge.end, edge.weight);
    }
}

pub fn make_cluster<T:Debug + Display + Copy + Hash + Eq>(epsilon: f32, mut edges: Vec::<Edge_py<T>>, cluster_threshold: usize, FragmentProcess: bool,
    CommonNeighborCluster: bool) -> Affinity<T> {
    let mut v_set = HashSet::<T>::new();
    for e in edges.iter() {
        v_set.insert(e.start);
        v_set.insert(e.end);
    }//对edges扫描以计算v_set可以只算一次，待优化
    let n = v_set.len() as f32;
    let mut m = edges.len() as f32;
    let mut c: f32 = m.ln().ceil() / n.ln().ceil() - 1.0;
    //println!("total edges is:{}", edges.len());
    while c > epsilon {
        let k = (n.powf((c - epsilon) / 2.0).floor()) as usize;
        c = m.ln().ceil() / n.ln().ceil() - 1.0;
        let eev = edges_of_every_vertexs(&edges);
        let half_partition = partition1(&eev, k);
        let same_start = group_by_end(half_partition);
        let full_partition = partition2(&same_start, k);
        edges = group_and_MST(full_partition);
        m = edges.len() as f32;
        println!("total edges of MST is:{}, present c is: {}", edges.len(), c);
    }
    let mut af = Affinity::new_and_init(&edges, cluster_threshold);
    af.clustering(FragmentProcess, CommonNeighborCluster);//CommonNeighborCluster为true表示对commonneighbor进行聚合
    //af.print_all_clusters();
    af
}






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

fn make_fat_tree(k: usize) -> Vec::<Edge_py<Node>> {
    let mut edges = Vec::<Edge_py<Node>>::new();
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
            let edge = Edge_py {
                start: kernel_node[i],
                end: pods[j].0[i*2/k],
                weight: 1,
            };
            edges.push(edge);
            let edge = Edge_py {
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
                let edge = Edge_py {
                    start: pods[i].0[j],
                    end: pods[i].1[m],
                    weight: 1,
                };
                edges.push(edge);
                let edge = Edge_py {
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
                let edge = Edge_py {
                    start: pods[i].1[j],
                    end: server_node[i][j][m],
                    weight: 1,
                };
                edges.push(edge);
                let edge = Edge_py {
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

fn make_random_graph (verticle: usize) ->  Vec::<Edge_py<Node>> {//随机生成一个图,矩阵中1表示存在
    //let mut data = Array2::<Node>::zeros((verticle, verticle));
    let mut i = 0;
    let mut j = 0;
    let mut edges = Vec::<Edge_py<Node>>::new();
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
                    edges.push(Edge_py {
                        start: Node::new(i, random_weight[i]),
                        end: Node::new(j, random_weight[j]),
                        weight: 1
                    });
                    edges.push(Edge_py {
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

fn get_hash_edges(edges: &Vec::<Edge_py<Node>>) -> HashMap::<(Node, Node), usize> {
    let mut hash_edges = HashMap::<(Node, Node), usize>::new();
    for edge in edges {
        hash_edges.insert((edge.start, edge.end), edge.weight);
    }
    hash_edges
}

fn find_common_neighbors<T: Eq + Hash + Copy + Debug + Display>(edges: &Vec<Edge_py<T>>) -> Vec::<Edge_py<T>>{//转换为以common neighbor为边权的图
    let mut neighbors_of_vertex = HashMap::<T, Vec::<T>>::new();
    let mut common_neighbor_reverse = HashMap::<T, Vec::<T>>::new();
    let mut edges_of_common_neighbors = HashMap::<(T, T), usize>::new();
    let mut new_edges = Vec::<Edge_py<T>>::new();
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
        new_edges.push(Edge_py {
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

fn DynamicProgram(line: Vec::<Node>, k: usize, edges: HashMap::<(Node, Node), usize>) {
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
                    let mut min_cut_size = 10000000;
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
    for i in 0..cluster_size.len() {
        mean += cluster_size[i] as f32 / cluster_size.len() as f32;
    }
    for i in 0..cluster_size.len() {
        variance += (cluster_size[i] as f32 - mean).powf(2.0);
    }
    variance /= cluster_size.len() as f32;
    let standard_deviation = variance.sqrt();
    let coefficient_of_variation = standard_deviation / mean;
    println!("The cut sizes is: {:?}", cluster_size);
    println!("The mean cut size is: {}", mean);
    println!("The max cut size is: {}", max_cluster_size);
    println!("The coefficient of variance is: {}", coefficient_of_variation);
}

fn Combination(af: Affinity::<Node>, partition_number: usize, interval_len: usize, rank_swap: bool) -> Vec::<Node> {
/*
    //partition_number: finally clusters number predetermined
    //interval_len: length of every interval of a partition
    //rank_swap: weather to implement rank_swap algorithm
*/
    let mut q = Vec::<usize>::new();
    let line = af.linear_embed();
    for i in 0..partition_number+1 {
        q.push(((i*line.len()) as f32 / partition_number as f32).floor() as usize);
    }
    if rank_swap {
        let adjusted_line = RankSwap(line, partition_number, interval_len, q);
        adjusted_line
    } else {
        line
    }
}

fn random_mock(graph_scale: usize, partition_number: usize, rank_swap: bool, rank_swap_mode: String,  cluster_threshold: usize) {
/*
    // cluster_threshold: usize, the threshold that stop affinity clustering
    // rank_swap_mode: String ("near" or "rank"). "near" mode pair two intervals nearby, can approximately minimize edges cut off,
       while "rank" mode pair the largest interval with the smallest one, may raise edges cut off.
    // rank_swap: weather to implement rank_swap algorithm
*/
    let edges = make_fat_tree(graph_scale);

    let mut vertex_set = HashSet::<Node>::new();
    for i in 0..edges.len() {
        vertex_set.insert(edges[i].start);
        vertex_set.insert(edges[i].end);
    }
    println!("The vertex number is: {}", vertex_set.len());

    let sw = Stopwatch::start_new();
    let hash_edges = get_hash_edges(&edges);    
    let new_edges = find_common_neighbors(&edges);
    let af = make_cluster(0.4, new_edges, cluster_threshold, false, true);
    let line_after_swap = Combination(af, partition_number, ((graph_scale/partition_number)as f32).sqrt() as usize, rank_swap);
    DynamicProgram(line_after_swap, partition_number, hash_edges);
    println!("The running time is:{}", sw.elapsed_ms());
}

fn main() {
    random_mock(10, 33, false, "rank".to_string(), 10);
}
