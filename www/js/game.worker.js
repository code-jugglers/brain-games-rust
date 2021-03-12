import { Action, ActionComplete } from "./actions.js";
import init, { Game } from "../pkg/brain_games.js";

main().then(() => {
  console.log("GAME INITIALIZED!");
});

export async function main() {
  await init();

  const game = Game.new(); // initialize game

  self.postMessage({ status: "READY" }); // signal that game is ready

  // Listen for actions
  self.onmessage = (msg) => {
    switch (msg.data.action) {
      case Action.Train:
        self.postMessage({
          status: ActionComplete.Train,
          payload: game.train(500000),
        });

        break;

      case Action.GetBoard:
        self.postMessage({
          status: ActionComplete.GetBoard,
          payload: game.board(),
        });

        break;

      case Action.PlayX:
        self.postMessage({
          status: ActionComplete.PlayX,
          payload: game.make_move_x(msg.data.payload),
        });

        break;

      case Action.PlayBotX:
        self.postMessage({
          status: ActionComplete.PlayBotX,
          payload: game.make_bot_move_x(),
        });

        break;

      case Action.PlayO:
        self.postMessage({
          status: ActionComplete.PlayO,
          payload: game.make_move_o(msg.data.payload),
        });

        break;

      case Action.ResetBoard:
        self.postMessage({
          status: ActionComplete.ResetBoard,
          payload: game.reset_board(),
        });

        break;
    }
  };
}
