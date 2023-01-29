# Hatch Result

A wrapper over `std::result::Result` that returns on `Ok` instead of `Err` when the `?` operator is used.  
This will exit the function with an `Ok` result if the computation has succeeded, or allows the developer to handle the error inside the function if it has failed.

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

### Exiting early after success of a fallible function.

If the function succeeds, an `Ok` value is returned using the `?` operator.
If it fails, the expression evaluates to the error value.

```rust
fn operation_that_might_fail() -> HatchResult<u32, String> {
    let result = // ... some computation
    HatchResult(result)
}

fn hatch_result() -> Result<u32, String> {
    let error = operation_that_might_fail()?;
    panic!("There was an error: {error}");
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