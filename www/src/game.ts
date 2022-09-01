import { Action } from './actions';

export interface TrainConfig {
  game_count?: number;
  winning_move_boost?: number;
  win_boost?: number;
  loose_boost?: number;
  tie_boost?: number;
}

const initializeToken = Symbol();

export class Game {
  worker = new Worker(new URL('./game.worker.ts', import.meta.url), { type: 'module' });

  static create() {
    return new Promise<Game>((resolve) => {
      const game = new Game(initializeToken);


      game.worker.addEventListener('message', (msg) => {
        if (msg.data.status === 'READY') {
          resolve(game);
        }
      });
    });
  }

  constructor(token: Symbol) {
    if(token !== initializeToken) {
      throw new Error('Game needs to be initialized async. Use Game.create instead');
    }
  }

  train(train_config: TrainConfig) {
    return this.run_command(Action.Train, train_config);
  }

  get_board(): Promise<string> {
    return this.run_command<string>(Action.GetBoard);
  }

  play_x(index: number) {
    return this.run_command<string>(Action.PlayX, index);
  }

  play_bot_x() {
    return this.run_command(Action.PlayBotX);
  }

  play_o(index: number) {
    return this.run_command<string>(Action.PlayO, index);
  }

  reset_board() {
    return this.run_command(Action.ResetBoard);
  }

  private run_command<T = void>(action: string, payload?: any) {
    return new Promise<T>((resolve) => {
      const listen = (msg: MessageEvent) => {
        if (msg.data.status === `${action}_COMPLETE`) {
          this.worker.removeEventListener('message', listen);

          resolve(msg.data.payload);
        }
      };

      this.worker.addEventListener('message', listen);
      this.worker.postMessage({ action, payload });
    });
  }
}
