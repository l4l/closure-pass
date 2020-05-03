closure-pass is a crate for passing arguments to a closure with capture feature of C++ lambdas.

# Usage

So far, this crate requires two nightly features: `stmt_expr_attributes` and `proc_macro_hygiene`. Usage is pretty straightforward, the following code:

```rust
let a = /*..*/;
let b = /*..*/;

#[closure_pass(a, b = b.f()]
move || {
    // ..
}
```

Will expand to something like:

```rust
let a = /*..*/;
let b = /*..*/;

{
    let a = a.clone();
    let b = b.f();
    move || {
        // ..
    }
}
```
