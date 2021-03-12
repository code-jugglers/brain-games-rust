import { Action } from "./actions.js";

export class GameWorker extends Worker {
  static create() {
    return new Promise((resolve) => {
      const worker = new GameWorker();

      worker.addEventListener("message", (msg) => {
        if (msg.data.status === "READY") {
          resolve(worker);
        }
      });
    });
  }

  constructor() {
    super("js/game.worker.js", { type: "module" });
  }

  train() {
    return this.run_command(Action.Train);
  }

  get_board() {
    return this.run_command(Action.GetBoard);
  }

  play_x(index) {
    return this.run_command(Action.PlayX, index);
  }

  play_bot_x() {
    return this.run_command(Action.PlayBotX);
  }

  play_o(index) {
    return this.run_command(Action.PlayO, index);
  }

  reset_board() {
    return this.run_command(Action.ResetBoard);
  }

  run_command(action, payload) {
    return new Promise((resolve) => {
      function listen(msg) {
        if (msg.data.status === `${action}_COMPLETE`) {
          this.removeEventListener("message", listen);

          resolve(msg.data.payload);
        }
      }

      this.addEventListener("message", listen);
      this.postMessage({ action, payload });
    });
  }
}
