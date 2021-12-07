import { observable, observe, OnChange } from '@joist/observable';
import { render, html } from 'lit-html';

@observable
export class BoardElement extends HTMLElement implements OnChange {
  @observe board_state = '---------';
  @observe disabled = false;

  connectedCallback() {
    this.render();
  }

  onChange() {
    this.render();
  }

  private template() {
    return html`
      ${this.board_state.split('').map((space, i) => {
        return html`
          ${i > 0 && i % 3 === 0 ? html`<br />` : ''}
          <button
            class=${space}
            data-index=${i}
            .disabled=${space !== '-' || this.disabled}
          >
            ${space !== '-' ? space : ''}
          </button>
        `;
      })}
    `;
  }

  private render() {
    render(this.template(), this);
  }
}

customElements.define('xo-board', BoardElement);
