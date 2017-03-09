
extern crate hmm;

use hmm::models::{HiddenMarkov};
use hmm::matrices::{Vector, Matrix};


fn main() {
    let initials: Vector = Vector::new(vec![0.5, 0.5]);
    let st = Matrix::new(vec![ vec![0.75, 0.25],
                               vec![0.25, 0.75]]).unwrap();
    let obs = Matrix::new(vec![ vec![0.5, 0.5],
                                vec![0.25, 0.75]]).unwrap();
    let hmm = HiddenMarkov::new(initials, st, obs).unwrap();
    let coins = hmm.map_estimate(vec![0, 0, 1, 1, 1]);
    println!("Coins used: {:?}", coins);
}
