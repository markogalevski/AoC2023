fn main() {
    println!("Hello, world!");
}

/*
Notes:
main part struct
{x, m, a, s}

workflows are a hashmap of:
{k:id,
 v: {Vec<Comparison>}
}

struct Comparison {
    characteristic: XMAS
    op: Lt/Gt,
    val: i64,
    target: &str
}

so, we build our hashmap of workflows (e.g. wf{x > 20:R, s>142:ww, a <2: A, R})

then we serde in our xmas parts.
Then, we send it in the "in" hash. If the target of a comparison is "A" or "R", push the part onto an accepted or rejected vector and move on.


the output result is the sum of all the x + m + a + s for all accepted parts.

*/
