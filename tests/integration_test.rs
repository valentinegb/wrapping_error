use std::num::ParseIntError;

use wrapping_error::wrapping_error;

#[test]
fn double_error() {
    wrapping_error!(DoubleError {
        EmptyVec => "please use a vector with at least one element",
        Parse(ParseIntError) => "the provided string could not be parsed as int",
    });
}
