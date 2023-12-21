"use strict";

import { Coordinate } from "./components.js";

// canvas setup

/** @type {HTMLCanvasElement} */
const cnv = document.getElementById("game-board");
const ctx = cnv.getContext('2d');

// canvas movement

/** @type {{x: number, y: number} | false} */
let dragging = false;
let cameraX  = 0;
let cameraY  = 0;

cnv.addEventListener("mousedown", e => dragging = {
	x: e.clientX,
	y: e.clientY,
});

cnv.addEventListener("mousemove", e => {
	// TODO: FIX CAMERA MOVEMNT SPEED
	// TODO: ADD CAMERA Y MANIPULATION AND ZOOM

	if (!dragging)
		return;

	cameraX -= (e.clientX - dragging.x) / 2;

	requestAnimationFrame(update);
});

// stop dragging regardless of if on canvas anymore or not
addEventListener("mouseup", e => dragging = false);

const gridWidth  = 8;
const gridHeight = 4;

const lineThickness = 2;

requestAnimationFrame(update);

function update() {
	ctx.clearRect(0, 0, cnv.width, cnv.height);

	// calculate the size of each tile

	const tileSize = (() => {
		const tileWidth  = cnv.width  / gridWidth;
		const tileHeight = cnv.height / gridHeight;

		return Math.min(tileWidth, tileHeight);
	})();

	// calculate the board positions
	// center the grid within the board

	const board  = {};

	board.top    = (cnv.height - tileSize * gridHeight) / 2;
	board.left   = (cnv.width  - tileSize * gridWidth ) / 2;
	board.right  =  cnv.width  - board.left;
	board.bottom =  cnv.height - board.top;

	// offset board positions by camera position

	board.top    -= cameraY;
	board.left   -= cameraX;
	board.right  -= cameraX;
	board.bottom -= cameraY;

	// draw tiles

	ctx.fillStyle = "#101010";

	for (let col = 0; col < gridHeight; col++)
		// col % 2 is used to checker the board by switching the starting position
		for (let row = col % 2; row < gridWidth; row += 2) {
			const x = board.left + tileSize * row;
			const y = board.top  + tileSize * col;

			ctx.rect(x, y, tileSize, tileSize);
		}

	ctx.fill();

	// draw vertical lines

	for (let i = 1; i < gridWidth; i++) {
		const x = board.left + tileSize * i;

		drawLine(
			new Coordinate(x, board.top   ),
			new Coordinate(x, board.bottom),
		);
	}

	// draw horizontal lines

	for (let i = 1; i < gridHeight; i++) {
		const y = board.top + tileSize * i;

		drawLine(
			new Coordinate(board.left,  y),
			new Coordinate(board.right, y),
		);
	}

	// draw borders

	drawLine(
		new Coordinate(board.left,  board.top   ),
		new Coordinate(board.left,  board.bottom),
	);

	drawLine(
		new Coordinate(board.left,  board.top   ),
		new Coordinate(board.right, board.top   ),
	);

	drawLine(
		new Coordinate(board.right, board.top   ),
		new Coordinate(board.right, board.bottom),
	);

	drawLine(
		new Coordinate(board.left,  board.bottom),
		new Coordinate(board.right, board.bottom),
	);

	// draw numbering and lettering

	// const labelMargin = 5/60 * gridboxSize;

	// ctx.font      = `${12/60 * gridboxSize}px Inter, sans-serif`;
	// ctx.fillStyle = "#DDDDDD";

	// // vertical numbering

	// ctx.textAlign = "left";
	// ctx.textBaseline = "top";

	// const numberingX = boardPosition.left + labelMargin;

	// for (let i = 0; i < gridHeight; i++) {
	// 	const label      = gridHeight - i;
	// 	const numberingY = gridboxSize * i + labelMargin;

	// 	ctx.fillText(label, numberingX, numberingY);
	// }

	// // horizontal numbering

	// ctx.textAlign = "right";
	// ctx.textBaseline = "bottom";

	// const letteringY = boardPosition.bottom - labelMargin;

	// for (let i = 0; i < gridWidth; i++) {
	// 	const label = gridWidth <= 26 ? "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[i] : i + 1;
	// 	const letteringX = gridboxSize * (i + 1) - labelMargin;

	// 	ctx.fillText(label, letteringX, letteringY);
	// }

	// TODO: MAKE NUMBERING/LETTERING + LABEL MARGIN SCALE WITH TILE SIZE
}

/**
 * @param {Coordinate} first
 * @param {Coordinate} final
 */
function drawLine(first, final) {
	ctx.beginPath();
	ctx.moveTo(first.x, first.y);
	ctx.lineTo(final.x, final.y);

	ctx.strokeStyle = "#666666";
	ctx.lineWidth   = lineThickness;
	ctx.stroke();
};

cnv.addEventListener("resize", () => {
	cnv.width  = Math.floor(cnv.getBoundingClientRect().width );
	cnv.height = Math.floor(cnv.getBoundingClientRect().height);
});

cnv.dispatchEvent(new Event("resize"));

