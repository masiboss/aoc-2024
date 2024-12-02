use code_timing_macros::time_snippet;

mod bin;

fn main() {
    let _ = time_snippet!(bin::day01::main());
}