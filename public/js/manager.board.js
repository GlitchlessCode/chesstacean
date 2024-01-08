"use strict";

/**
 * @type     {object}
 * @property {number} tilesize
 * @property {() => undefined} recalculateTilesize
 */
const board = {};

board.recalculateTilesize = function() {
	// adjust grid sizes based on zoom

	const gridW = Math.max(gridW - camera.zoom, 0);
	const gridH = Math.max(gridH - camera.zoom, 0);

	// determine tile sizes based on grid width

	const tileW = canvas.width  / gridW;
	const tileH = canvas.height / gridH;

	board.tilesize = Math.min(gridW, gridH);
};
