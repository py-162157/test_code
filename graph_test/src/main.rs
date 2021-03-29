use rand::Rng;
use ndarray::{Array2, ArrayView};
use std::fmt::Display;

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

impl Graph {
    fn new(verticle: usize) -> Self {
        Graph {
            vex: Vec::new(),
            vexnum: verticle,
            edgenum: 0,
            matrix: Array2::<usize>::zeros((verticle, verticle))
        }
    }

    fn load(&mut self, matrix: Array2::<usize>, edgesnum: usize) {
        //println!("{}", self.vexnum);
        //println!("{}", matrix.ncols());
        if self.vexnum != matrix.ncols() {
            panic!("The shape of G and matrix don't match!")
        }
        let mut V = Vec::<usize>::new();
        for i in 0..matrix.ncols() {
            V.push(i);
        }
        //这里直接把点的类型设为了u32
        self.vex = V;
        self.vexnum = matrix.ncols();
        self.edgenum = edgesnum;
        self.matrix = matrix;
    }
}

fn make_graph_matrix(verticle: usize, max_edge_weight: usize) -> (Array2::<usize>, Vec::<Edge>) {//随机生成一个图，usize类型不满足ndarray::IntoDimension特征，故verticle用usize
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
                let rand_number1 = rand::thread_rng().gen_range(1, 30);
                if rand_number1 != 1 {
                    data.row_mut(i)[j] = 1000;
                } else {
                    let rand_number2 = rand::thread_rng().gen_range(1, max_edge_weight);
                    data.row_mut(i)[j] = rand_number2;
                    edges.push(Edge {
                        start: i,
                        end: j,
                        weight: rand_number2 as usize
                    })
                    }
                j += 1;
            }
        }
        i += 1;
    }
    (data, edges)
}

fn get_end(vends: &Vec<Option<usize>>, mut x: usize) -> usize {
    while vends[x].is_some() {
        x = vends[x].unwrap();
    }
    x
}

fn MST(G: Graph, mut edges: Vec::<Edge>) -> (Vec::<Edge>, usize) {
    let mut p1:usize; let mut p2:usize; let mut m:usize; let mut n:usize;
    let mut index: usize = 0;//rets数组的索引
    let mut vends: Vec<Option<usize>> = vec!(None; edges.len());//保存已有MST中每个顶点在该树中的终点
    let mut rets = Vec::<Edge>::new();//保存最小生成树的边
    let mut mst_weight = 0;

    edges.sort_by_key(|k| k.weight);
    /*for i in 0..edges.len() {
        println!("{}", edges[i].weight);
    }*/

    for i in 0..G.edgenum {
        p1 = edges[i].start;
        p2 = edges[i].end;

        m = get_end(&vends, p1);
        n = get_end(&vends, p2);

        if m != n {
            vends[m] = Some(n);
            rets.push(Edge {
                start: edges[i].start,
                end: edges[i].end,
                weight: edges[i].weight,
            });
            index += 1;
            mst_weight += edges[i].weight;
        }
    }

    (rets, mst_weight)
}

fn main() {
    /*let data = ndarray::arr2(&[[0   , 4   , 1000, 1000, 1000, 1000, 1000, 8   , 1000],
                               [4   , 0   , 8   , 1000, 1000, 1000, 1000, 11  , 1000],
                               [1000, 8   , 0   , 7   , 1000, 4   , 1000, 1000, 2   ],
                               [1000, 1000, 7   , 0   , 9   , 14  , 1000, 1000, 1000],
                               [1000, 1000, 1000, 9   , 0   , 10  , 1000, 1000, 1000],
                               [1000, 1000, 4   , 14  , 10  , 0   , 2   , 1000, 1000],
                               [1000, 1000, 1000, 1000, 1000, 2   , 0   , 1   , 6   ],
                               [8   , 11  , 1000, 1000, 1000, 1000, 1   , 0   , 7   ],
                               [1000, 1000, 2   , 1000, 1000, 1000, 6   , 7   , 0   ]]);

    let mut i = 0;
    let mut j;
    let mut edges = Vec::<Edge>::new();
    let mst_tuple: (Vec::<Edge>, usize);
    while i < data.ncols() {
        j = 0;
        while j < data.ncols() {
            if i > j && data.row(i)[j] != 1000 {
                edges.push(Edge {
                    start: i,
                    end: j,
                    weight: data.row(i)[j],
                    });
            }
                j += 1;
        }
        i += 1;
    }
    /*for i in 0..edges.len() {
        println!("{}", edges[i].weight);
    }*/
*/
    let data_edge:(Array2::<usize>, Vec::<Edge>) = make_graph_matrix(100, 20);
    let mut G = Graph::new(100);
    let mst_tuple: (Vec::<Edge>, usize);
    G.load(data_edge.0, data_edge.1.len());
    mst_tuple = MST(G, data_edge.1);
    for i in 0..mst_tuple.0.len() {
        println!("MST edge {}: {} -> {}, wtighet is: {}", i, mst_tuple.0.get(i).unwrap().start, mst_tuple.0.get(i).unwrap().end, mst_tuple.0.get(i).unwrap().weight);
    }
    println!("The MST total weight is: {}", mst_tuple.1);
}
