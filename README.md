# threadr (work in progress)

A node.js module for spawning processes in separate threads.

To start, you'll need to have the Rust programming language installed along with Neon (`npm i -g neon`).

To run this, you'll first need to compile the sample timeout script: `gcc timeout.c -o timeout`.

Once you've compiled that, build the rust bindings by running `neon build --release`.

To test, start a node process and run:

```
require(".")
```
