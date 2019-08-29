/// Sorted indices (equal to CRS format in sparce matrices without elements)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionMatrix {
    fr: Vec<usize>,
    to: Vec<usize>,
    to_max: usize,
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
        let mut to_max = 0;
        for (n, (f, t)) in indices.into_iter().enumerate() {
            while f != current_fr {
                fr.push(n);
                current_fr += 1;
            }
            to_max = std::cmp::max(to_max, t);
            to.push(t);
        }
        fr.push(to.len());
        ConnectionMatrix { fr, to, to_max }
    }

    pub fn get_connected(&self, from_index: usize) -> &[usize] {
        let first = self.fr[from_index];
        let last = self.fr[from_index + 1];
        &self.to[first..last]
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.fr.len() - 1, self.to_max + 1)
    }

    pub fn indices(&self) -> IndexIter {
        IndexIter {
            f_index: 0,
            f_count: 0,
            t_index: 0,
            fr: &self.fr,
            to: &self.to,
        }
    }

    pub fn transpose(&self) -> Self {
        Self::from_iter(self.indices().map(|(f, t)| (t, f)))
    }

    pub fn map(&self, from_indices: &[usize]) -> Vec<usize> {
        let mut mapped: Vec<usize> = from_indices
            .iter()
            .map(|&from_index| self.get_connected(from_index).iter())
            .flatten()
            .cloned()
            .collect();
        mapped.sort_unstable();
        mapped.dedup();
        mapped
    }
}

pub struct IndexIter<'mat> {
    f_index: usize,
    f_count: usize,
    t_index: usize,
    fr: &'mat [usize],
    to: &'mat [usize],
}

impl<'mat> Iterator for IndexIter<'mat> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.t_index >= self.to.len() {
            return None;
        }
        // Decompress from index
        let f = self.f_index;
        self.f_count += 1;
        if self.f_count >= self.fr[self.f_index] {
            self.f_count = 0;
            self.f_index += 1;
        }
        let t = self.to[self.t_index];
        self.t_index += 1;
        Some((f, t))
    }
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
        assert_eq!(mat.shape(), (4, 4));
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
        assert_eq!(mat.shape(), (3, 4));
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
        assert_eq!(mat.shape(), (4, 4));
    }

    #[test]
    fn indices_iter() {
        // 1 0 0 0
        // 0 1 0 0
        // 0 1 0 1
        // 1 0 0 1
        let mat = ConnectionMatrix::from_vec(vec![(0, 0), (1, 1), (2, 1), (2, 3), (3, 0), (3, 3)]);
        let mat2 = ConnectionMatrix::from_iter(mat.indices());
        assert_eq!(mat, mat2);
    }
}
