/*!

Implementation of discrete differential geometrical (DDG) algorithms
based on [Keenan Crane][DDG].

[DDG]: https://www.cs.cmu.edu/~kmcrane/Projects/DDG

*/

use crate::{connection_matrix::*, permutation::*};

#[derive(Debug, Clone)]
pub struct Mesh {
    /// A0 matrix in DDG
    vertex_edge: ConnectionMatrix,
    /// Transpose of A0
    edge_vertex: ConnectionMatrix,
    /// A1 matrix in DDG
    edge_face: ConnectionMatrix,
    /// Transpose of A1
    face_edge: ConnectionMatrix,
}

impl Mesh {
    /// Create mesh from two connection matrices `A0` and `A1` in DDG
    ///
    /// Vertices, edges, and faces are initialized by `Default` trait.
    pub fn from_connections(vertex_edge: ConnectionMatrix, edge_face: ConnectionMatrix) -> Self {
        let (_, e1) = vertex_edge.shape();
        let (e2, _) = edge_face.shape();
        assert_eq!(e1, e2); // Vertex-Edge matrix and Edge-Face matrix are compatible

        let edge_vertex = vertex_edge.transpose();
        let face_edge = edge_face.transpose();
        Mesh {
            vertex_edge,
            edge_vertex,
            edge_face,
            face_edge,
        }
    }

    /// Create from permutation (see DDG §2.5 for detail)
    ///
    /// Vertices, edges, and faces are initialized by `Default` trait.
    pub fn from_permutation(permutation: &[usize]) -> Self {
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

    /// Get simplicies
    pub fn simplicies(
        &self,
        vertices: Vec<usize>,
        edges: Vec<usize>,
        faces: Vec<usize>,
    ) -> Simplices {
        Simplices {
            mesh: self,
            vertices,
            edges,
            faces,
        }
    }
}

/// Simplices in the mesh
///
/// - Simplex on the half-edge mesh must be one of vertex, edge, and face.
#[derive(Debug, Clone)]
pub struct Simplices<'mesh> {
    mesh: &'mesh Mesh,
    vertices: Vec<usize>,
    edges: Vec<usize>,
    faces: Vec<usize>,
}

impl<'mesh> Simplices<'mesh> {
    pub fn is_complex(&self) -> bool {
        unimplemented!()
    }

    pub fn is_pure_complex(&self) -> bool {
        unimplemented!()
    }
}
