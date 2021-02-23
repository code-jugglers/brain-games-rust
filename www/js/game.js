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
    return new Promise((resolve) => {
      function listen(msg) {
        if (msg.data.status === "TRAINING_COMPLETE") {
          this.removeEventListener("message", listen);

          resolve(msg.data.message);
        }
      }

      this.addEventListener("message", listen);
      this.postMessage({ action: "TRAIN" });
    });
  }

  get_board() {
    return new Promise((resolve) => {
      function listen(msg) {
        if (msg.data.status === "GET_BOARD_COMPLETE") {
          this.removeEventListener("message", listen);

          resolve(msg.data.message);
        }
      }

      this.addEventListener("message", listen);
      this.postMessage({ action: "GET_BOARD" });
    });
  }

  play_x(index) {
    return new Promise((resolve) => {
      function listen(msg) {
        if (msg.data.status === "PLAY_X_COMPLETE") {
          this.removeEventListener("message", listen);

          resolve(msg.data.message);
        }
      }

      this.addEventListener("message", listen);
      this.postMessage({ action: "PLAY_X", payload: index });
    });
  }

  reset_board() {
    return new Promise((resolve) => {
      function listen(msg) {
        if (msg.data.status === "RESET_BOARD_COMPLETE") {
          this.removeEventListener("message", listen);

          resolve();
        }
      }

      this.addEventListener("message", listen);
      this.postMessage({ action: "RESET_BOARD" });
    });
  }
}
