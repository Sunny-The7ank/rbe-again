/*
####### NOTES #######

fmt::Debug doesn't look compact and clean, so it's often better to customize the output
appearance. Done manually by implementing fmt::Display, which uses the {} print marker.
Implementation looks like:

    // Import (via `use`) the `fmt` module to make it available.
    use std::fmt;

    // Define a structure for which `fmt::Display` will be implemented. This is
    // a tuple struct named `Structure` that contains an `i32`.
    struct Structure(i32);

    // To use the `{}` marker, the trait `fmt::Display` must be implemented
    // manually for the type.
    impl fmt::Display for Structure {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }

fmt::Display may be cleaner than fmt::Debug but represents a problem for the std library.
How should ambiguous types be displayed? For example, if the std library implemented a 
single style for all Vec<T>, what would it be? Would it be either of these?

    Vec<path>: /:/etd:/home/username:/bin (split on :)
    Vec<number: 1,2,3 (split on ,)

No, because no ideal for all types and the std library doesn't dictate one.  fmt::Display
is not implemented for Vec<T> or for any other generic containers. fmt::Debug must be 
used for generic cases.

This isn't a problem though because for any new container type which isn't generic, fmt::Display
can be implemeted.



*/

