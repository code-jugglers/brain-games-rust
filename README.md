# "Brain" games in rust

[Try it here.](https://wasm-games.web.app/)

A simple tic tac toe bot.
The program that trains two "dumb" bots over 1,000,000 games, one playing Xs and one playing Os.
What you should observe is that the bots will begin to tie the further along they get until neither bot can gain an advantage.

The training portion of the program is written in rust and compiled with wasm-pack.
Those output files are loading via a Web Worker to keep the training loop off of the main thread.

## Running in browser

```BASH
npm run build
npm start
```

Then point a local server to www and play right away or train first.
The wasm specific code is in "lib" and the standard rust program starts in "main" (will be updated soon to play and train).

## Prebuilding Brain

If you want to prebuild brains you can run the rust program directly.

```BASH
cargo run --release
```

## NOTE

currently only works in chrome or chromium since I am using the "module" type of web worker. Will work fine in others with some bundling.

## Explanation:

The implementation here isn't machine learning as we would think about it with neural networks and the like. It is very brute force and isn't something that would scale much past something like "tic tac toe" since it requires the bot to be aware of all potential game states. This bot is an emulation of the match box "computer" described in the video below. A fun exercise and nothing more.

[![Alt text](https://img.youtube.com/vi/R9c-_neaxeU/0.jpg)](https://www.youtube.com/watch?v=R9c-_neaxeU)

## Observations

When training the bot the first time (500000) games. You will generally end up with ~10000 wins for Xs and ~5000 for Os with the rest being ties.
The bots should eventually no longer being
