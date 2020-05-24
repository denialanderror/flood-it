import { html, render } from "lit-html";

const classes = ["red", "green", "blue", "yellow", "purple"];
let game;
let squares;
let state;

const size = 30;

import("../pkg/index.js").then((module) => {
  game = module.Game.new(size, 5);
  squares = game.get_board();
  state = game.get_state();
  render(board(), document.body);
});

const takeTurn = (position) => {
  game.take_turn(position);
  squares = game.get_board();
  state = game.get_state();
  render(board(), document.body);
};

const board = () => html` <h1>${state}</h1>
  <div
    class="board"
    style="grid-template: repeat(${size}, 50px) / repeat(${size}, 50px);"
  >
    ${squares.map(
      (n, i) =>
        html`<div
          class="square ${classes[n]}"
          @click=${() => takeTurn(i)}
        ></div>`
    )}
  </div>`;
