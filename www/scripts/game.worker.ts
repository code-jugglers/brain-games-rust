/// <reference lib="WebWorker" />

import init, { Game } from '../pkg/brain_games';

import { Action, ActionComplete } from './actions';

main().then(() => {
  console.log('GAME INITIALIZED!');
});

export async function main() {
  await init();

  const [bot_x_brain, bot_o_brain] = await Promise.all([
    fetch('../bot_x_brain.bin').then((res) => res.arrayBuffer()),
    fetch('../bot_o_brain.bin').then((res) => res.arrayBuffer()),
  ]).then((res) => res.map((buffer) => new Uint8Array(buffer)));

  const game = Game.new(); // initialize game

  // game.load_x_brain(bot_x_brain);
  // game.load_o_brain(bot_o_brain);

  self.postMessage({ status: 'READY' }); // signal that game is ready

  // Listen for actions
  self.onmessage = (msg: MessageEvent) => {
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
