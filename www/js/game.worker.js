import init, { Game } from "../pkg/brain_games.js";

main().then(() => {
  console.log("GAME INITIALIZDED");
});

export async function main() {
  await init();

  const game = Game.new(); // initialize game

  self.postMessage({ status: "READY" });

  self.onmessage = (msg) => {
    switch (msg.data.action) {
      case "TRAIN":
        const message = game.train(3000000);

        self.postMessage({ status: "TRAINING_COMPLETE", message });
    }
  };
}
