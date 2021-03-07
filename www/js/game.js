export class GameWorker extends Worker {
  static create() {
    return new Promise((resolve) => {
      const worker = new GameWorker();

      worker.addEventListener('message', (msg) => {
        if (msg.data.status === 'READY') {
          resolve(worker);
        }
      });
    });
  }

  constructor() {
    super('js/game.worker.js', { type: 'module' });
  }

  train() {
    return this.run_command('TRAIN');
  }

  get_board() {
    return this.run_command('GET_BOARD');
  }

  play_x(index) {
    return this.run_command('PLAY_X', index);
  }

  play_bot_x() {
    return this.run_command('PLAY_BOT_X');
  }

  play_o(index) {
    return this.run_command('PLAY_O', index);
  }

  reset_board() {
    return this.run_command('RESET_BOARD');
  }

  run_command(action, payload) {
    return new Promise((resolve) => {
      function listen(msg) {
        if (msg.data.status === `${action}_COMPLETE`) {
          this.removeEventListener('message', listen);

          resolve(msg.data.message);
        }
      }

      this.addEventListener('message', listen);
      this.postMessage({ action, payload });
    });
  }
}
