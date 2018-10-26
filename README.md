# (Work in progress)

# whitespace_text_steganography

(Zero width whitespace steganography)

This repo is a module for the commandline tool [`steg`](https://github.com/peterheesterman/steg) but can also be used independently

It takes paths to two text files and hide the contents of the first in the second one, it can then reveal the hidden text again.


### Usage

Add the following to the Cargo.toml in your project:

```toml
[dependencies]
whitespace_text_steganography = "*"
```

and import using ```extern crate```:

```rust
extern crate whitespace_text_steganography;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use whitespace_text_steganography::{ hide, reveal };

fn run () {
    let payload_path = "./text/payload.txt";
    let carrier_path = "./text/carrier.txt";
    let output_path = "./hidden.txt";

    let text = hide(payload_path, carrier_path);
    println!(text);

    let mut file = match File::create(output_path) {
        Err(why) => panic!("couldn't create because: {}", why.description()),
        Ok(file) => file,
    };
    
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write because: {}", why.description()),
        Ok(_) => (),
    }

    let hidden_message = reveal(output_path);
    println!(hidden_message);
}
```

## Documentation

Read it. . .(coming soon)


