/// Remember the passed value as the solution for the current puzzle.
/// Prints a specially formatted line so that the solution can be found from the justfile.
pub fn solved_level_1<S: ToString>(s: S) {
    println!("level-1-solution={}", s.to_string());
}

/// See [solved_level_1].
pub fn solved_level_2<S: ToString>(s: S) {
    println!("level-2-solution={}", s.to_string());
}
