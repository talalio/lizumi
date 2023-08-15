
# Lizumi

A simple remote access trojan that i built as a challenge to myself. This was built in a 
week, with no internet access and no external packages, using only rust's local documentation 
as a help.

The payload generation command isn't done yet, and some commands are not implemented on the payload (e.g. scrshot).

To build and run the server clone this repository and run `cargo run listen [lhost] [lport]` :

```shell
$ git clone https://github.com/talalio/lizumi.git
$ cd liznmi
$ cargo run listen 127.0.0.1 1337
```

To generate a payload move to `lizumi/src/pawn/payload/`, change the listening host and port in the source file `pawn.rs` and run
the build file `./build.sh`, note that you will need to have rust installed to compile the payload.


### The name ?

Linux + ねずみ = Lizumi


 **This RAT was built for educational purposes.**
