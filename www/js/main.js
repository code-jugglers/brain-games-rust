import { GameWorker } from "./game.js";

const train_btn = document.getElementById("train");
const reset_btn = document.getElementById("reset");
const play_o_btn = document.getElementById("play_o");
const board = document.getElementById("board");
const title = document.getElementById("title");

let player = "X";
let has_trained = false;

export async function main() {
  console.log("APP STARTING");

  const worker = await GameWorker.create();

  await update();

  train_btn.addEventListener("click", onTrainClick);
  reset_btn.addEventListener("click", onResetClick);
  play_o_btn.addEventListener("click", onPlayO);
  board.addEventListener("click", onBoardClick);

  async function onTrainClick() {
    title.innerHTML = "Let me practice for a bit.";

    await worker.reset_board();

    await update();

    let timer = 0;

    train_btn.innerHTML = timer;

    disable();

    const interval = setInterval(() => {
      timer++;

      train_btn.innerHTML = timer;
    }, 1000);

    const training_result = await worker.train();

    console.log(training_result);

    clearInterval(interval);

    train_btn.innerHTML = "Train Again";

    enable();

    title.innerHTML = "Now I am ready!";

    has_trained = true;
  }

  async function onResetClick() {
    await worker.reset_board();

    await update();

    enable();

    player = "X";
    title.innerHTML = "Can you beat me?";
  }

  async function onPlayO() {
    player = "O";
    play_o_btn.disabled = true;

    await worker.play_bot_x();
    await update();
  }

  async function onBoardClick(e) {
    play_o_btn.disabled = true;

    const index = Number(e.target.dataset.index);

    let winner;

    if (player === "X") {
      winner = await worker.play_x(index);
    } else {
      winner = await worker.play_o(index);
    }

    await update();

    if (winner) {
      board.disabled = true;

      console.log(`Game Result: ${winner}`);

      if (winner === player) {
        if (has_trained) {
          title.innerHTML = `You win!`;
        } else {
          title.innerHTML = `Wait! I wasn't ready. \n`;
        }
      } else if (winner !== "TIE") {
        if (has_trained) {
          title.innerHTML = `Gotcha! Well Played`;
        } else {
          title.innerHTML = `Wow I wasn't even trying!`;
        }
      } else {
        title.innerHTML = `A tie! Well played!`;
      }
    }
  }

  async function update() {
    board.board_state = await worker.get_board();
  }
}

function disable() {
  train_btn.disabled = true;
  reset_btn.disabled = true;
  play_o_btn.disabled = true;
  board.disabled = true;
}

function enable() {
  train_btn.disabled = false;
  reset_btn.disabled = false;
  play_o_btn.disabled = false;
  board.disabled = false;
}
