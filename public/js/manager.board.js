"use strict";

const DEFAULT_FEN         = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
const DEFAULT_GRID_WIDTH  = 8;
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
 * @property {boolean} playingAsWhite
 * @property {boolean} reversePov
 * @property {Tile[][]} rows
 * @property {() => undefined} unmarkTiles
 */

/** @type {Board} */
const board = {};

board.rows           = [];
board.gridwidth      = DEFAULT_GRID_WIDTH;
board.gridheight     = DEFAULT_GRID_HEIGHT;
board.playingAsWhite = true;

board.unmarkTiles = function() {
	tileSelected = false;

	board.rows.forEach(row => {
		row.forEach(tile => {
			tile.mark = Tile.marks.none;
		});
	});

	requestAnimationFrame(update);
}

// parse initial board layout

let row = 0;
let col = 0;

for (let i = 0; i < DEFAULT_FEN.length; i++) {
	const character = DEFAULT_FEN[i];

	if (board.rows[row] == null)
		board.rows[row] = [];

	if (character === '/') {
		for (let j = col; j < board.gridwidth; j++) {
			board.rows[row][j] = new Tile();
		}

		row++;
		col = 0;

		continue;
	}

	if (Number.isInteger(+character)) {
		for (let i = 0; i < +character; i++) {
			if (col + i >= board.gridwidth)
				break;

			board.rows[row][col + i] = new Tile();
		}

		col += +character;
		continue;
	}

	if (col < board.gridwidth)
		board.rows[row][col] = new Tile(character);

	col++;
}

const lastRow = board.rows[board.rows.length - 1];

if (lastRow.length < board.gridwidth)
	for (let i = lastRow.length; i < board.gridwidth; i++)
		lastRow[i] = new Tile();
