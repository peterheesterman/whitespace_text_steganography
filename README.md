
# whitespace_text_steganography

(Zero width whitespace steganography)

This repo is a module for the commandline tool [`steg`](https://github.com/peterheesterman/steg) but can also be used independently.

It takes paths to two text files and hides the contents of the first in the second one, it can then reveal the hidden text again.


### Usage

Add the following to the Cargo.toml in your project:

```toml
[dependencies]
whitespace_text_steganography = "*"
```

Add a `payload.txt` and a `carrier.txt` to a folder called `texts`.

and import using ```extern crate```:

```rust
extern crate whitespace_text_steganography;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use whitespace_text_steganography::{ hide, reveal };

fn main () {
    let payload_path = "./texts/payload.txt";
    let carrier_path = "./texts/carrier.txt";
    let output_path = "./hidden.txt";

    let text = hide(payload_path, carrier_path);
    println!("\n-------{}------", output_path);
    println!("{}", text);

    let mut file = match File::create(output_path) {
        Err(why) => panic!("couldn't create because: {}", why.description()),
        Ok(file) => file,
    };
    
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write because: {}", why.description()),
        Ok(_) => (),
    }

    println!("\n--------The hidden message revealed---------");

    let hidden_message = reveal(output_path);
    println!("{}", hidden_message);
}
```
