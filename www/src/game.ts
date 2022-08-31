import { Action } from './actions';

export interface TrainConfig {
  game_count?: number;
  winning_move_boost?: number;
  win_boost?: number;
  loose_boost?: number;
  tie_boost?: number;
}

export class GameWorker {
  worker = new Worker(new URL('./game.worker.ts', import.meta.url), { type: 'module' });

  static create() {
    return new Promise<GameWorker>((resolve) => {
      const game = new GameWorker();


      game.worker.addEventListener('message', (msg) => {
        if (msg.data.status === 'READY') {
          resolve(game);
        }
      });
    });
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
