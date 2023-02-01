# Hatch Result

A wrapper over `std::result::Result` that returns on `Ok` instead of `Err` when the `?` operator is used.  

The builtin `Result`'s implementation of the `?` operator mimics the [short circuiting](https://en.wikipedia.org/wiki/Short-circuit_evaluation) on a logical **and**: if one operation returns an error, immediately return from the function. If not, proceed to the next statements.  

`HatchResult`'s implementation of the `?` operator mimics the short circuiting on a logical **or**: if one operation returns an `Ok`, immediately return from the function. If not, proceed to the next statements.  

This crate also implements a `hatch` method on the builtin `Result` type that wraps the value in a `HatchResult`.  

## Examples

### Typical use case

```rust
fn operation1() -> Result<u32, String> {
    Ok(4)
    // Err("Some error occurred")

}

fn operation2() -> Result<u32, String> {
    Ok(4)
    // Err("Some error occurred")
}

fn exit_early_if_possible() -> Result<u32, String> {
    let err1 = operation1().hatch()?;
    let err2 = operation2().hatch()?;
    handle_errors(err1,err2)
}

fn handle_errors(err1: String, err2: String) -> Result<u32, String> {
    Err(format!("Both operations failed:\n\top1: {err1}\n\top2: {err2}"))
}
```

### Improves on matching
HatchResult is more concise (but equivalent) to the traditional approach:
```rust
fn exit_early_if_possible() -> Result<u32, String> {
    let err = match operation1() {
        Err(error) => error,
        ok_result => return ok_result
    };
    // ... 
}

```
### Difference between regular `Result` and `HatchResult`
```rust
fn regular_result() -> Result<u32, String> {
    let value: u32 = Result::<u32, String>::Ok(4)?;
    Ok(value)
}

fn hatch_result() -> Result<u32, String> {
    let err: String = HatchResult::<u32, String>(Ok(3))?;
    Err(err)
}
```

### Converting from `Result`

The `hatch` method "converts" a result to a `HatchResult`.  
This allows you to exit early on an `Ok` result or handle the error.

```rust
fn operation_that_might_fail() -> Result<u32, String> {
    let result = // ... some computation
    result
}

fn hatch_result() -> Result<u32, String> {
    let error = operation_that_might_fail().hatch()?;
    panic!("There was an error: {error}");
}
```

