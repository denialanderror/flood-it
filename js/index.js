import { html, render } from "lit-html";

const classes = ["red", "green", "blue", "yellow", "purple"];
let game;
let squares;
let state;

const size = 30;

import("../pkg/index.js").then((module) => {
  game = module.Game.new(size, 5);
  squares = game.get_board();
  setState(game.get_state());
  render(board(), document.getElementById("game"));
});

const takeTurn = (position) => {
  game.take_turn(position);
  squares = game.get_board();
  setState(game.get_state());
  render(board(), document.getElementById("game"));
};

const setState = (newState) => {
  if (newState === "WIN") {
    state = "You win!";
  } else if (newState === "LOSE") {
    state = "You lose... :(";
  } else {
    state = `${newState} turns remaining`;
  }
};

const board = () => html` <div class="controls">${state}</div>
  <div
    class="board"
    style="grid-template: repeat(${size}, 1fr) / repeat(${size}, 1fr);"
  >
    ${squares.map(
      (n, i) =>
        html`<div
          class="square ${classes[n]}"
          @click=${() => takeTurn(i)}
        ></div>`
    )}
  </div>`;
