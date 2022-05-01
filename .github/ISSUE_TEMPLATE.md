## Current Contribution

1234abc

## Description

The program is exited when inputting an invalid username.

## How to replicate this issue

Input an invalid username. For example, and empty one.

## What should happen

The program should resort to a random username.

## Optional: suspected code causing this issue

```rust
fn main() {
    println!("Enter a username:");
    let username = input();

    if !is_valid(username) {
        return;
    }
    /* code emitted */
}
```

### Describe what's going wrong in the code

The code is checking `username`'s validity, and if it is invalid, it returns instead of providing a random string.

### Optional: Provide a fix

```rust
fn generate_random_username() -> String {
    /* code emitted */
}

fn main() {
    println!("Enter a username:");
    let mut username = input();

    if !is_valid(username) {
        username = generate_random_username();
    }
    /* code emitted */
}
```