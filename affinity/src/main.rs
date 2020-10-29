use rand::{Rng, thread_rng};
use std::collections::HashMap;
use ndarray::{Array2, ArrayView};
use std::fmt::Display;
use std::clone::Clone;

//需要手动实现groupby函数

struct ArrayUnion {
    group: HashMap::<usize, usize>,
    size: HashMap::<usize, usize>,
    items: HashMap::<usize, Vec::<usize>>,
}

impl Drop for ArrayUnion {
    fn drop(&mut self) {
        let group_key: Vec::<usize> = self.group.keys().into_iter().map(|&x| x).collect();
        let size_key: Vec::<usize> = self.size.keys().into_iter().map(|&x| x).collect();
        let items_key: Vec::<usize> = self.items.keys().into_iter().map(|&x| x).collect();
        for g in group_key {
            self.group.remove(&g);
        }
        for s in size_key {
            self.size.remove(&s);
        }
        for i in items_key {
            self.size.remove(&i);
        }
    }
}

struct Affinity {
    k: usize,
    E: Vec::<Edge>,
    V: Vec::<usize>,//目前所有节点均为已知，故不需要集合类型
    uf: ArrayUnion,
    clost_neighbors: HashMap::<usize, usize>,
    merged: HashMap::<usize, Option::<usize>>//使用没有value的hashmap作为集合类型
}

impl Drop for Affinity {
    fn drop(&mut self) {
        while !self.E.is_empty() {
            self.E.remove(0);
        }
        while !self.V.is_empty() {
            self.V.remove(0);
        }
        let clost_key: Vec::<usize> = self.clost_neighbors.keys().into_iter().map(|&x| x).collect();
        let merged_key: Vec::<usize> = self.merged.keys().into_iter().map(|&x| x).collect();
        for c in clost_key {
            self.clost_neighbors.remove(&c);
        }
        for m in merged_key {
            self.merged.remove(&m);
        }
    }
}

impl Affinity {
    fn new_and_init(edges: Vec<Edge>, k: usize) -> Affinity {
        let mut v_set = HashMap::<usize, Option<usize>>::new();
        for e in edges.iter() {
            v_set.insert(e.start, None);
            v_set.insert(e.end, None);
        }
        let v: Vec::<usize> = v_set.keys().into_iter().map(|&x| x).collect();
        //println!("number of vertexs is:{}", v.len());
        Affinity {
            k: k,
            E: edges,
            V: v.clone(),
            uf: ArrayUnion::new_and_init(v),
            clost_neighbors: HashMap::new(),
            merged: HashMap::new()
        }
    }

    fn print_all_clusters(&self) {
        for (name, clusters) in self.uf.items.iter() {
            println!("cluster{}: {:?}", name, clusters);
        }
    }

    fn merge_with_cloest_neighbors(&mut self, v: usize, mut v_stack: HashMap::<usize, Option<usize>>) -> HashMap::<usize, Option<usize>>{
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
        let init_group: Vec::<usize> = self.uf.get_items();
        for group_name in init_group {
            if self.uf.items.contains_key(&group_name) {
                if *self.uf.size.get(&group_name).unwrap() < 2usize.pow(round) {
                    let mut edges_of_group = Vec::<Edge>::new();
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
        let mut new_edges = Vec::<Edge>::new();
            for e in &self.E {
                if self.uf.find(e.start) != self.uf.find(e.end) {
                    new_edges.push(Edge {
                        start: self.uf.find(e.start),
                        end: self.uf.find(e.end),
                        weight: e.weight,
                    })
                }
            }
        self.E = new_edges;
    }

    fn clustering(&mut self) {
        let mut number_of_clusters = self.V.len();
        let mut vertexs = self.V.clone();
        let mut count = 0;
        while number_of_clusters > self.k {
            count += 1;
            println!("-----------------------------------after {} rounds-------------------------------", count);
            self.clost_neighbors = HashMap::new();
            self.merged = HashMap::new();
            let selfe = &self.E;
            let mut EEV = edges_of_every_vertexs(selfe);
            let mut min_edges = Vec::<Edge>::new();
            let mut v_stack = HashMap::<usize, Option<usize>>::new();

            for value in EEV.values_mut() {
                value.sort_by_key(|x| x.weight);
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

            //处理碎片，可以避免极端不平衡的碎片，但会降低性能
            self.fragment_process(count);
            self.edges_update();
            self.print_all_clusters();
            //print_edges(&self.E);
        }
    }
}

fn edges_of_every_vertexs(edges: &Vec::<Edge>) -> HashMap::<usize, Vec::<Edge>> {//找到每个点的所有边
    let mut edges_of_v = HashMap::<usize, Vec::<Edge>>::new();
    for edge in edges {
        if edges_of_v.contains_key(&edge.start) {
            edges_of_v.get_mut(&edge.start).unwrap().push(edge.clone());
        } else {
            edges_of_v.insert(edge.start, vec!(edge.clone()));
        }
    }
    edges_of_v
}

impl ArrayUnion {
    fn new_and_init(V: Vec::<usize>) -> Self {
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

    fn find(&self, target: usize) -> usize {//返回包含target的group id
         if let Some(gp) = self.group.get(&target) {
             *gp
         } else {
             panic!("Error happened when finding the target's group!");
         }
    }

    fn union(&mut self, mut a: usize, mut b: usize) {
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

    fn get_items(&self) -> Vec::<usize> {
        self.items.keys().into_iter().map(|&x| x).collect()//将hashmap中所有key或者value输出的方法
    }

    fn get_partitions(self) ->Vec::<Vec::<usize>> {//该函数仅调用一次
        let mut partitions = Vec::<Vec::<usize>>::new();
        for partition in self.items.values() {
            partitions.push((*partition).clone());
        }
        partitions
    }
}

struct Graph {
    vex: Vec<usize>,
    vexnum: usize,
    edgenum: usize,
    matrix: Array2::<usize>
}

struct Edge {
    start: usize,
    end: usize,
    weight: usize
}

impl Edge {
    fn clone(&self) -> Self {
        Edge {
            start: self.start,
            end: self.end,
            weight: self.weight,
        }
    }
}

fn MST(edges: &mut Vec::<Edge>) -> Vec::<Edge> {
    let mut mst = Vec::<Edge>::new();
    edges.sort_by_key(|x| x.weight);
    let mut v_set = HashMap::<usize, Option<usize>>::new();//使用空value的hashmap作为集合
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

fn partition1(v_edges: &HashMap::<usize, Vec::<Edge>>, k: usize) 
                    ->Vec::<(usize, (usize, Edge))> {
    //对相同起点的边进行随机划分，并分配partition_key
    let mut out = Vec::<(usize, (usize, Edge))>::new();
    let mut rng = thread_rng();
    let partition_key =  rng.gen_range(0, k);
    for (v, edges) in v_edges.iter() {
        for e in edges {
            out.push((e.end, (partition_key, e.clone())));
        }
    }
    out
}

fn group_by_end(edges: Vec::<(usize, (usize, Edge))>) -> HashMap::<usize, Vec::<(usize, Edge)>> {
    let mut out = HashMap::<usize, Vec::<(usize, Edge)>>::new();
    for edge in edges {
        if out.contains_key(&edge.0) {
            out.get_mut(&edge.0).unwrap().push((edge.1.0, edge.1.1));
        } else {
            out.insert(edge.0, vec!((edge.1.0, edge.1.1)));
        }
    }
    out
}

fn group_and_MST(edges: Vec::<((usize, usize), Edge)>) -> Vec::<Edge> {
    let mut cluster_edges = HashMap::<(usize, usize), Vec::<Edge>>::new();
    let mut mst = Vec::<Edge>::new();
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

fn partition2(v_edges: &HashMap::<usize, Vec::<(usize, Edge)>>, k: usize) 
                    -> Vec::<((usize, usize), Edge)> {
    //对相同终点的边进行随机划分，并分配first_partition
    let mut out = Vec::<((usize, usize), Edge)>::new();
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

fn make_random_graph_matrix(verticle: usize) -> (Array2::<usize>, Vec::<Edge>) {//随机生成一个图，usize类型不满足ndarray::IntoDimension特征，故verticle用usize
    let mut data = Array2::<usize>::zeros((verticle, verticle));
    let mut i = 0;
    let mut j = 0;
    let mut edges = Vec::<Edge>::new();
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
                    edges.push(Edge {
                        start: i,
                        end: j,
                        weight: rand_number2 as usize
                    });
                    edges.push(Edge {
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

fn print_edges(edges: &Vec::<Edge>) {
    for edge in edges {
        println!("start:{}, end:{}, weight:{}" ,edge.start, edge.end, edge.weight);
    }
}

fn main() {
    let epsilon = 0.4;//设定参数一，小于一定阈值会导致closet_neighbor出现死循环
    let (random_matrix, mut edges) = make_random_graph_matrix(10000);//设定参数二,随机生成图的顶点数
    let n = random_matrix.ncols() as f32;
    let mut m = edges.len() as f32;
    let mut c: f32 = m.ln().ceil() / n.ln().ceil() - 1.0;
    println!("before MST total edges is:{}", edges.len());
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
    //print_edges(&edges);
    //println!("total edges of MST is:{}", edges.len());
    let mut af = Affinity::new_and_init(edges, 30);//设定参数三,最终cluster的最大个数
    af.clustering();
    //af.print_all_clusters();
}
