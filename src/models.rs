/// Graphical Models

use ndarray::{Array, Ix2};


type VertexId = u32;
/// Probability is a value from range [0, 1].
type Probability = f64;


/// Undirected Graphical Model (also known as Markov Random Field)
pub struct GraphicalModel {
    /// List of potential function. Probability for each vertex
    /// Note that they don't have to sum to 1. We can assume that we will scale them later
    vertices: Vec<Probability>,
    /// List of edges and their potentials
    /// The edges must adhere to the following rules:
    ///   * The VertexId < lower then number of vertices
    ///   * The can't make loops
    ///   * Marginalization in probability table along the axes must equal 1.
    edges: Vec<Edge>
}

/// Edge links 2 vertices and contains potential table
pub struct Edge {
    vertex1: VertexId,
    vertex2: VertexId,
    table: Array<Probability, Ix2>
}

impl Graph {
    /// Create new graphical model. Validate the edges (no loops)
    fn new(vertices: Vec<Probability>, edges: Vec<Edge>) -> Option<Graph> {
        None
    }

    // Marginalization
    // Compute marginal probability table px for every i ∊ 𝕧.
}

impl Edge {
    /// Create new Edge.
    /// Can fail if probability table contains wrong values or u == v.
    fn new(u: VertexId, v: VertexId, table: Array<Probability, Ix2>) -> Option<Edge> {
        None
    }
}