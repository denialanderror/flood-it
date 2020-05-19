import("../pkg/index.js").then((module) => {
  const game = module.Game.new(5, 5);
  console.log(game.get_board());
  game.take_turn(0, 3);
  console.log(game.get_board());
});
