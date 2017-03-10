
extern crate hmm;

use hmm::models::{HiddenMarkov};


/// A: 0, C: 1, G: 2, T: 3
/// coding: 0, non-coding: 1
fn main() {
    let initials = vec![0.5, 0.5];
    let st = vec![ vec![0.98, 0.02],
                   vec![0.02, 0.98]];
    let obs = vec![ vec![0.18, 0.32, 0.32, 0.18],
                    vec![0.25, 0.25, 0.25, 0.25]];
    let hmm = HiddenMarkov::new(initials, st, obs).unwrap();
    // ATGCGA
    let coins = hmm.map_estimate(vec![0, 3, 2, 1, 2, 0]);
    println!("states used: {:?}", coins);
}
