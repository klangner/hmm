//! # Hidden Markov Model


use ndarray::{Array, Ix1, Ix2};


type Probability = f64;
type LabelId = usize;
type StateId = usize;
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
    /// Please note that:
    ///   * initials should have more then 1 state. Its value should be in range [0, 1] and sum to 1.
    ///   * transitions should have exactly #state x #state elements
    fn new(initials: Vec<Probability>, transitions: Vec<Probability>,
           observation_model: Vec<Probability>) -> Option<HiddenMarkov>
    {
        // Check if initials is correct vector
        let num_states = initials.len();
        // We need at least 2 states
        if num_states < 2 || initials.iter().sum::<f64>() != 1.0 { return None; }
        // Transition needs num_state^2 elements
        let ts = Array::from_shape_vec((num_states, num_states), transitions).ok();
        // observation is matrix of size #num_states x #num_outcomes
        // And it should be bigger then 1
        let num_outcomes = observation_model.len() / num_states;
        if num_outcomes < 2 { return None; }
        let obs = Array::from_shape_vec((num_states, num_outcomes), observation_model).ok();

        ts.and_then(|t| obs.map(|b| {
            HiddenMarkov {
                init_states: Array::from_vec(initials),
                state_transitions: Array::from_shape_vec((2, 2), vec![5., 1., 1., 5.]).unwrap(),
                observation_model: Array::from_shape_vec((2, 2), vec![5., 1., 1., 5.]).unwrap(),
            }
        }))
    }

    /// Viterbi algorithm for estimating MAP (Maximum a posteriori)
    fn viterbi(&self, observations: Vec<LabelId>) -> Vec<StateId> {
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
        let st = vec![0.75, 0.25, 0.25, 0.75];
        let obs = vec![0.5, 0.5, 0.25, 0.75];

        assert!(HiddenMarkov::new(initials, st, obs).is_some());
    }

    #[test]
    fn test_new_none1() {
        let initials: Vec<f64> = vec![0.5, 0.7];
        let st = vec![0.75, 0.25, 0.25, 0.75];
        let obs = vec![0.5, 0.5, 0.25, 0.75];

        assert!(HiddenMarkov::new(initials, st, obs).is_none());
    }

    #[test]
    fn test_new_none2() {
        let initials: Vec<f64> = vec![0.5, 0.5];
        let st = vec![0.75, 0.25, 0.25];
        let obs = vec![0.5, 0.5, 0.25, 0.75];

        assert!(HiddenMarkov::new(initials, st, obs).is_none());
    }

    #[test]
    fn test_viterbi() {
        let initials: Vec<f64> = vec![0.5, 0.5];
        let st = vec![0.75, 0.25, 0.25, 0.75];
        let obs = vec![0.5, 0.5, 0.25, 0.75];
        let hmm = HiddenMarkov::new(initials, st, obs).unwrap();
        let map = hmm.viterbi(vec![0, 0, 1, 1, 1]);
        assert!(map == vec![0, 0, 1, 1, 1])
    }

}