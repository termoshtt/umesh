use petgraph::dot::Dot;
use std::env;
use umesh::Permutations;

fn main() {
    let pm: Vec<usize> = env::args()
        .skip(1)
        .map(|x| x.parse().expect("Cannot read integer"))
        .collect();
    let p = Permutations::new(&pm);
    let g = p.to_graph();

    println!("{:?}", Dot::new(&g));
}
