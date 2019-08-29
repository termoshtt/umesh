/*!
Utility for permutation (see DDG §2.5 for detail)
*/

use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Orbit {
    /// rotated to the minimum comes first
    edges: Vec<usize>,
}

impl Ord for Orbit {
    fn cmp(&self, other: &Orbit) -> Ordering {
        for i in 0.. {
            match self.edges[i].cmp(&other.edges[i]) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => {
                    if i == self.edges.len() - 1 {
                        return Ordering::Equal;
                    }
                }
            }
        }
        unreachable!()
    }
}

impl Orbit {
    fn new(edges: &[usize]) -> Self {
        assert!(edges.len() > 0);
        // take argmin
        let mut argmin = 0;
        let mut min = edges[0];
        for (i, e) in edges.iter().enumerate() {
            if min > *e {
                argmin = i;
                min = *e;
            }
        }
        // rotate
        let (a, b) = edges.split_at(argmin);
        let rotated: Vec<_> = b.iter().chain(a.iter()).cloned().collect();
        Orbit { edges: rotated }
    }

    pub fn indices(&self) -> &[usize] {
        &self.edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbit_new() {
        let v = Orbit::new(&[2, 1, 3]);
        assert_eq!(
            v,
            Orbit {
                edges: vec![1, 3, 2]
            }
        );
    }
}

fn twin(index: usize) -> usize {
    if index % 2 == 0 {
        index + 1
    } else {
        index - 1
    }
}

pub fn gather_vertices(permutation: &[usize]) -> Vec<Orbit> {
    // XXX More check?
    assert_eq!(permutation.len() % 2, 0);

    // FIXME we can implement by more efficient algorithm
    let mut vs: Vec<_> = (0..permutation.len())
        .map(|init| {
            // iterator over twin-next orbit
            let mut orbit = vec![init];
            let mut current = init;
            loop {
                let t = twin(current);
                current = permutation[t];
                if current == init {
                    break;
                }
                orbit.push(current);
            }
            Orbit::new(&orbit)
        })
        .collect();
    vs.sort_unstable();
    vs.dedup();
    vs
}

pub fn gather_faces(permutation: &[usize]) -> Vec<Orbit> {
    // XXX More check?
    assert_eq!(permutation.len() % 2, 0);

    // FIXME we can implement by more efficient algorithm
    let mut vs: Vec<_> = (0..permutation.len())
        .map(|init| {
            // iterator over next orbit
            let mut orbit = vec![init];
            let mut current = init;
            loop {
                current = permutation[current];
                if current == init {
                    break;
                }
                orbit.push(current);
            }
            Orbit::new(&orbit)
        })
        .collect();
    vs.sort_unstable();
    vs.dedup();
    vs
}
