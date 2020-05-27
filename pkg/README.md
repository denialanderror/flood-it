# Flood-It with Rust and WebAssembly

https://denialanderror.github.io/flood-it/

## How to play

The goal of Flood-It is to turn all squares the same colour. Click on a coloured square and the first square (top-left) will turn that same colour, along with any matching square adjacent to it. Continue to choose colours until either all squares match or you run out of turns.

## Implementation

This project was developed as a playground for Rust and WebAssembly. It went through a number of iterations and missteps/misunderstandings before settling in this final state. The initial idea came from reading the documentation for the web asset bundler [Parcel](https://parceljs.org/), which made the claim that it could import Rust files as WebAssembly with zero configuration, which turned out to be at best an overexageration of its abilities.

The final solution was to use the fantastic [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) library, which allowed for easy exposure of the game logic to the UI as easily ingestable JavaScript code. Initially, this was all being managed and bundled with Webpack but as the UI developed, it became clear that it was possible to do all that was needed without any further transpiliation or external dependencies. As a result, it was simple enough to simply serve the static files as they were and manually call `wasm-pack`.

## To run locally

You will need [Rust](https://www.rust-lang.org/tools/install) installed, along with [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) to compile the code to WebAssembly and provide friendly bindings for easy JavaScript interop, allowing the exposed API to be imported from `.js` files as you would with any other.

The game logic is written entirely in Rust and was a test suite that can be run with `cargo test`.

The UI is a single HTML file with a module script tag. As this needs to import the WebAssembly JS, you will need to run a web server to not run into CORS issues. Assuming you have Node installed, you should just be able to run `npx http-server` and visit `http://localhost:8080`.
