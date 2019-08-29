/*!

Implementation of discrete differential geometrical (DDG) algorithms
based on [Keenan Crane][DDG].

[DDG]: https://www.cs.cmu.edu/~kmcrane/Projects/DDG

*/

use crate::{connection_matrix::*, permutation::*};

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
    pub fn simplicies(
        &self,
        mut vertices: Vec<usize>,
        mut edges: Vec<usize>,
        mut faces: Vec<usize>,
    ) -> Simplices {
        vertices.sort_unstable();
        vertices.dedup();
        edges.sort_unstable();
        edges.dedup();
        faces.sort_unstable();
        faces.dedup();
        unsafe { self.simplicies_sorted(vertices, edges, faces) }
    }

    /// Get simplicies with sorted unique indices
    ///
    /// Safety
    /// -------
    /// - Input vectors must be sorted and uniqued
    pub unsafe fn simplicies_sorted(
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

fn append_unique(mut a: Vec<usize>, b: &[usize]) -> Vec<usize> {
    for &val in b {
        a.push(val)
    }
    a.sort_unstable();
    a.dedup();
    a
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
        let edges = append_unique(
            self.mesh.vertex_edge.gather_connected(&self.vertices),
            &self.edges,
        );
        let faces = append_unique(self.mesh.edge_face.gather_connected(&edges), &self.faces);
        Simplices {
            mesh: self.mesh,
            vertices: self.vertices.clone(),
            edges,
            faces,
        }
    }

    /// Closure operation `Cl(S)`
    pub fn closure(&self) -> Self {
        let edges = append_unique(
            self.mesh.face_edge.gather_connected(&self.faces),
            &self.edges,
        );
        let vertices = append_unique(
            self.mesh.edge_vertex.gather_connected(&edges),
            &self.vertices,
        );
        Simplices {
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
