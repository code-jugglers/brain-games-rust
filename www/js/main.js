const train_btn = document.getElementById("train");
const reset_btn = document.getElementById("reset");
const results_container = document.getElementById("train_results");
const board = document.getElementById("board");

export async function main() {
  console.log("APP STARTING");

  const worker = await GameWorker.create();

  train_btn.addEventListener("click", async () => {
    results_container.innerHTML = "Training...";

    const training_results = await worker.train();

    results_container.innerHTML = training_results.replaceAll("\n", "<br>");
  });

  reset_btn.addEventListener("click", async () => {
    worker.reset_board();

    await render(worker);
  });

  render(worker);

  board.addEventListener("click", async (e) => {
    const index = Number(e.target.dataset.index);

    worker.play_x(index);

    await render(worker);
  });
}

async function render(worker) {
  board.innerHTML = "";

  const board_state = await worker.get_board();

  const res = board_state
    .split("")
    .map((space) => space.trim())
    .filter((space) => !!space)
    .map((space, i) => {
      const el = document.createElement("button");
      el.innerHTML = space;
      el.dataset.index = i;

      return el;
    });

  res.forEach((space, i) => {
    if (i > 0 && i % 3 === 0) {
      board.append(document.createElement("br"));
    }

    board.append(space);
  });
}

class GameWorker extends Worker {
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
