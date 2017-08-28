# pretty_bit_mask
All this does is make bit masking operations a bit prettier.

## Example
```
let mut n = 0;

let m = 1;

n |= m;    // apply mask m normally
n.mask(m); // apply mask m prettily

assert!(n & m == m);  // check if mask m has been applied normally
assert!(n.masked(m)); // check if mask m has been applied prettily

n &= !m;     // unmask mask m normally
n.unmask(m); // unmask mask m prettily

n ^= m;    // flip mask m normally
n.flip(m); // flip mask m prettily
```
Typing out the pretty version takes a tad longer, but it can help make one's code clearer.

## How to Obtain
This program is available on crates.io [here](https://crates.io/crates/pretty_bit_mask). Just copy the large monospace text into the `Cargo.toml` file of your project.  

This project is so small, though, that maybe you should just copy the source code into your project.
