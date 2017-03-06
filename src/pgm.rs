/// Probabilistic Graphical Models

use ndarray::{Array, Ix1, Ix2};


type VertexId = usize;
type Vector = Array<f64, Ix1>;
type Matrix = Array<f64, Ix2>;


/// Undirected Graphical Model (also known as Markov Random Field)
pub struct GraphModel {
    /// List of potential function.
    /// Potentials have to be positive number but they are not probabilities.
    /// They don't have to sum to 1. We can assume that we will scale them later
    vertices: Vec<Vector>,
    /// List of edges and their potentials
    /// The edges must adhere to the following rules:
    ///   * The VertexId < lower then number of vertices
    ///   * The can't make loops
    ///   * Marginalization in probability table along the axes must equal 1.
    edges: Vec<Edge>
}

/// Edge links 2 vertices and contains potential table
pub struct Edge {
    /// First vertex
    u: VertexId,
    /// Second vertex
    v: VertexId,
    /// Potentials table
    table: Matrix
}

impl GraphModel {
    /// Create new graphical model. Validate the edges (no loops)
    /// Can fail if:
    ///  * any potential < 0
    ///  * edges reference nodes without potentials
    fn new(vertices: Vec<Vector>, edges: Vec<Edge>) -> Option<GraphModel> {
        let n = vertices.len();
        if edges.iter().any(|ref x| x.u < n && x.v < n){
            Some(GraphModel { vertices: vertices, edges: edges })
        } else {
            None
        }
    }

    // Marginalization
    // Compute marginal probability table px for every i ∊ 𝕧.
    // Using sum - product algorithm as a starting point
}

impl Edge {
    /// Create new Edge.
    /// Can fail if u == v or table is empty.
    fn new(u: VertexId, v: VertexId, table: Matrix) -> Option<Edge> {
        if u != v && table.len() > 0 {
            Some(Edge { u: u, v: v, table: table })
        } else {
            None
        }
    }
}



/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vs = vec![Array::from_vec(vec![1.]),
                      Array::from_vec(vec![1.])];
        let edges = vec![
            Edge::new(0, 1, Array::from_shape_vec((2, 2), vec![5., 1., 1., 5.]).unwrap()).unwrap(),
            Edge::new(1, 2, Array::from_shape_vec((2, 2), vec![0., 1., 1., 0.]).unwrap()).unwrap(),
        ];
        let m = GraphModel::new(vs, edges);
        assert!(m.is_some());
    }
}