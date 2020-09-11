fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}

/*
Notes:
Printing is handled by a series of macros defined in std::fmt some of which include:
format! : write formatted text to String
print! : same as format! but text is printed to the console (io::stdout)
println! : same as print! but a new line is appended
eprint! : same as format! but text is printed to standard error (io:stderr)
eprintln! : same as eprint! but new line is appended

####### Code block from above #######

std::fmt contains many traits which govern the display of text.  The base
form of two important ones are listed below:

fmt::Debug : uses the {:?} marker. Format text for debugging purposes.
fmt::Display : uses the {} marker. Format text in more elegant, user friendly fashion

Here, we used fmt::Display because the std library provides implementatiosn for these types.
To print text for custom types, more steps are required.

Implementing the fmt::Diplsay trait automatically implements the ToString trait which allows
us to convert the type to String
*/