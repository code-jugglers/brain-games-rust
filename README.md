# "Brain" games in rust

A simple tic tac toe bot.
The program that trains two "dumb" bots over 3,000,000 games, one playing Xs and one playing Os.
What you should observe is that the bots will begin to tie the further along they get until neither bot can gain an advantage.

The training portion of the program is written in rust and compiled with wasm-pack.
Those output files are loading via a Web Worker to keep the training loop off of the main thread.

```BASH
wasm-pack build --target web -d www/pkg
```

Then open point a local server to www and play right away or train first.
The wasm specific code is in "lib" and the standard rust program starts in "main" (will be updated soon to play and train).

## NOTE
currently only works in chrome or chromium since I am using the "module" type of web worker. Will work fine in others with some bundling.

## Explanation:

[![Alt text](https://img.youtube.com/vi/R9c-_neaxeU/0.jpg)](https://www.youtube.com/watch?v=R9c-_neaxeU)
