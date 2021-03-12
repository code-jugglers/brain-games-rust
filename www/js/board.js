import { html, render } from "https://unpkg.com/lit-html?module";

export class Board extends HTMLElement {
  set board_state(board_state) {
    this._board_state = board_state.split("");

    this.render();
  }

  set disabled(val) {
    this._disabled = val;

    this.render();
  }

  template() {
    return html`
      ${this._board_state.map((space, i) => {
        return html`
          ${i > 0 && i % 3 === 0 ? html`<br />` : ""}

          <button
            class=${space}
            data-index=${i}
            .disabled=${space !== "-" || this._disabled}
          >
            ${space !== "-" ? space : ""}
          </button>
        `;
      })}
    `;
  }

  render() {
    render(this.template(), this);
  }
}

customElements.define("xo-board", Board);
