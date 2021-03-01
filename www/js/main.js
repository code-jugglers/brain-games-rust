import { GameWorker } from "./game.js";

const train_btn = document.getElementById("train");
const reset_btn = document.getElementById("reset");
const play_o_btn = document.getElementById("play_o");
const results_container = document.getElementById("train_results");
const game_results_container = document.getElementById("game_results");
const board = document.getElementById("board");

let player = "X";

export async function main() {
  console.log("APP STARTING");

  const worker = await GameWorker.create();

  await update();

  train_btn.addEventListener("click", onTrainClick);
  reset_btn.addEventListener("click", onResetClick);
  play_o_btn.addEventListener("click", onPlayO);
  board.addEventListener("click", onBoardClick);

  async function onTrainClick() {
    let timer = 0;

    results_container.innerHTML = `Training... ${timer}s`;
    train_btn.disabled = true;
    reset_btn.disabled = true;
    board.disabled = true;

    const interval = setInterval(() => {
      timer++;

      results_container.innerHTML = `Training... ${timer}s`;
    }, 1000);

    const training_results = await worker.train();

    clearInterval(interval);

    results_container.innerHTML = training_results.replaceAll("\n", "<br>");
    train_btn.disabled = false;
    reset_btn.disabled = false;
    board.disabled = false;
  }

  async function onResetClick() {
    await worker.reset_board();

    await update();

    board.disabled = false;
    play_o_btn.disabled = false;
    game_results_container.innerHTML = "";
    player = "X";
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

      game_results_container.innerHTML =
        winner === "TIE" ? `It is a tie!` : `${winner} wins!`;
    }
  }

  async function update() {
    board.board_state = await worker.get_board();
  }
}
