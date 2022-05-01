## Current Contribution

1234abc

## Description

The program is exited when inputting an empty string.

## How to replicate this issue

Input an empty string when prompted.

## What should happen

The program should

## Optional: suspected code causing this issue

```rust
println!("Enter a username:");
let username = input();
if ! is_valid(username) {
return;
}
```

### Describe what's going wrong in the code

The code is checking `username`'s validity, and if it is invalid, it returns instead of providing a random string.

### Optional: Provide a fix

```rust
fn generate_random_username() -> String {
    /* code emitted */
}

println!("Enter a username:");
let mut username = input();

if !is_valid(username) {
    username = generate_random_username();
}
```