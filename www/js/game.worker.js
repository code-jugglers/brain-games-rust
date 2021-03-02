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
          message: game.train(100000),
        });

        break;

      case "GET_BOARD":
        self.postMessage({
          status: "GET_BOARD_COMPLETE",
          message: game.board(),
        });

        break;

      case "PLAY_X":
        self.postMessage({
          status: "PLAY_X_COMPLETE",
          message: game.make_move_x(msg.data.payload),
        });

        break;

      case "PLAY_BOT_X":
        game.make_bot_move_x();

        self.postMessage({
          status: "PLAY_BOT_X_COMPLETE",
        });

        break;

      case "PLAY_O":
        self.postMessage({
          status: "PLAY_O_COMPLETE",
          message: game.make_move_o(msg.data.payload),
        });

        break;

      case "RESET_BOARD":
        game.reset_board();

        self.postMessage({ status: "RESET_BOARD_COMPLETE" });

        break;
    }
  };
}
