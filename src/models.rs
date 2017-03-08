//! # Hidden Markov Model
//!
// Implementation details
// for numerical stability this library operates on log values and uses addition
// instead of multiplication.  And since we are interested in probabilities in range [0, 1]
// We will operate on -log. So instead of max probability we will minimize log probabilities.


use ndarray::{Array, Axis, ArrayView1, Ix1, Ix2};


type Probability = f64;
type LabelId = usize;
type StateId = usize;
type Matrix1 = Array<f64, Ix1>;
type Matrix2 = Array<f64, Ix2>;


/// Specialized structure for Hidden Markov Model of order 1
/// The states are identified by ids taken from natural numbers.
// The values in this structure are converted to the log.
pub struct HiddenMarkov {
    // Number of states
    state_count: usize,
    // Number of labels (different observation types)
    observation_count: usize,
    // Probability of starting states. Row Id == state id
    init_states: Matrix1,
    // Probability table of switching states
    state_transitions: Matrix2,
    // Observation model. This matrix contains states as a rows and possible outcomes as columns
    // So the size of this matrix is: #states x #outcomes
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
        if num_states < 2 { return None }
        if initials.iter().any(|&x| x < 0.) { return None; }
        if transitions.len() != num_states.pow(2) { return None; }
        if transitions.iter().any(|&x| x < 0.) { return None; }
        if observation_model.len() / num_states < 2 { return None; }
        if observation_model.iter().any(|&x| x < 0.) { return None; }

        // We need -log values.
        let num_outcomes = observation_model.len() / num_states;
        let initials_log: Vec<f64> = initials.iter().map(|x| -x.log2()).collect();
        let tx_log: Vec<f64> = transitions.iter().map(|x| -x.log2()).collect();
        let ts = Array::from_shape_vec((num_states, num_states), tx_log).ok();
        let obs_log: Vec<f64> = observation_model.iter().map(|x| -x.log2()).collect();
        let obs = Array::from_shape_vec((num_states, num_outcomes), obs_log).ok();

        ts.and_then(|t| obs.map(|b| {
            HiddenMarkov {
                state_count: num_states,
                observation_count: num_outcomes,
                init_states: Array::from_vec(initials_log),
                state_transitions: t,
                observation_model: b,
            }
        }))
    }

    /// Calculate MAP (Maximum a posteriori) using Viterbi algorithm
    /// As a input provide list of observations and as a output this function will provide
    /// The most probable sequence of states which generates such observations
    /// ## Example
    /// Lets say that we have 2 coins:
    ///   * Fair which generates H (Head) and T (Tails) with probability of 1/2
    ///   * Biased - with probabilities H: 1/4, T: 3/4
    ///
    /// We also know that after each toss we can switch coin with the probability of
    ///   * Use the same coin: 3/4
    ///   * Switch coin: 1/4
    ///
    /// First time we select coin with probability of 1/2
    ///
    /// Question: If we now get observation of H H T T T which coins were used during each toss?
    ///
    /// Lest build HMM model for this example and check the anwser:
    ///
    /// let initials: Vec<f64> = vec![0.5, 0.5];
    /// let st = vec![0.75, 0.25, 0.25, 0.75];
    /// let obs = vec![0.5, 0.5, 0.25, 0.75];
    /// let hmm = HiddenMarkov::from_vec(initials, st, obs).unwrap();
    /// hmm.map_estimate(vec![0, 0, 1, 1, 1]) == vec![0, 0, 1, 1, 1]
    ///
    /// Observation Vec should:
    ///   * Have at least 1 observation
    ///   * Each LabelId should be less then maximum number of observation in HMM model
    pub fn map_estimate(&self, observations: Vec<LabelId>) -> Vec<StateId> {
        // Validate input
        if observations.len() == 0 {return vec![]}
        if observations.iter().any(|&x| x >= self.observation_count) { return vec![]; }

        let phi = &(self.init_states) + &(self.state_from_observation(observations[0]));
        let m_1_2 = self.msg_table(&phi);
        println!("ϕ_1 = {:?}", phi);
        println!("msg_table = {:?}", m_1_2);
        vec![3]
    }

    // The probability of being in given state based on the observation.
    // This probability is column in observation_model
    fn state_from_observation(&self, obs: LabelId) -> ArrayView1<Probability> {
        self.observation_model.column(obs)
    }

    // Calculate message table
    // This table is build from probability distribution of being in given state
    // and transition table. Since we are in -log, we need to add state dist to each column.
    fn msg_table(&self, phi: &Matrix1) -> Matrix2 {
        self.state_transitions.axis_iter(Axis(1)).collect()
//        self.state_transitions.axis_iter(Axis(1)).map(|col|{
//            col
//        }).collect::<Matrix2>()

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
    fn test_map_estimation() {
        let initials: Vec<f64> = vec![0.5, 0.5];
        let st = vec![0.75, 0.25, 0.25, 0.75];
        let obs = vec![0.5, 0.5, 0.25, 0.75];
        let hmm = HiddenMarkov::from_vec(initials, st, obs).unwrap();
        let estimate = hmm.map_estimate(vec![0, 0, 1, 1, 1]);
        println!("MAP estimate {:?}", estimate);
        assert!(estimate == vec![0, 0, 1, 1, 1])
    }

}