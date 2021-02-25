# "Brain" games in rust

A simple tic tac toe bot.
The program that trains two "dumb" bots over 1,000,000 games, one playing Xs and one playing Os.
What you should observe is that the bots will begin to tie the further along they get until neither bot can gain an advantage.

The training portion of the program is written in rust and compiled with wasm-pack.
Those output files are loading via a Web Worker to keep the training loop off of the main thread.

## Running in browser

```BASH
wasm-pack build --target web -d www/pkg
```

Then open point a local server to www and play right away or train first.
The wasm specific code is in "lib" and the standard rust program starts in "main" (will be updated soon to play and train).

## Running native

Will run one single game (for now)

```BASH
cargo run --release
```

## NOTE

currently only works in chrome or chromium since I am using the "module" type of web worker. Will work fine in others with some bundling.

## Explanation:

[![Alt text](https://img.youtube.com/vi/R9c-_neaxeU/0.jpg)](https://www.youtube.com/watch?v=R9c-_neaxeU)

## Observations

When training the bot the first time (1000000) games. You will generally end up with ~65000 wins for Xs and ~35000 for Os with the rest being ties.
If you train them a seecond time those numbers drop to just a couple hundred each and eventually trailing off with neither X or O being able to win any games.
Since at the heart the decisions are "random" there is still a chance that either bot could loose a game but it is very unlikely.

The starting value for each potential move impacts the numbers above a significant amount as well as how well the bot will play against a person.
If the number is to low (3) then it is possible for a play to throw non obvious moves at the bot and it will fail to play optimally. If that number is to high (100) then it takes significantly MORE games then 1000000 to teach the bot to play well. The current values sits at 10.

The bots don't generally find the optimal solution. They will sometimes block instead of going for the win but if and only if they will win at the end. A win is a win.

Ex. (Bot is O)

Given this game, the bot is just as likely to take index 1 as index 0. The end result is the same but the bot "plays with" it's oponent. Could be improved by boosting winning moves.

| | | |
|-|-|-|
| | |X|
|X|O|X|
| | |O|

Could be follow by this:
| | | |
|-|-|-|
| |O|X|
|X|O|X|
| | |O|

Or this:
| | | |
|-|-|-|
|O| |X|
|X|O|X|
| | |O|

To the bot both moves are equally good since they both lead to a winning result.
