import {  attr, observable, observe, OnPropertyChanged } from '@joist/observable';
import { styled, css } from '@joist/styled';
import { queryAll } from '@joist/query';

export class BoardChangeEvent extends Event {
  constructor(public index: number) {
    super('boardchange');
  }
}

@observable
@styled
export class BoardElement extends HTMLElement implements OnPropertyChanged {
  static styles = [
    css`
      :host {
        padding: 1rem 0;
        min-height: 300px;
        width: 100%;
        display: flex;
        flex-wrap: wrap;
      }

      * {
        box-sizing: border-box;
      }

      button {
        background: none;
        border: none;
        flex: 0 0 33.333333%;
        height: calc(300px / 3);
        font-size: 1.5rem;
        cursor: pointer;
        margin: 0;
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

  @observe 
  @attr({ 
    read: val => val.split(''),
    write: val => val.join('')
  }) 
  board_state = ['-', '-', '-', '-', '-', '-', '-', '-', '-'];

  @observe @attr disabled = false;

  @queryAll('button')
  spaces!: NodeListOf<HTMLButtonElement>

  constructor() {
    super();

    const shadow = this.attachShadow({ mode: 'open' });

    this.board_state.forEach((space, i) => {
      if(i > 0 && i % 3 === 0) {
        shadow.appendChild(document.createElement('br'));
      }

      const btn = document.createElement('button');
      btn.id = i.toString();
      btn.className = space;
      btn.disabled = space !== '-' || this.disabled;
      btn.innerHTML = space !== '-' ? space : '';
      btn.onclick = () => {
        this.dispatchEvent(new BoardChangeEvent(i))
      };

      shadow.appendChild(btn);
    });
  }

  onPropertyChanged(): void {
    this.spaces.forEach((btn, i) => {
      const space = this.board_state[i];

      btn.className = space;
      btn.disabled = space !== '-' || this.disabled;
      btn.innerHTML = space !== '-' ? space : '';
    });
  }
}


