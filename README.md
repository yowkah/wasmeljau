# wasmeljau
experiment with calculating sections of a brezier curve using wasm

This is an experiment by Mats Veldhuizen and Jouke Ypma with combination of WASM Rust and Javascript.
It is used to take a SVG breziercurve path and then cut it at a specified point to return a section of it.

**run the demo**

clone the repo
```
git clone https://github.com/yowkah/wasmeljau.git
```


host the file from a webserver for example, a nodejs based on:
```
$ npm install -g http-server
$ http-server
```

visit the url returned by http-server and enjoy the functionality.


**build this project yourself** 

Build instructions are as follows:
clone the repo
```
git clone https://github.com/yowkah/wasmeljau.git
```

install rust if you haven't done so already and make sure to install the nightly toolchain
install the wasm32-unknown-unknown target
```
$ rustup update
$ rustup target add wasm32-unknown-unknown --toolchain nightly
```
build the wasm file
```
$ rustc +nightly --target wasm32-unknown-unknown -O main.rs
```
use a minifier to reduce the wasm filesize for use in browser
```
$ cargo install --git https://github.com/alexcrichton/wasm-gc
$ wasm-gc main.wasm main.min.wasm
```

host the file from a webserver for example, a nodejs based on:
```
$ npm install -g http-server
$ http-server
```

visit the url returned by http-server and enjoy the functionality.
