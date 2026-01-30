fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    // FIX: Replaced 'for' loop with 'if let'
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
