"use strict";

// board layout in fen notation
const fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

let maxOffsetX = 0;
let maxOffsetY = 0;
let firstRender = true;

function update() {
	canvas.clear();

	// recalculate tile size

	board.tilesize = (() => {
		// adjust grid sizes based on zoom

		const zoomedGridwidth  = Math.max(board.gridwidth  - camera.z, 0);
		const zoomedGridheight = Math.max(board.gridheight - camera.z, 0);

		// determine tile sizes based on grid width

		const tilewidth  = canvas.width  / zoomedGridwidth;
		const tileheight = canvas.height / zoomedGridheight;

		return Math.min(tilewidth, tileheight);
	})();

	// calculate the board positions
	// center the grid within the board

	board.top    = (canvas.height - board.tilesize * board.gridheight) / 2;
	board.left   = (canvas.width  - board.tilesize * board.gridwidth)  / 2;
	board.right  =  canvas.width  - board.left;
	board.bottom =  canvas.height - board.top;

	maxOffsetX = Math.abs(board.left);
	maxOffsetY = Math.abs(board.top);

	// prevent dragging outside of border

	// cap the camera position at the newly calculated max offset
	if (Math.abs(camera.x) > maxOffsetX)
		camera.x = Math.sign(camera.x) * maxOffsetX;
	if (Math.abs(camera.y) > maxOffsetY)
		camera.y = Math.sign(camera.y) * maxOffsetY;

	// offset board positions by camera position

	board.top    -= camera.y;
	board.left   -= camera.x;
	board.right  -= camera.x;
	board.bottom -= camera.y;

	// draw tiles

	for (let col = 0; col < board.gridheight; col++)
		// col % 2 is used to checker the board by switching the starting position
		for (let row = col % 2; row < board.gridwidth; row += 2) {
			const x = board.left + board.tilesize * row;
			const y = board.top  + board.tilesize * col;

			canvas.rect(new Point(x, y), board.tilesize, board.tilesize);
		}

	const lineWidth = 2 * board.tilesize / 90;

	// draw vertical lines

	for (let i = 1; i < board.gridwidth; i++) {
		const x = board.left + board.tilesize * i;

		canvas.line(
			new Point(x, board.top),
			new Point(x, board.bottom),
			lineWidth,
		);
	}

	// draw horizontal lines

	for (let i = 1; i < board.gridheight; i++) {
		const y = board.top + board.tilesize * i;

		canvas.line(
			new Point(board.left,  y),
			new Point(board.right, y),
			lineWidth,
		);
	}

	// draw borders

	canvas.line(
		new Point(board.left,  board.top   ),
		new Point(board.left,  board.bottom),
		lineWidth,
	);

	canvas.line(
		new Point(board.left,  board.top   ),
		new Point(board.right, board.top   ),
		lineWidth,
	);

	canvas.line(
		new Point(board.right, board.top   ),
		new Point(board.right, board.bottom),
		lineWidth,
	);

	canvas.line(
		new Point(board.left,  board.bottom),
		new Point(board.right, board.bottom),
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

			canvas.image(pieces[character], x, y, board.tilesize, board.tilesize);

			col++;
		});
	})();

	// draw numbering and lettering

	const font = `${12/60 * board.tilesize}px Inter, sans-serif`;
	const labelMargin = 5/60 * board.tilesize;

	// vertical numbering

	canvas.ctx.font      = font;
	canvas.ctx.fillStyle = "#DDDDDD";

	canvas.ctx.textAlign    = "left";
	canvas.ctx.textBaseline = "top";

	const numberingX = board.left + labelMargin;

	for (let i = 0; i < board.gridheight; i++) {
		const label      = board.gridheight - i;
		const numberingY = board.top + board.tilesize * i + labelMargin;

		canvas.text(label, numberingX, numberingY);
	}

	// horizontal numbering

	canvas.ctx.textAlign    = "right";
	canvas.ctx.textBaseline = "bottom";

	const letteringY = board.bottom - labelMargin;

	for (let i = 0; i < board.gridwidth; i++) {
		const label      = board.gridwidth <= 26 ? "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[i] : i + 1;
		const letteringX = board.left + board.tilesize * (i + 1) - labelMargin;

		canvas.text(label, letteringX, letteringY);
	}

	// in case images didn't load properly
	if (firstRender) {
		firstRender = false;
		requestAnimationFrame(update);
	}
}

requestAnimationFrame(update);
