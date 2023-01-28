# Hatch Result
A wrapper on a result that returns on Ok instead of Err when ? operator is used
Hatch comes from escape hatch, in the sense that the Ok result escapes early
It's tough to come up with good names for things and I'll take suggestions, just open an issue!

The tests are located in lib.rs

## Examples
```rust
fn disjunct_function() -> HatchResult<u32,String> {
    HatchResult(Result::<u32, String>::Ok())
}

fn test() -> u32{
    let err = disjunct_function()?
    panic!("There was an error: {err}");
}
```
