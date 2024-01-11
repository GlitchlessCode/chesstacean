"use strict";

const DEFAULT_GRID_WIDTH  = 26;
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
 */

/** @type {Board} */
const board = {};

board.gridwidth  = DEFAULT_GRID_WIDTH;
board.gridheight = DEFAULT_GRID_HEIGHT;
