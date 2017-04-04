# Hidden Markov Model 

[![Crates.io](https://img.shields.io/crates/v/hmm.svg)](https://crates.io/crates/hmm) [![Crates.io](https://img.shields.io/crates/l/hmm.svg)](https://github.com/klangner/hmm/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/hmm/badge.svg)](https://docs.rs/hmm/)

**hmm is an early-stage open-source project**. It means that API can change at any time.
If you think that this library can help you, then let me know. We can discuss future direction and try to stabilize the API.



## Features
   
  * [x] Viterbi MAP estimation


## Example

Lets say that we have 2 coins:
  * Fair which generates H (Head) and T (Tails) with probability of 1/2
  * Biased - with probabilities H: 1/4, T: 3/4

We also know that after each toss we can switch coin with the probability of
  * Use the same coin: 3/4
  * Switch coin: 1/4

First time we select coin with probability of 1/2

Using this library we can answer the following question:
 
Given the observations 'H H T T T' which coins were used during each toss?


Lest build HMM model and check the answer:


```rust
extern crate hmm;

use hmm::models::{HiddenMarkov};


fn main() {
    // 50%:50% chance to start at either coin
    let initials = vec![0.5, 0.5];

    // state transitions between coins:
    //   3/4 to stay on same coin (on matrix diagonals)
    //   1/4 chance to switch
    let st = vec![ vec![0.75, 0.25],
                   vec![0.25, 0.75]];

    // observation probabilities for each coin:
    //    first coin is fair, second one is biased
    let obs = vec![ vec![0.5, 0.5],
                    vec![0.25, 0.75]];

    // generate the HMM model
    let hmm = HiddenMarkov::new(initials, st, obs).unwrap();
    let coins = hmm.map_estimate(vec![0, 0, 1, 1, 1]);
    println!("Coins used: {:?}", coins);
}
```
 
For more check [Examples](https://github.com/klangner/hmm/tree/master/examples). 

  
# License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
