# Hatch Result
A wrapper on a result that returns on Ok instead of Err when ? operator is used.  
This allows to exit a function with an Ok result if a computation has succeeded, or handle the error inside the function if it has failed  

Hatch comes from escape hatch, in the sense that the Ok result escapes early  
It's tough to come up with good names for things and I'll take suggestions, just open an issue!  

The tests are located in lib.rs  

## Examples
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