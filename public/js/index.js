"use strict";

import pieces from "./pieces.js";
import { Coordinate } from "./components.js";

// board tracking

const fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

const gridWidth  = 8;
const gridHeight = 8;

// canvas movement

/** @type {{x: number, y: number} | false} */
let dragging = false;

let zoom = 0;

let cameraX = 0;
let cameraY = 0;

let maxOffsetX = 0;
let maxOffsetY = 0;

// zooming

canvas.addEventListener("wheel", e => {
	e.preventDefault();

	// make zooming in faster the more zoomed out you are
	// and slower the more zoomed in you are
	const factor = (Math.max(gridWidth, gridHeight) - zoom) / Math.min(gridWidth, gridHeight);

	zoom -= Math.sign(e.deltaY) * factor;
	if (zoom < 0) zoom = 0;

	// - 2 ensures a minimum number of tiles
	const max = Math.max(gridWidth, gridHeight) - 2;

	if (zoom > max)
		zoom = max;

	requestAnimationFrame(update);
});

// dragging

canvas.addEventListener("mousedown", e => {
	const rect = canvas.getBoundingClientRect();

	dragging = {
		x: e.clientX - rect.left + cameraX,
		y: e.clientY - rect.top  + cameraY,
	};
});

canvas.addEventListener("mousemove", e => {
	if (!dragging)
		return;

	const rect = canvas.getBoundingClientRect();

	canvas.setAttribute("width",  rect.width );
	canvas.setAttribute("height", rect.height);

	cameraX = dragging.x - (e.clientX - rect.left);
	cameraY = dragging.y - (e.clientY - rect.top );

	// update frame

	requestAnimationFrame(update);
});

// stop dragging regardless of if on canvas anymore or not
addEventListener("mouseup", () => dragging = false);

// board definition

/**
 * @type     {object}
 * @property {number} tilesize
 * @property {() => undefined} recalculateTilesize
 */
const board = {};

board.recalculateTilesize = function() {
	// adjust grid sizes based on zoom

	const scaledGridWidth  = Math.max(gridWidth  - zoom, 0);
	const scaledGridHeight = Math.max(gridHeight - zoom, 0);

	// determine tile sizes based on grid width

	const tileWidth  = canvas.width  / scaledGridWidth;
	const tileHeight = canvas.height / scaledGridHeight;

	board.tilesize = Math.min(tileWidth, tileHeight);
};

// move pieces

canvas.addEventListener("click", e => {
});

function update() {
	context.clearRect(0, 0, canvas.width, canvas.height);

	board.recalculateTilesize();

	// calculate the board positions
	// center the grid within the board

	const board  = {};

	board.top    = (canvas.height - board.tilesize * gridHeight) / 2;
	board.left   = (canvas.width  - board.tilesize * gridWidth ) / 2;
	board.right  =  canvas.width  - board.left;
	board.bottom =  canvas.height - board.top;

	maxOffsetX = Math.abs(board.left);
	maxOffsetY = Math.abs(board.top);

	// prevent dragging outside of border

	let prevCameraX = cameraX;
	let prevCameraY = cameraY;

	// cap the camera position at the newly calculated max offset
	if (Math.abs(cameraX) > maxOffsetX)
		cameraX = Math.sign(cameraX) * maxOffsetX;
	if (Math.abs(cameraY) > maxOffsetY)
		cameraY = Math.sign(cameraY) * maxOffsetY;

	// offset board positions by camera position

	board.top    -= cameraY;
	board.left   -= cameraX;
	board.right  -= cameraX;
	board.bottom -= cameraY;

	// draw tiles

	context.fillStyle = "#101010";

	for (let col = 0; col < gridHeight; col++)
		// col % 2 is used to checker the board by switching the starting position
		for (let row = col % 2; row < gridWidth; row += 2) {
			const x = board.left + board.tilesize * row;
			const y = board.top  + board.tilesize * col;

			context.rect(x, y, board.tilesize, board.tilesize);
		}

	context.fill();

	const lineWidth = 2 * board.tilesize / 90;

	// draw vertical lines

	for (let i = 1; i < gridWidth; i++) {
		const x = board.left + board.tilesize * i;

		canvas.line(
			new Coordinate(x, board.top   ),
			new Coordinate(x, board.bottom),
			lineWidth,
		);
	}

	// draw horizontal lines

	for (let i = 1; i < gridHeight; i++) {
		const y = board.top + board.tilesize * i;

		canvas.line(
			new Coordinate(board.left,  y),
			new Coordinate(board.right, y),
			lineWidth,
		);
	}

	// draw borders

	canvas.line(
		new Coordinate(board.left,  board.top   ),
		new Coordinate(board.left,  board.bottom),
		lineWidth,
	);

	canvas.line(
		new Coordinate(board.left,  board.top   ),
		new Coordinate(board.right, board.top   ),
		lineWidth,
	);

	canvas.line(
		new Coordinate(board.right, board.top   ),
		new Coordinate(board.right, board.bottom),
		lineWidth,
	);

	canvas.line(
		new Coordinate(board.left,  board.bottom),
		new Coordinate(board.right, board.bottom),
		lineWidth,
	);

	// draw pieces

	// some pieces dont draw without this anonymous function usage for some reason...

	(() => {
		let row = 0;
		let col = 0;

		Array.from(fen).forEach(character => {
			if (character === '/') {
				row++;
				col = 0;

				return;
			}

			if (Number.isInteger(+character)) {
				col += +character;
				return;
			}

			const x = board.left + col * board.tilesize;
			const y = board.top  + row * board.tilesize;

			context.drawImage(pieces[character], x, y, board.tilesize, board.tilesize);

			col++;
		});
	})();

	// draw numbering and lettering

	const labelMargin = 5/60 * board.tilesize;

	context.font      = `${12/60 * board.tilesize}px Inter, sans-serif`;
	context.fillStyle = "#DDDDDD";

	// vertical numbering

	context.textAlign    = "left";
	context.textBaseline = "top";

	const numberingX = board.left + labelMargin;

	for (let i = 0; i < gridHeight; i++) {
		const label      = gridHeight - i;
		const numberingY = board.top + board.tilesize * i + labelMargin;

		context.fillText(label, numberingX, numberingY);
	}

	// horizontal numbering

	context.textAlign    = "right";
	context.textBaseline = "bottom";

	const letteringY = board.bottom - labelMargin;

	for (let i = 0; i < gridWidth; i++) {
		const label      = gridWidth <= 26 ? "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[i] : i + 1;
		const letteringX = board.left + board.tilesize * (i + 1) - labelMargin;

		context.fillText(label, letteringX, letteringY);
	}

	setTimeout(() => requestAnimationFrame(update), 1000);
}

canvas.addEventListener("resize", () => {
	canvas.width  = Math.floor(canvas.getBoundingClientRect().width );
	canvas.height = Math.floor(canvas.getBoundingClientRect().height);

	requestAnimationFrame(update);
});

canvas.dispatchEvent(new Event("resize"));

requestAnimationFrame(update);
