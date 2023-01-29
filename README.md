# Hatch Result

A wrapper on a result that returns on `Ok` instead of `Err` when `?` operator is used.  
This allows to exit a function with an `Ok` result if a computation has succeeded, or handle the error inside the function if it has failed

Regular `Result`'s `?` mimcs the [shortcircuiting](https://en.wikipedia.org/wiki/Short-circuit_evaluation) on a logical **and**: if one operation returns an error, immedtiatly return from the function. If not, proceed to the next statements.  

`HatchResult`'s `?` mimics the shortcircuiting on a logical **or**: if one operation returns an Ok, immediately return from the function. If not, proceed to the next statements.  

This crate also implements a `hatch` method on regular `Result` that returns self inside of a `HatchResult` wrapper.  

"Hatch" comes from escape hatch, in the sense that the Ok result escapes early  
It's tough to come up with good names for things and I'll take suggestions, just open an issue!  

The tests are located in lib.rs  

## Examples

### Tipical use case

```rust
fn operation1() -> Result<u32, String> {
    Result::<u32,String>::Ok(4)
    // Result::<u32,String>::Err("Some error occurred")

}

fn operation2() -> Result<u32, String> {
    Result::<u32,String>::Ok(4)
    // Result::<u32,String>::Err("Some error occurred")
}

fn exit_early_if_possible() -> Result<u32,String> {
    let err1 = operation1().hatch()?;
    let err2 = operation2().hatch()?;
    handle_errors(err1,err2)
}

fn handle_errors(err1: String, err2: String) -> Result<u32, String> {
    Err(format!("Both operations failed:\n\top1: {err1}\n\top2: {err2}"))
}
```

### Difference between regular result and HatchResult
```rust
fn regular_result() -> Result<u32, String> {
    let value: u32 = Result::<u32,String>::Ok(4)?;
    Ok(value)
}

fn hatch_result() -> Result<u32,String> {
    let err:String  = HatchResult::<u32,String>(Ok(3))?;
    Err(err)
}
```

### Exiting early after success of a fallible function.

If the function succeeds, an Ok Result is returned using ? operator.
If it fails, the expression evaluates to the error value.

```rust
fn operation_that_might_fail() -> HatchResult<u32,String> {
    let result = // ... some computation
    HatchResult(result)
}

fn hatch_result() -> Result<u32,String> {
    let error = operation_that_might_fail()?;
    panic!("There was an error: {error}");
}
```

### Converting from result

The `hatch` method "converts" a result to a HatchResult.  
This allows you to exit early on an Ok result or handle the error.

```rust
fn operation_that_might_fail() -> Result<u32,String> {
    let result = // ... some computation
    result
}

fn hatch_result() -> Result<u32,String> {
    let error = operation_that_might_fail().hatch()?;
    panic!("There was an error: {error}");
}
```
