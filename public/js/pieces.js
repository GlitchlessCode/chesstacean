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

export default pieces;
