import { observable, observe, OnChange } from '@joist/observable';
import { styled, css } from '@joist/styled';
import { render, html } from 'lit-html';

export class BoardChangeEvent extends Event {
  constructor(public index: number) {
    super('boardchange');
  }
}

@observable
@styled
export class BoardElement extends HTMLElement implements OnChange {
  static styles = [
    css`
      :host {
        display: flex;
        flex-wrap: wrap;
        padding: 1rem 0;
        min-height: 300px;
      }

      button {
        background: none;
        border: none;
        flex: 0 0 33.333333%;
        height: calc(300px / 3);
        font-size: 1.5rem;
        cursor: pointer;
      }

      button:disabled {
        background: none;
      }

      button.X {
        color: #140078;
      }

      button.O {
        color: #9c27b0;
      }

      button:nth-child(2n) {
        border-left: solid 1px gray;
        border-right: solid 1px gray;
      }

      button:nth-child(5),
      button:nth-child(6),
      button:nth-child(7) {
        border-top: solid 1px gray;
        border-bottom: solid 1px gray;
      }
    `,
  ];

  @observe board_state = '---------';
  @observe disabled = false;

  constructor() {
    super();

    this.attachShadow({ mode: 'open' });
  }

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
            @click=${() => this.onClick(i)}
            .disabled=${space !== '-' || this.disabled}
          >
            ${space !== '-' ? space : ''}
          </button>
        `;
      })}
    `;
  }

  private render() {
    render(this.template(), this.shadowRoot!);
  }

  private onClick(index: number) {
    this.dispatchEvent(new BoardChangeEvent(index));
  }
}

customElements.define('xo-board', BoardElement);
