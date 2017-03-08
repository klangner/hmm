//! # Hidden Markov Model
//!
// Implementation details
// for numerical stability this library operates on log values and uses addition
// instead of multiplication.  And since we are interested in probabilities in range [0, 1]
// We will operate on -log. So instead of max probability we will minimize log probabilities.


use ndarray::{Array, Ix1, Ix2};


type Probability = f64;
type LabelId = usize;
type StateId = usize;
type Matrix1 = Array<f64, Ix1>;
type Matrix2 = Array<f64, Ix2>;


/// Specialized structure for Hidden Markov Model of order 1
/// The states are identified by ids taken from natural numbers.
// The values in this structure are converted to the log.
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
    ///   * initials should have more then 1 state. Its values should be positive
    ///   * transitions should have exactly #state x #state elements. Values positive
    ///   * observation_model should have #state x #outcomes elements. Values positive
    fn from_vec(initials: Vec<Probability>, transitions: Vec<Probability>,
                observation_model: Vec<Probability>) -> Option<HiddenMarkov>
    {
        let num_states = initials.len();
        // Validate parameters
        if num_states < 2 {return None }
        if initials.iter().any(|&x| x < 0.) { return None; }
        if transitions.len() != num_states.pow(2) { return None; }
        if transitions.iter().any(|&x| x < 0.) { return None; }
        if observation_model.len() / num_states < 2 { return None; }
        if observation_model.iter().any(|&x| x < 0.) { return None; }

        // We need log values.
        let num_outcomes = observation_model.len() / num_states;
        let initials_log: Vec<f64> = initials.iter().map(|x| x.log2()).collect();
        let tx_log: Vec<f64> = transitions.iter().map(|x| x.log2()).collect();
        let ts = Array::from_shape_vec((num_states, num_states), tx_log).ok();
        let obs_log: Vec<f64> = observation_model.iter().map(|x| x.log2()).collect();
        let obs = Array::from_shape_vec((num_states, num_outcomes), obs_log).ok();

        ts.and_then(|t| obs.map(|b| {
            HiddenMarkov {
                init_states: Array::from_vec(initials_log),
                state_transitions: t,
                observation_model: b,
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

        assert!(HiddenMarkov::from_vec(initials, st, obs).is_some());
    }

    #[test]
    fn test_new_none1() {
        let initials: Vec<f64> = vec![0.5, -0.5];
        let st = vec![0.75, 0.25, 0.25, 0.75];
        let obs = vec![0.5, 0.5, 0.25, 0.75];

        assert!(HiddenMarkov::from_vec(initials, st, obs).is_none());
    }

    #[test]
    fn test_new_none2() {
        let initials: Vec<f64> = vec![0.5, 0.5];
        let st = vec![0.75, 0.25, 0.25];
        let obs = vec![0.5, 0.5, 0.25, 0.75];

        assert!(HiddenMarkov::from_vec(initials, st, obs).is_none());
    }

    #[test]
    fn test_viterbi() {
        let initials: Vec<f64> = vec![0.5, 0.5];
        let st = vec![0.75, 0.25, 0.25, 0.75];
        let obs = vec![0.5, 0.5, 0.25, 0.75];
        let hmm = HiddenMarkov::from_vec(initials, st, obs).unwrap();
        let map = hmm.viterbi(vec![0, 0, 1, 1, 1]);
//        assert!(map == vec![0, 0, 1, 1, 1])
    }

}