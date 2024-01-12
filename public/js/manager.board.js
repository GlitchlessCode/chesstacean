"use strict";

const DEFAULT_FEN         = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
const DEFAULT_GRID_WIDTH  = 12;
const DEFAULT_GRID_HEIGHT = 8;

/**
 * @typedef  {object} Board
 * @property {number} top
 * @property {number} left
 * @property {number} right
 * @property {number} bottom
 * @property {number} tilesize
 * @property {number} gridwidth
 * @property {number} gridheight
 * @property {(Piece|undefined)[][]} rows
 */

/** @type {Board} */
const board = {};

board.rows       = [];
board.gridwidth  = DEFAULT_GRID_WIDTH;
board.gridheight = DEFAULT_GRID_HEIGHT;

// parse initial board layout

let row = 0;
let col = 0;

Array.from(DEFAULT_FEN).forEach(character => {
	if (character === '/') {
		row++;
		col = 0;

		return;
	}

	if (Number.isInteger(+character)) {
		col += +character;
		return;
	}

	if (board.rows[row] == null)
		board.rows[row] = [];

	board.rows[row][col] = pieces[character];

	col++;
});
