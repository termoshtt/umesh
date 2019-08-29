/*!

Implementation of discrete differential geometrical (DDG) algorithms
based on [Keenan Crane][DDG].

[DDG]: https://www.cs.cmu.edu/~kmcrane/Projects/DDG

*/

use crate::{connection_matrix::*, permutation::*};

#[derive(Debug, Clone)]
pub struct Mesh<Vertex, Edge, Face> {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    /// A0 matrix in DDG
    vertex_edge: ConnectionMatrix,
    /// A1 matrix in DDG
    edge_face: ConnectionMatrix,
}

impl<Vertex, Edge, Face> Mesh<Vertex, Edge, Face> {
    /// Create mesh from two connection matrices `A0` and `A1` in DDG
    ///
    /// Vertices, edges, and faces are initialized by `Default` trait.
    pub fn from_connections(vertex_edge: ConnectionMatrix, edge_face: ConnectionMatrix) -> Self
    where
        Vertex: Default + Clone,
        Edge: Default + Clone,
        Face: Default + Clone,
    {
        let (v, e1) = vertex_edge.shape();
        let (e2, f) = edge_face.shape();
        assert_eq!(e1, e2); // Vertex-Edge matrix and Edge-Face matrix are compatible
        let vertices = vec![Default::default(); v];
        let edges = vec![Default::default(); e1];
        let faces = vec![Default::default(); f];
        Mesh {
            vertices,
            edges,
            faces,
            vertex_edge,
            edge_face,
        }
    }

    /// Create from permutation (see DDG §2.5 for detail)
    ///
    /// Vertices, edges, and faces are initialized by `Default` trait.
    pub fn from_permutation(permutation: &[usize]) -> Self
    where
        Vertex: Default + Clone,
        Edge: Default + Clone,
        Face: Default + Clone,
    {
        let vertex_edge = ConnectionMatrix::from_iter(
            gather_vertices(permutation)
                .iter()
                .enumerate()
                .map(move |(v, orbit)| orbit.indices().iter().map(move |&e| (v, e)))
                .flatten(),
        );

        let edge_face = ConnectionMatrix::from_iter(
            gather_faces(permutation)
                .iter()
                .enumerate()
                .map(move |(f, orbit)| orbit.indices().iter().map(move |&e| (e, f)))
                .flatten(),
        );
        Self::from_connections(vertex_edge, edge_face)
    }
}
