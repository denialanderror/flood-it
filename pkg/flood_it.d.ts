/* tslint:disable */
/* eslint-disable */
/**
*/
export function main_js(): void;
/**
*/
export class Game {
  free(): void;
/**
* @param {number} size 
* @param {number} colours 
* @returns {Game} 
*/
  static new(size: number, colours: number): Game;
/**
* @returns {Array<any>} 
*/
  get_board(): Array<any>;
/**
* @param {number} position 
*/
  take_turn(position: number): void;
/**
* @returns {string} 
*/
  get_state(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main_js: () => void;
  readonly __wbg_game_free: (a: number) => void;
  readonly game_new: (a: number, b: number) => number;
  readonly game_get_board: (a: number) => number;
  readonly game_take_turn: (a: number, b: number) => void;
  readonly game_get_state: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        