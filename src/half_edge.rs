/*!

Implementation of discrete differential geometrical (DDG) algorithms
based on [Keenan Crane][DDG].

[DDG]: https://www.cs.cmu.edu/~kmcrane/Projects/DDG

*/

use crate::{connection_matrix::*, permutation::*};
use std::collections::BTreeSet;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Mesh {
    /// A0 matrix in DDG
    vertex_edge: Connection,
    /// Transpose of A0
    edge_vertex: Connection,
    /// A1 matrix in DDG
    edge_face: Connection,
    /// Transpose of A1
    face_edge: Connection,
}

impl Mesh {
    /// Create mesh from two connection matrices `A0` and `A1` in DDG
    ///
    /// Vertices, edges, and faces are initialized by `Default` trait.
    pub fn from_connections(vertex_edge: Connection, edge_face: Connection) -> Self {
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
        let vertex_edge = Connection::from_iter(
            gather_vertices(permutation)
                .iter()
                .enumerate()
                .map(move |(v, orbit)| orbit.indices().iter().map(move |&e| (v, e)))
                .flatten(),
        );

        let edge_face = Connection::from_iter(
            gather_faces(permutation)
                .iter()
                .enumerate()
                .map(move |(f, orbit)| orbit.indices().iter().map(move |&e| (e, f)))
                .flatten(),
        );
        Self::from_connections(vertex_edge, edge_face)
    }

    /// Get simplicies
    pub fn simplicies(&self, vertices: &[usize], edges: &[usize], faces: &[usize]) -> Simplices {
        Simplices {
            mesh: self,
            vertices: BTreeSet::from_iter(vertices.iter().cloned()),
            edges: BTreeSet::from_iter(edges.iter().cloned()),
            faces: BTreeSet::from_iter(faces.iter().cloned()),
        }
    }
}

/// Simplices in the mesh
///
/// - Simplex on the half-edge mesh must be one of vertex, edge, and face.
#[derive(Debug, Clone)]
pub struct Simplices<'mesh> {
    mesh: &'mesh Mesh,
    vertices: BTreeSet<usize>,
    edges: BTreeSet<usize>,
    faces: BTreeSet<usize>,
}

impl<'mesh> Simplices<'mesh> {
    pub fn is_complex(&self) -> bool {
        unimplemented!()
    }

    pub fn is_pure_complex(&self) -> bool {
        unimplemented!()
    }

    /// Star operation `St(S)` (not Hodge star)
    pub fn star(&self) -> Self {
        let mut edges = self
            .mesh
            .vertex_edge
            .gather_connected(self.vertices.iter().cloned());
        for &edge in &self.edges {
            edges.insert(edge);
        }
        let mut faces = self.mesh.edge_face.gather_connected(edges.iter().cloned());
        for &face in &self.faces {
            faces.insert(face);
        }
        Self {
            mesh: self.mesh,
            vertices: self.vertices.clone(),
            edges,
            faces,
        }
    }

    /// Closure operation `Cl(S)`
    pub fn closure(&self) -> Self {
        let mut edges = self
            .mesh
            .face_edge
            .gather_connected(self.faces.iter().cloned());
        for &edge in &self.edges {
            edges.insert(edge);
        }
        let mut vertices = self
            .mesh
            .edge_vertex
            .gather_connected(edges.iter().cloned());
        for &vertex in &self.vertices {
            vertices.insert(vertex);
        }
        Self {
            mesh: self.mesh,
            vertices,
            edges,
            faces: self.faces.clone(),
        }
    }

    /// Link operation `Lk(S)`
    pub fn link(&self) -> Self {
        unimplemented!()
    }

    /// Boundary operation `bd(S)`
    pub fn boundary(&self) -> Self {
        unimplemented!()
    }
}
