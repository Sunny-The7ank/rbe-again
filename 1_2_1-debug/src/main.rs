/*
Notes:
All types that want to use std::fmt formatting traits require an implementation 
to be printable.  Auto implementations are only provided for types in the std
library.  All others mus be manually implemented somehow



// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

All std library types are automatically printable with {:?} too
*/

