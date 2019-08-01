use std::cmp::{Ord, Ordering};

type EdgeIndex = usize;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
struct Vertex {
    /// rotated to the minimum comes first
    edges: Vec<EdgeIndex>,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Vertex) -> Ordering {
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

impl Vertex {
    fn new(edges: &[EdgeIndex]) -> Self {
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
        Vertex { edges: rotated }
    }
}

#[derive(Debug, Clone)]
struct Permutations {
    next: Vec<EdgeIndex>,
}

impl Permutations {
    fn new(next: &[EdgeIndex]) -> Self {
        assert_eq!(next.len() % 2, 0);
        // TODO Can we check the permutation is even easily?
        Permutations {
            next: next.to_vec(),
        }
    }

    fn len(&self) -> usize {
        self.next.len()
    }

    fn next_twin_orbit(&self, init: EdgeIndex) -> Vertex {
        let mut orbit = vec![init];
        let mut current = init;
        loop {
            let twin = if current % 2 == 0 {
                current + 1
            } else {
                current - 1
            };
            current = self.next[twin];
            if current == init {
                break;
            }
            orbit.push(current);
        }
        Vertex::new(&orbit)
    }

    fn vertices(&self) -> Vec<Vertex> {
        let n = self.len();
        let mut vs: Vec<_> = (0..n).map(|init| self.next_twin_orbit(init)).collect();
        vs.sort_unstable();
        vs.dedup();
        vs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_new() {
        let v = Vertex::new(&[2, 1, 3]);
        assert_eq!(
            v,
            Vertex {
                edges: vec![1, 3, 2]
            }
        );
    }

    #[test]
    fn permutations() {
        let p = Permutations::new(&[2, 7, 4, 1, 6, 3, 0, 5]);
        dbg!(&p);
        let v = p.vertices();
        dbg!(&v);
        panic!()
    }
}
