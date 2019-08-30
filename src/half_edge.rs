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

impl<'mesh> std::ops::Sub for Simplices<'mesh> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Simplices {
            mesh: self.mesh,
            vertices: self.vertices.difference(&other.vertices).cloned().collect(),
            edges: self.edges.difference(&other.edges).cloned().collect(),
            faces: self.faces.difference(&other.faces).cloned().collect(),
        }
    }
}

impl<'mesh> Simplices<'mesh> {
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() && self.edges.is_empty() && self.faces.is_empty()
    }

    pub fn is_complex(&self) -> bool {
        let edges = self
            .mesh
            .face_edge
            .gather_connected(self.faces.iter().cloned());
        if !edges.is_subset(&self.edges) {
            return false;
        }
        let vertices = self
            .mesh
            .edge_vertex
            .gather_connected(edges.iter().cloned());
        vertices.is_subset(&self.vertices)
    }

    pub fn is_pure_complex(&self) -> Option<usize> {
        let edges = self
            .mesh
            .face_edge
            .gather_connected(self.faces.iter().cloned());
        if edges != self.edges {
            return None;
        }
        let vertices = self
            .mesh
            .edge_vertex
            .gather_connected(edges.iter().cloned());
        if vertices != self.vertices {
            return None;
        }
        if !self.faces.is_empty() {
            return Some(2);
        }
        if !self.edges.is_empty() {
            return Some(1);
        }
        return Some(0);
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
        self.star().closure() - self.closure().star()
    }

    /// Boundary operation `bd(S)`
    pub fn boundary(&self) -> Self {
        unimplemented!()
    }
}
