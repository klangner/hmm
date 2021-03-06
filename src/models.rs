//! # Hidden Markov Model
//!
//! ## Example
//!
//! Lets say that we have 2 coins:
//!   * Fair which generates H (Head) and T (Tails) with probability of 1/2
//!   * Biased - with probabilities H: 1/4, T: 3/4
//!
//! We also know that after each toss we can switch coin with the probability of
//!   * Use the same coin: 3/4
//!   * Switch coin: 1/4
//!
//! First time we select coin with probability of 1/2
//!
//! Question: If we now get observation of H H T T T which coins were used during each toss?
//!
//! Lest build HMM model for this example and check the answer:
//!
//! ```rust
//! let initials: Vec<f64> = vec![0.5, 0.5];
//! let st = vec![0.75, 0.25, 0.25, 0.75];
//! let obs = vec![0.5, 0.5, 0.25, 0.75];
//! let hmm = HiddenMarkov::from_vec(initials, st, obs).unwrap();
//! hmm.map_estimate(vec![0, 0, 1, 1, 1]) == vec![0, 0, 1, 1, 1]
//! ```
//!

// Implementation details
// for numerical stability this library operates on log values and uses addition
// instead of multiplication.  And since we are interested in probabilities in range [0, 1]
// We will operate on -log. So instead of max probability we will minimize log probabilities.

use matrices::{Vector, Matrix};


type LabelId = usize;
type StateId = usize;


/// Specialized structure for Hidden Markov Model of order 1
/// The states are identified by ids taken from natural numbers.
// The values in this structure are converted to the log.
pub struct HiddenMarkov {
    // Number of labels (different observation types)
    labels_count: usize,
    // Probability of starting states. Row Id == state id
    init_states: Vector,
    // Probability table of switching states
    state_transitions: Matrix,
    // Observation model. This matrix contains states as a rows and possible outcomes as columns
    // So the size of this matrix is: #states x #outcomes
    observation_model: Matrix,
}

impl HiddenMarkov {
    /// Create a new Hidden Markov Model
    /// Please note that:
    ///   * initials should have more then 1 state. Its values should be positive
    ///   * transitions should have exactly #state x #state elements. Values positive
    ///   * observation_model should have #state x #outcomes elements. Values positive
    ///
    /// Params:
    ///   - initials - Initial probability for each state
    ///   - transition - Probability of changing state from x_1 to x_2 (state x state)
    ///   - observation_matrix - Probability of generating outcome in each state (state x outcome)
    ///
    /// Observation Vec should:
    ///   * Have at least 1 observation
    ///   * Each LabelId should be less then maximum number of observation in HMM model
    pub fn new(initials: Vec<f64>, transitions: Vec<Vec<f64>>,
               observation_model: Vec<Vec<f64>>) -> Option<HiddenMarkov>
    {
        let num_states = initials.len();
        let is = Vector::new(initials);
        let ts = Matrix::new(transitions);
        let os = Matrix::new(observation_model);


        // Validate parameters
        if num_states < 2 { return None }
        if !is.is_positive() { return None }
        if ts.is_none() { return None }
        let trans = ts.unwrap();
        if !trans.is_positive() { return None }
        if os.is_none() { return None }
        let obs = os.unwrap();
        if !obs.is_positive() { return None }

        // We need -log values.
        let num_outcomes = obs.cols();
        let is_log: Vector = is.minus_log();
        let trans_log: Matrix = trans.minus_log();
        let obs_log: Matrix = obs.minus_log();

        Some(
            HiddenMarkov {
                labels_count: num_outcomes,
                init_states: is_log,
                state_transitions: trans_log,
                observation_model: obs_log
            }
        )

    }

    /// Calculate MAP (Maximum a posteriori) using Viterbi algorithm
    /// As a input provide list of observations and as a output this function will provide
    /// The most probable sequence of states which generates such observations
    pub fn map_estimate(&self, observations: Vec<LabelId>) -> Vec<StateId> {
        // Validate input
        let obs_len = observations.len();
        if obs_len == 0 {return vec![]}
        if observations.iter().any(|&x| x >= self.labels_count) { return vec![]; }

        let mut last_msg: Vector = self.init_states.clone();
        let mut tracebacks: Vec<Vec<StateId>> = Vec::with_capacity(obs_len);

        for i in 0..observations.len() {
            // Based on the last message calculate ϕ_i + last_msg
            let phi = last_msg.add_vector(&self.state_from_observation(observations[i]));
            // Add transition
            let (msg, t) = self.next_msg_and_traceback(&phi);
            last_msg = msg;
            tracebacks.push(t);
        }

        // Based on the last message select most probable end state
        let mut states: Vec<StateId> = vec![0; obs_len];
        let mut last_state = last_msg.argmin();
        for i in (0..obs_len).rev() {
            let state: StateId = tracebacks[i][last_state];
            last_state = state;
            states[i] = state;

        }

        states
    }

    // The probability of being in given state based on the observation.
    // This probability is column in observation_model
    fn state_from_observation(&self, obs: LabelId) -> Vector {
        self.observation_model.column(obs).unwrap()
    }

    /// Calculate message and traceback
    /// Message is minimal value across columns, trace back is argmax from columns
    fn next_msg_and_traceback(&self, phi: &Vector) -> (Vector, Vec<StateId>) {
        let mat = self.state_transitions.add_to_columns(phi);
        (mat.min_by_column(), mat.argmin_by_column())
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
        let st = vec![ vec![0.75, 0.25],
                       vec![0.25, 0.75]];
        let obs = vec![ vec![0.5, 0.5],
                        vec![0.25, 0.75]];

        assert!(HiddenMarkov::new(initials, st, obs).is_some());
    }

    #[test]
    fn test_new_none1() {
        let initials: Vec<f64> = vec![0.5, -0.5];
        let st = vec![ vec![0.75, 0.25],
                       vec![0.25, 0.75]];
        let obs = vec![ vec![0.5, 0.5],
                        vec![0.25, 0.75]];

        assert!(HiddenMarkov::new(initials, st, obs).is_none());
    }

    #[test]
    fn test_map_estimation() {
        let initials: Vec<f64> = vec![0.5, 0.5];
        let st = vec![ vec![0.75, 0.25],
                       vec![0.25, 0.75]];
        let obs = vec![ vec![0.5, 0.5],
                        vec![0.25, 0.75]];
        let hmm = HiddenMarkov::new(initials, st, obs).unwrap();
        let estimate = hmm.map_estimate(vec![0, 0, 1, 1, 1]);
        assert!(estimate == vec![0, 0, 1, 1, 1])
    }

}