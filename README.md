# "Brain" games in rust

A simple tic tac toe bot. You can't play against it yet but you can watch two bots play against each other.
The program will spit out the results incrementally over 3,000,000 games.
What you should observe is that the bots will begin to tie the further along they get until neither bot can gain an advantage.

After a round of training is compelte the bots will dump their "brains" into local files and use them fo continued rounds. You can also explore these files to see which moves a bot is most likesly to take for a given board state.

```BASH
cargo run --release
```

## Example Results:

![Results Image](/images/game-results.png)

## Explanation:

[![Alt text](https://img.youtube.com/vi/R9c-_neaxeU/0.jpg)](https://www.youtube.com/watch?v=R9c-_neaxeU)
