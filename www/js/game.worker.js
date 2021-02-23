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
        const message = game.train(1000000);

        self.postMessage({ status: "TRAINING_COMPLETE", message });

        break;

      case "GET_BOARD":
        self.postMessage({
          status: "GET_BOARD_COMPLETE",
          message: game.board(),
        });

        break;

      case "PLAY_X":
        console.log("#######", msg.data);

        const index = msg.data.payload;

        game.make_move_x(index);

        self.postMessage({ status: "PLAY_X_COMPLETE" });

        break;
    }
  };
}
