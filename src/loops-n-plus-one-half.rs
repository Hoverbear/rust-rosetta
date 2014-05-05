// Implements http://rosettacode.org/wiki/Loops/N_plus_one_half

#[cfg(not(test))]
use std::iter;

#[cfg(not(test))]
fn main() {
    for i in iter::range_inclusive(1,10) {
        print!("{}", i);
        if i == 10 {
            break;
        }
        print!(", ");
    }
}
