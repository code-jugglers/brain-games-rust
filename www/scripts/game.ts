import { Action } from './actions';

export class GameWorker extends Worker {
  static create() {
    return new Promise<GameWorker>((resolve) => {
      const worker = new GameWorker();

      worker.addEventListener('message', (msg) => {
        if (msg.data.status === 'READY') {
          resolve(worker);
        }
      });
    });
  }

  constructor() {
    super(new URL('game.worker.js', import.meta.url), { type: 'module' });
  }

  train() {
    return this.run_command(Action.Train);
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
          this.removeEventListener('message', listen);

          resolve(msg.data.payload);
        }
      };

      this.addEventListener('message', listen);
      this.postMessage({ action, payload });
    });
  }
}
