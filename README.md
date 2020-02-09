# "Brain" games in rust

A simple tic tac toe bot.
The program will spit out the results incrementally over 3,000,000 games.
What you should observe is that the bots will begin to tie the further along they get until neither bot can gain an advantage.

After a round of training is complete the bots will dump their "brains" into local files and use them fo continued rounds. You can also explore these files to see which moves a bot is most likesly to take for a given board state.

Once the bots have been trained you can play against them. Enter moves by enter first the column and then the row separated by a space. For example the top left corner would be "0 0" and the center right would be 2 1.

```BASH
# build program
cargo build --release
```

```BASH
# run training program
./target/release/brain_games train
```

```BASH
./target/release/brain_games play_x # play against the x bot
./target/release/brain_games play_o # play against the o bot
```

## Example Results:

![Results Image](/images/game-results.png)

## Explanation:

[![Alt text](https://img.youtube.com/vi/R9c-_neaxeU/0.jpg)](https://www.youtube.com/watch?v=R9c-_neaxeU)
