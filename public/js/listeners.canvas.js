"use strict";

let firstRender        = true;
let freezePlayerMoving = false;

/** @type {Piece | false} */
let tileSelected = false;

function getCurrentTile(mouseX, mouseY) {
	const rect = canvas.cnv.getBoundingClientRect();

    const scaleX = canvas.cnv.width  / rect.width;
    const scaleY = canvas.cnv.height / rect.height;

	const mouse = {
		x: (mouseX - rect.left) * scaleX,
		y: (mouseY - rect.top ) * scaleY,
	};

	for (let row = 0; row < board.rows.length; row++) {
		for (let col = 0; col < board.rows[row].length; col++) {
			const tile = board.rows[row][col];

			if (mouse.y < tile.top)
				continue;

			if (mouse.x < tile.left)
				continue;

			if (mouse.x > tile.right)
				continue;

			if (mouse.y > tile.bottom)
				continue;

			return [tile, row, col];
		}
	}

	// outside of board

	return [undefined, undefined, undefined];
}

function runMove(from, to, row) {
	if ((from.piece.isWhite) && (row === 0) && (from.piece.constructor.name === Pawn.name)) {
		to.piece   = new Queen(true);
		from.piece = undefined;
	} else if ((!from.piece.isWhite) && (row === board.rows.length - 1) && (from.piece.constructor.name === Pawn.name)) {
		to.piece   = new Queen(false);
		from.piece = undefined;
	} else {
		to.piece   = from.piece;
		from.piece = undefined;
	}
}

canvas.cnv.addEventListener("mousedown", async e => {
	if (freezePlayerMoving)
		return;

	const [tile, row, col] = getCurrentTile(e.clientX, e.clientY);

	if (tileSelected) {
		if (tile.mark !== Tile.marks.none) {
			runMove(tileSelected, tile, row);
			board.unmarkTiles();
		} else {
			board.unmarkTiles();
			return;
		}

		// make response move
		// pretend to be player

		freezePlayerMoving = true;
		await new Promise(r => setTimeout(r, 500));
		freezePlayerMoving = false;

		board.playingAsWhite = false;

		const possibleTiles = [];
		board.rows.forEach((row, rowIndex) => row.forEach((tile, colIndex) => {
			if (tile.piece && !tile.piece.isWhite)
				possibleTiles.push({ tile: tile, row: rowIndex, col: colIndex });
		}));

		while (true) {
			if (possibleTiles.length === 0) {
				noPossibleMovesError();
				break;
			}

			const selectedTileIndex = Math.floor(Math.random() * possibleTiles.length);
			let selectedTileInfo = possibleTiles[selectedTileIndex];
			selectedTileInfo.tile.piece.markTiles(selectedTileInfo.row, selectedTileInfo.col);

			let selectedTile = board.rows[selectedTileInfo.row][selectedTileInfo.col];

			const possibleMoves = [];
			board.rows.forEach((row, rowIndex) => row.forEach(tile => {
				if (tile.mark !== Tile.marks.none)
					possibleMoves.push({ tile: tile, row: rowIndex });
			}));

			if (possibleMoves.length === 0) {
				possibleTiles.splice(selectedTileIndex, 1);
				continue;
			}

			let selectedMove = possibleMoves[Math.floor(Math.random() * possibleMoves.length)];
			runMove(selectedTile, selectedMove.tile, selectedMove.row);

			break;
		}

		board.playingAsWhite = true;
		board.unmarkTiles();

		return;
	}

	board.unmarkTiles();

	// tile is undefined if user clicked outside of board
	if (tile === undefined)
		return;

	// piece is undefined if user clicked on an empty tile
	if (tile.piece === undefined)
		return;

	// not your piece
	if (tile.piece.isWhite !== board.playingAsWhite)
		return;

	tileSelected = tile;

	tile.piece.markTiles(row, col);

	requestAnimationFrame(update);
});

// zooming

canvas.cnv.addEventListener("wheel", e => {
	e.preventDefault();

	// make zooming in faster the more zoomed out you are
	// and slower the more zoomed in you are
	const factor = (Math.max(board.gridwidth, board.gridheight) - camera.z) / Math.min(board.gridwidth, board.gridheight);

	camera.z -= Math.sign(e.deltaY) * factor;
	if (camera.z < 0) camera.z = 0;

	// - 2 ensures a minimum number of tiles
	const max = Math.max(board.gridwidth, board.gridheight) - 2;

	if (camera.z > max)
		camera.z = max;

	requestAnimationFrame(update);
});

// dragging

canvas.cnv.addEventListener("mousedown", e => {
	const rect = canvas.cnv.getBoundingClientRect();

	canvas.dragging = {
		x: e.clientX - rect.left + camera.x,
		y: e.clientY - rect.top  + camera.y,
	};
});

canvas.cnv.addEventListener("mousemove", e => {
	if (firstRender) {
		requestAnimationFrame(update);
		firstRender = false;
	}

	if (!canvas.dragging)
		return;

	const rect = canvas.cnv.getBoundingClientRect();

	canvas.cnv.setAttribute("width",  rect.width );
	canvas.cnv.setAttribute("height", rect.height);

	camera.x = canvas.dragging.x - (e.clientX - rect.left);
	camera.y = canvas.dragging.y - (e.clientY - rect.top );

	requestAnimationFrame(update);
});

addEventListener("mouseup", () => {
	canvas.dragging = false;
});
