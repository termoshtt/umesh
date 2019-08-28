/*!

Implementation of discrete differential geometrical (DDG) algorithms
based on [Keenan Crane][DDG].

[DDG]: https://www.cs.cmu.edu/~kmcrane/Projects/DDG

*/

#[derive(Debug, Clone)]
struct Mesh {
    /// A0 matrix in DDG
    vertex_edge: Vec<(usize, usize)>,
    /// A1 matrix in DDG
    edge_face: Vec<(usize, usize)>,
}
