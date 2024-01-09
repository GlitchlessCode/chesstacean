"use strict";

class Board {
	gridwidth;
	gridheight;

	/** @type {number} */
	top;

	/** @type {number} */
	left;

	/** @type {number} */
	right;

	/** @type {number} */
	bottom;

	/** @type {number} */
	tilesize;

	/**
	 * @param {number} gridwidth
	 * @param {number} gridheight
	 */
	constructor(gridwidth, gridheight) {
		this.gridwidth  = gridwidth;
		this.gridheight = gridheight;
	}
}

export default Board;
