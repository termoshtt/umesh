#[derive(Debug, Clone)]
struct Mesh {
    vertex_edge: Vec<(usize, usize)>,
    edge_face: Vec<(usize, usize)>,
}
