import { html, render } from 'https://unpkg.com/lit-html?module';

export class Board extends HTMLElement {
  set board_state(board_state) {
    this._board_state = board_state.split('');

    this.render();
  }

  set disabled(val) {
    this._disabled = val;

    this.render();
  }

  template() {
    return html`
      ${this._board_state.map((space, i) => {
        const render = [];

        if (i > 0 && i % 3 === 0) {
          render.push(html`<br />`);
        }

        render.push(
          html`
            <button
              data-index=${i}
              .disabled=${space !== '-' || this._disabled}
            >
              ${space}
            </button>
          `
        );

        return render;
      })}
    `;
  }

  render() {
    render(this.template(), this);
  }
}

customElements.define('xo-board', Board);
