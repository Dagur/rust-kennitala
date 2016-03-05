rust-kennitala
==============

Simple crate for handling Icelandic national identifiers

Works with rust 1.7 stable


##  Usage

Add this to your Cargo.toml


```toml
[dependencies.kennitala]
git = "https://github.com/Dagur/rust-kennitala.git"
```
And import and use like this

```rust
extern crate kennitala;
use kennitala::is_valid;

fn main(){
        if is_valid("012345-6789") {
                println!("Valid kennitala")
        } else {
                println!("Invalid")
        }
}
```
