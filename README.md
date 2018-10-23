
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

use whitespace_text_steganography::{ hide, reveal };

fn run () {
    let payload_path = "./text/payload.txt";
    let carrier_path = "./text/carrier.txt";

    let text = hide(payload_path, carrier_path);
    println!(text);

    // reveal coming soon...
}
```

## Documentation

Read it. . .(coming soon)


