import init, { Game } from '../pkg/brain_games.js';

export async function main() {
    await init();

    const board_container = document.getElementById("board");
    const results = document.getElementById("results");

    const game = Game.new();
    const res = game.play();

    board_container.innerHTML = game.board().replaceAll('\n', '<br>');
    results.innerHTML = res;
}