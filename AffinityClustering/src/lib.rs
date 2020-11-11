pub mod affinity_clustering {
    use rand::{Rng, thread_rng};
    use std::collections::{HashMap, HashSet};
    use ndarray::{Array2, ArrayView};
    use std::fmt::{Display, Debug};
    use std::clone::Clone;
    use std::marker::Copy;
    use std::hash::Hash;
    use std::cmp::Eq;

    pub struct ArrayUnion<T> {
        group: HashMap::<T, T>,
        size: HashMap::<T, usize>,
        items: HashMap::<T, Vec::<T>>,
    }
    
    pub struct Affinity<T> {
        k: usize,
        pub E: Vec::<Edge<T>>,
        pub V: Vec::<T>,//目前所有节点均为已知，故不需要集合类型
        uf: ArrayUnion::<T>,
        clost_neighbors: HashMap::<T, T>,
        merged: HashMap::<T, Option::<T>>//使用没有value的hashmap作为集合类型
    }
    
    impl<T:Debug + Display + Copy + Hash + Eq> Affinity<T> {
        pub fn new_and_init(edges: &Vec<Edge<T>>, k: usize) -> Self {
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
                        let mut edges_of_group = Vec::<Edge<T>>::new();
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
            let mut new_edges = Vec::<Edge<T>>::new();
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
    
        fn clustering(&mut self, FragmentProcess: bool, CommonNeighborCluster: bool) {
            let mut number_of_clusters = self.V.len();
            let mut vertexs = self.V.clone();
            let mut count = 0;
            while number_of_clusters > self.k {
                count += 1;
                println!("-----------------------------------after {} rounds clustering-------------------------------", count);
                self.clost_neighbors = HashMap::new();
                self.merged = HashMap::new();
                let selfe = &self.E;
                let mut EEV = edges_of_every_vertexs(selfe);
                let mut min_edges = Vec::<Edge<T>>::new();
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
            }
        }

        pub fn linear_embed(&self) -> Vec::<T> {
            let mut line = Vec::<T>::new();
            for (name, cluster) in &self.uf.items {
                line.append(&mut cluster.clone());
            }
            line
        }
    }

    pub fn edges_of_every_vertexs<T:Debug + Display + Copy + Hash + Eq> (edges: &Vec::<Edge<T>>) -> HashMap::<T, Vec::<Edge<T>>> {//找到每个点的所有边
        let mut edges_of_v = HashMap::<T, Vec::<Edge<T>>>::new();
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
    
    pub struct Edge<T> {
        pub start: T,
        pub end: T,
        pub weight: usize
    }
    
    /*impl<T:Debug + Display + Copy + Hash + Eq> Edge<T> {
        fn clone(&self) -> Self {
            Edge {
                start: self.start,
                end: self.end,
                weight: self.weight,
            }
        }
    }*/

    impl<T:Debug + Display + Copy + Hash + Eq> Clone for Edge<T> {
        fn clone(&self) -> Self {
            Edge {
                start: self.start,
                end: self.end,
                weight: self.weight,
            }
        }
    }
    
    fn MST<T:Debug + Display + Copy + Hash + Eq> (edges: &mut Vec::<Edge<T>>) -> Vec::<Edge<T>> {
        let mut mst = Vec::<Edge<T>>::new();
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
    
    fn partition1<T:Debug + Display + Copy + Hash + Eq> (v_edges: &HashMap::<T, Vec::<Edge<T>>>, k: usize) 
                        ->Vec::<(T, (usize, Edge<T>))> {
        //对相同起点的边进行随机划分，并分配partition_key
        let mut out = Vec::<(T, (usize, Edge<T>))>::new();
        let mut rng = thread_rng();
        let partition_key =  rng.gen_range(0, k);
        for (v, edges) in v_edges.iter() {
            for e in edges {
                out.push((e.end, (partition_key, e.clone())));
            }
        }
        out
    }
    
    fn group_by_end<T:Debug + Display + Copy + Hash + Eq> (edges: Vec::<(T, (usize, Edge<T>))>) -> HashMap::<T, Vec::<(usize, Edge<T>)>> {
        let mut out = HashMap::<T, Vec::<(usize, Edge<T>)>>::new();
        for edge in edges {
            if out.contains_key(&edge.0) {
                out.get_mut(&edge.0).unwrap().push((edge.1.0, edge.1.1));
            } else {
                out.insert(edge.0, vec!((edge.1.0, edge.1.1)));
            }
        }
        out
    }
    
    fn group_and_MST<T:Debug + Display + Copy + Hash + Eq> (edges: Vec::<((usize, usize), Edge<T>)>) -> Vec::<Edge<T>> {
        let mut cluster_edges = HashMap::<(usize, usize), Vec::<Edge<T>>>::new();
        let mut mst = Vec::<Edge<T>>::new();
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
    
    fn partition2<T:Debug + Display + Copy + Hash + Eq> (v_edges: &HashMap::<T, Vec::<(usize, Edge<T>)>>, k: usize) 
                        -> Vec::<((usize, usize), Edge<T>)> {
        //对相同终点的边进行随机划分，并分配first_partition
        let mut out = Vec::<((usize, usize), Edge<T>)>::new();
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
    
    pub fn make_random_graph_matrix (verticle: usize) -> (Array2::<usize>, Vec::<Edge<usize>>) {//随机生成一个图，usize类型不满足ndarray::IntoDimension特征，故verticle用usize
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
    
    pub fn print_edges<T:Debug + Display + Copy + Hash + Eq> (edges: &Vec::<Edge<T>>) {
        for edge in edges {
            println!("start:{}, end:{}, weight:{}" ,edge.start, edge.end, edge.weight);
        }
    }

    pub fn make_cluster<T:Debug + Display + Copy + Hash + Eq>(epsilon: f32, mut edges: Vec::<Edge<T>>, cluster_threshold: usize, FragmentProcess: bool,
        CommonNeighborCluster: bool) -> Affinity<T> {
        let mut v_set = HashSet::<T>::new();
        for e in edges.iter() {
            v_set.insert(e.start);
            v_set.insert(e.end);
        }//对edges扫描以计算v_set可以只算一次，待优化
        let n = v_set.len() as f32;
        let mut m = edges.len() as f32;
        let mut c: f32 = m.ln().ceil() / n.ln().ceil() - 1.0;
        println!("total edges is:{}", edges.len());
        /*while c > epsilon {
            let k = (n.powf((c - epsilon) / 2.0).floor()) as usize;
            c = m.ln().ceil() / n.ln().ceil() - 1.0;
            let eev = edges_of_every_vertexs(&edges);
            let half_partition = partition1(&eev, k);
            let same_start = group_by_end(half_partition);
            let full_partition = partition2(&same_start, k);
            edges = group_and_MST(full_partition);
            m = edges.len() as f32;
            println!("total edges of MST is:{}, present c is: {}", edges.len(), c);
        }*/
        let mut af = Affinity::new_and_init(&edges, cluster_threshold);
        af.clustering(FragmentProcess, CommonNeighborCluster);//CommonNeighborCluster为true表示对commonneighbor进行聚合
        //af.print_all_clusters();
        af
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
