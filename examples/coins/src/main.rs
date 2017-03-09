
extern crate hmm;

use hmm::models::{HiddenMarkov};


fn main() {
    let initials = vec![0.5, 0.5];
    let st = vec![ vec![0.75, 0.25],
                   vec![0.25, 0.75]];
    let obs = vec![ vec![0.5, 0.5],
                    vec![0.25, 0.75]];
    let hmm = HiddenMarkov::new(initials, st, obs).unwrap();
    let coins = hmm.map_estimate(vec![0, 0, 1, 1, 1]);
    println!("Coins used: {:?}", coins);
}
