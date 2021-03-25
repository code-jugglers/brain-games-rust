import {
  component,
  property,
  JoistElement,
  get,
  State,
} from '@joist/component';
import { template, html } from '@joist/component/lit-html';

interface XoBoardState {
  board_state: string;
  disabled: boolean;
}

@component<XoBoardState>({
  tagName: 'xo-board',
  state: {
    board_state: '---------',
    disabled: false,
  },
  render: template(({ state: { board_state, disabled } }) => {
    return html`
      ${board_state.split('').map((space, i) => {
        return html`
          ${i > 0 && i % 3 === 0 ? html`<br />` : ''}

          <button
            class=${space}
            data-index=${i}
            .disabled=${space !== '-' || disabled}
          >
            ${space !== '-' ? space : ''}
          </button>
        `;
      })}
    `;
  }),
})
export class Board extends JoistElement {
  @get(State)
  state!: State<XoBoardState>;

  @property()
  board_state: string = '---------';

  @property()
  disabled: boolean = false;

  onPropChanges() {
    const { board_state, disabled } = this;

    this.state.setValue({ board_state, disabled });
  }
}
