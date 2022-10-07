// I'm interested in how well Rust can be used to define embedded DSLs.
//
// One major factor for usability of an embedded DSL is whether it's
// simple to include source location information in the tree making up
// the DSL; if so, then we can provide reasonably good diagnostics.

#[derive(Debug)]
struct DslIdentifier {
    name: &'static str,
    file: &'static str,
    line: u32,
}

macro_rules! dsl_identifier {
    ($name: ident) => {
        DslIdentifier {
            name: stringify!($name),
            file: file!(),
            line: line!(),
        }
    }
}

fn main() {
    let x = dsl_identifier!(x);
    println!("The dsl identifer is {:?}", x);
}
