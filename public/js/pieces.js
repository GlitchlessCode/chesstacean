"use strict";

const pieces = {};

['r', 'n', 'b', 'q', 'k', 'p'].forEach(piece => {
	const black = piece;
	const white = piece.toUpperCase();

	pieces[black] = new Image();
	pieces[white] = new Image();

	pieces[black].src = `./img/pieces/b${black}.svg`;
	pieces[white].src = `./img/pieces/w${white}.svg`;
});

class Piece {
	/** @type {CanvasImageSource} */
	image;

	/** @type {boolean} */
	isWhite;

	/** Mark tiles with possible moves for this piece. */
	markTiles(row, col) {}

	constructor(isWhite) {
		this.isWhite = isWhite;
	}
}

// Returns true if should break iteration
function markTile(row, col) {
	const tile = board.rows[row][col];

	if (tile.piece === undefined) {
		tile.mark = Tile.marks.available;
		return false;
	}

	if (tile.piece.isWhite === board.playingAsWhite)
		return true;

	tile.mark = Tile.marks.capture;
	return true;
}

function markStraightFrom(row, col) {
	// leftwards

	for (let c = col - 1; c >= 0; c--)
		if (markTile(row, c))
			break;

	// upwards

	for (let r = row - 1; r >= 0; r--)
		if (markTile(r, col))
			break;

	// rightwards

	for (let c = col + 1; c < board.gridwidth; c++)
		if (markTile(row, c))
			break;

	// downwards

	for (let r = row + 1; r < board.gridheight; r++)
		if (markTile(r, col))
			break;
}

function markDiagonallyFrom(row, col) {
	// top-left

	// TODO: Make these loops more readable if have the time
	for (let [r, c] = [row - 1, col - 1]; r >= 0 && c >= 0; [r--, c--])
		if (markTile(r, c))
			break;

	// top-right

	for (let [r, c] = [row - 1, col + 1]; r >= 0 && c < board.gridwidth; [r--, c++])
		if (markTile(r, c))
			break;

	// bottom-right

	for (let [r, c] = [row + 1, col + 1]; r < board.gridheight && c < board.gridwidth; [r++, c++])
		if (markTile(r, c))
			break;

	// bottom-left

	for (let [r, c] = [row + 1, col - 1]; r < board.gridheight && c >= 0; [r++, c--])
		if (markTile(r, c))
			break;
}

class Rook extends Piece {
	markTiles(row, col) {
		if (board.playingAsWhite !== this.isWhite)
			return;

		markStraightFrom(row, col);
	}

	constructor(isWhite) {
		super(isWhite);

		this.image = isWhite ? pieces['R'] : pieces['r'];
	}
}

class Knight extends Piece {
	markTiles(row, col) {}

	constructor(isWhite) {
		super(isWhite);

		this.image = isWhite ? pieces['N'] : pieces['n'];
	}
}

class Bishop extends Piece {
	markTiles(row, col) {
		if (board.playingAsWhite !== this.isWhite)
			return;

		markDiagonallyFrom(row, col);
	}

	constructor(isWhite) {
		super(isWhite);

		this.image = isWhite ? pieces['B'] : pieces['b'];
	}
}

class Queen extends Piece {
	markTiles(row, col) {
		if (board.playingAsWhite !== this.isWhite)
			return;

		markStraightFrom(row, col);
		markDiagonallyFrom(row, col);
	}

	constructor(isWhite) {
		super(isWhite);

		this.image = isWhite ? pieces['Q'] : pieces['q'];
	}
}

class King extends Piece {
	markTiles(row, col) {}

	constructor(isWhite) {
		super(isWhite);

		this.image = isWhite ? pieces['K'] : pieces['k'];
	}
}

class Pawn extends Piece {
	markTiles(row, col) {}

	constructor(isWhite) {
		super(isWhite);

		this.image = isWhite ? pieces['P'] : pieces['p'];
	}
}

class Tile {
	/** @type {Piece|undefined} */
	piece;

	/** @type {number} */
	top;

	/** @type {number} */
	left;

	/** @type {number} */
	right;

	/** @type {number} */
	bottom;

	/** @type {string} */
	mark;

	static marks = {
		available: "circle",
		capture: "square",
		none: "none",
	}

	/** @param {string} character */
	constructor(character) {
		this.mark = Tile.marks.none;

		if (character == null)
			return;

		const isWhite = character === character.toUpperCase();

		switch (character.toLowerCase()) {
			case 'r':
				this.piece = new Rook(isWhite);
				break;
			case 'n':
				this.piece = new Knight(isWhite);
				break;
			case 'b':
				this.piece = new Bishop(isWhite);
				break;
			case 'q':
				this.piece = new Queen(isWhite);
				break;
			case 'k':
				this.piece = new King(isWhite);
				break;
			case 'p':
				this.piece = new Pawn(isWhite);
				break;
		}
	}
}
