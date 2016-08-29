# SL(1): Cure your bad habit of mistyping

Rust clone of [original SL(1), originally written in C](https://github.com/mtoyoda/sl).

Still in WIP.

# Download and Install

You'll need Rust compiler and Cargo. With MacPorts, you can easily follow this:

```sh
$ sudo port install rust cargo
$ git clone git://github.com/kuenishi/sl
$ cd sl
$ cargo build
$ cargo install
$ export PATH=$HOME/.cargo/bin:$PATH
$ which sl
/Users/kuenishi/.cargo/bin/sl
$ sl
```

# Current status

- [x] Simple Scroll
- [x] Wheels
- [ ] Smokes
- [ ] Long
- [ ] Help!
- [ ] Documentation, manpage
- [ ] Cargo repository
