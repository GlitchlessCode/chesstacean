export class Move {
	startPosition;
	finalPosition;

	/**
	 * @param {Position} startPosition
	 * @param {Position} finalPosition
	 */
	constructor(startPosition, finalPosition) {
		this.startPosition = startPosition;
		this.finalPosition = finalPosition;
	}
}

export class Position {
	x;
	y;

	/**
	 * @param {number} x
	 * @param {number} y
	 */
	constructor(x, y) {
		this.x = x;
		this.y = y;
	}
}
