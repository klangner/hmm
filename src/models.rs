//! # Hidden Markov Model


use ndarray::{Array, Ix1, Ix2};


type Probability = f64;
type LabelId = usize;
type Matrix1 = Array<f64, Ix1>;
type Matrix2 = Array<f64, Ix2>;


/// Specialized structure for Hidden Markov Model of order 1
/// The states are identified by ids taken from natural numbers.
pub struct HiddenMarkov {
    /// Probability of starting states. Row Id == state id
    init_states: Matrix1,
    /// Probability table of switching states
    state_transitions: Matrix2,
    /// Observation model. This matrix contains states as a rows and possible outcomes as columns
    /// So the size of this matrix is: #states x #outcomes
    observation_model: Matrix2,
}

impl HiddenMarkov {
    /// Create a new Hidden Markov Model
    fn new(initials: Vec<Probability>, transitions: Vec<Vec<Probability>>,
           observation_model: Vec<Vec<Probability>>) -> Option<HiddenMarkov>
    {
        let num_states = initials.len();
        None
    }

    /// Viterbi algorithm for estimating MAP (Maximum a posteriori)
    fn viterbi(&self, observations: Vec<LabelId>) -> Vec<Probability> {
        vec![]
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
        let initials: Vec<f64> = vec![0.5, 0.5];
        let st = vec![vec![0.75, 0.25], vec![0.25, 0.75]];
        let obs = vec![vec![0.5, 0.5], vec![0.25, 0.75]];
        let m = HiddenMarkov::new(initials, st, obs);
        assert!(m.is_some());
    }

    #[test]
    fn test_viterbi() {
        let initials: Vec<f64> = vec![0.5, 0.5];
        let st = vec![vec![0.75, 0.25], vec![0.25, 0.75]];
        let obs = vec![vec![0.5, 0.5], vec![0.25, 0.75]];
        let hmm = HiddenMarkov::new(initials, st, obs).unwrap();
        let map = hmm.viterbi(vec![0, 0, 1, 1, 1]);
//        assert!(map == vec![0, 0, 1, 1, 1])
    }

}