/*!

Implementation of discrete differential geometrical (DDG) algorithms
based on [Keenan Crane][DDG].

[DDG]: https://www.cs.cmu.edu/~kmcrane/Projects/DDG

*/

/// Sorted indices (equal to CRS format in sparce matrices)
#[derive(Debug, Clone)]
pub struct ConnectionMatrix {
    fr_indices: Vec<usize>,
    to_indices: Vec<usize>,
}

impl ConnectionMatrix {
    /// Create connection matrix from a series of pair of indices.
    /// The indices will be sorted.
    pub fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: Iterator<Item = (usize, usize)>,
    {
        let indices: Vec<_> = iter.collect();
        Self::from_vec(indices)
    }

    /// Create connection matrix from a series of pair of indices.
    /// The indices will be sorted.
    pub fn from_vec(mut indices: Vec<(usize, usize)>) -> Self {
        indices.sort_unstable();
        unsafe { Self::from_sorted_vec(indices) }
    }

    /// Create connection matrix from a series of pair of indices without sorting.
    ///
    /// Safety
    /// ------
    /// - unsafe if the input indices are not sorted
    pub unsafe fn from_sorted_vec(indices: Vec<(usize, usize)>) -> Self {
        let mut fr_indices = Vec::with_capacity(indices.len());
        let mut to_indices = Vec::with_capacity(indices.len());
        for &(fr, to) in &indices {
            fr_indices.push(fr);
            to_indices.push(to);
        }
        ConnectionMatrix {
            fr_indices,
            to_indices,
        }
    }
}

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
