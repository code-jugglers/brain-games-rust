import init, { Game } from "../pkg/brain_games.js";

main().then(() => {
  console.log("GAME INITIALIZED!");
});

export async function main() {
  await init();

  const game = Game.new(); // initialize game

  self.postMessage({ status: "READY" });

  self.onmessage = (msg) => {
    switch (msg.data.action) {
      case "TRAIN":
        self.postMessage({
          status: "TRAINING_COMPLETE",
          message: game.train(1000000),
        });

        break;

      case "GET_BOARD":
        self.postMessage({
          status: "GET_BOARD_COMPLETE",
          message: game.board(),
        });

        break;

      case "PLAY_X":
        const index = msg.data.payload;

        self.postMessage({
          status: "PLAY_X_COMPLETE",
          message: game.make_move_x(index),
        });

        break;

      case "RESET_BOARD":
        game.reset_board();

        self.postMessage({ status: "RESET_BOARD_COMPLETE" });

        break;
    }
  };
}
