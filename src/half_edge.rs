/*!

Implementation of discrete differential geometrical (DDG) algorithms
based on [Keenan Crane][DDG].

[DDG]: https://www.cs.cmu.edu/~kmcrane/Projects/DDG

*/

/// Sorted indices (equal to CRS format in sparce matrices without elements)
#[derive(Debug, Clone)]
pub struct ConnectionMatrix {
    fr: Vec<usize>,
    to: Vec<usize>,
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
        let mut to = Vec::with_capacity(indices.len());
        let mut fr = vec![0];
        let mut current_fr = 0;
        for (n, (f, t)) in indices.into_iter().enumerate() {
            while f != current_fr {
                fr.push(n);
                current_fr += 1;
            }
            to.push(t);
        }
        fr.push(to.len());
        ConnectionMatrix { fr, to }
    }

    pub fn get_connected(&self, from_index: usize) -> &[usize] {
        let first = self.fr[from_index];
        let last = self.fr[from_index + 1];
        &self.to[first..last]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connnection_matrix_square() {
        // 1 0 1 0
        // 1 1 0 0
        // 0 1 0 1
        // 1 0 0 1
        let mat = ConnectionMatrix::from_vec(vec![
            (0, 0),
            (0, 2),
            (1, 0),
            (1, 1),
            (2, 1),
            (2, 3),
            (3, 0),
            (3, 3),
        ]);
        dbg!(&mat);
        assert_eq!(mat.fr, vec![0, 2, 4, 6, 8]);
        assert_eq!(mat.to, vec![0, 2, 0, 1, 1, 3, 0, 3]);
    }

    #[test]
    fn connnection_matrix_nonsquare() {
        // 1 0 1 0
        // 0 1 0 1
        // 1 0 0 1
        let mat = ConnectionMatrix::from_vec(vec![(0, 0), (0, 2), (1, 1), (1, 3), (2, 0), (2, 3)]);
        dbg!(&mat);
        assert_eq!(mat.fr, vec![0, 2, 4, 6]);
        assert_eq!(mat.to, vec![0, 2, 1, 3, 0, 3]);
    }

    #[test]
    fn connnection_matrix_empty_row() {
        // 1 0 1 0
        // 0 0 0 0
        // 0 1 0 1
        // 1 0 0 1
        let mat = ConnectionMatrix::from_vec(vec![(0, 0), (0, 2), (2, 1), (2, 3), (3, 0), (3, 3)]);
        dbg!(&mat);
        assert_eq!(mat.fr, vec![0, 2, 2, 4, 6]);
        assert_eq!(mat.to, vec![0, 2, 1, 3, 0, 3]);
    }
}
