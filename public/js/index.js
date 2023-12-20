
import "./modules/ca.chesstacean.components.js";
import "./web-components/registry.js";

const nav        = document.querySelector("body > nav");
const main       = document.querySelector("body > main");

const gameWindow = document.getElementById("game-window");

window.openGameWindow = async () => {
	gameWindow.classList.add("active");

	setTimeout(() => {
		nav.style.display = "none";
		main.style.display = "none";
	}, 200); // transition time of .2s
}

window.closeGameWindow = () => {
	gameWindow.classList.remove("active");

	nav.removeAttribute('style');
	main.removeAttribute('style');
}

/** @type {HTMLCanvasElement} */
const cnv = document.getElementById("game-board");
const ctx = cnv.getContext('2d');

const gridWidth  = 8;
const gridHeight = 4;

const lineWidth = 2;

requestAnimationFrame(update);

class Coordinate {
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

function update() {
	ctx.clearRect(0, 0, cnv.width, cnv.height);

	// draw tiles

	const gridboxWidth  = cnv.width  / gridWidth;
	const gridboxHeight = cnv.height / gridHeight;

	const gridboxSize   = Math.min(gridboxWidth, gridboxHeight);

	const boardPosition = {
		top:    (cnv.height - gridboxSize * gridHeight) / 2,
		left:   (cnv.width  - gridboxSize * gridWidth ) / 2,
		right:  cnv.width  - (cnv.width  - gridboxSize * gridWidth) / 2,
		bottom: cnv.height - (cnv.height - gridboxSize * gridHeight) / 2,
	};

	// draw tiles

	ctx.fillStyle = "#101010";

	for (let col = 0; col < gridHeight; col++)
		// col % 2 is used to checker the board by switching the starting position
		for (let row = col % 2; row < gridWidth; row += 2) {
			const x = boardPosition.left + gridboxSize * row;
			const y = boardPosition.top  + gridboxSize * col;

			ctx.rect(x, y, gridboxSize, gridboxSize);
		}

	ctx.fill();

	// draw vertical lines

	for (let i = 1; i < gridWidth; i++) {
		const x = boardPosition.left + gridboxSize * i;

		drawLine(
			new Coordinate(x, boardPosition.top   ),
			new Coordinate(x, boardPosition.bottom),
		);
	}

	// draw horizontal lines

	for (let i = 1; i < gridHeight; i++) {
		const y = boardPosition.top + gridboxSize * i;

		drawLine(
			new Coordinate(boardPosition.left,  y),
			new Coordinate(boardPosition.right, y),
		);
	}

	// draw borders

	drawLine(
		new Coordinate(boardPosition.left,  boardPosition.top   ),
		new Coordinate(boardPosition.left,  boardPosition.bottom),
	);

	drawLine(
		new Coordinate(boardPosition.left,  boardPosition.top   ),
		new Coordinate(boardPosition.right, boardPosition.top   ),
	);

	drawLine(
		new Coordinate(boardPosition.right, boardPosition.top   ),
		new Coordinate(boardPosition.right, boardPosition.bottom),
	);

	drawLine(
		new Coordinate(boardPosition.left,  boardPosition.bottom),
		new Coordinate(boardPosition.right, boardPosition.bottom),
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

	// TODO: ADD ZOOM TO CHESSBOARD
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
	ctx.lineWidth   = lineWidth;
	ctx.stroke();
};

cnv.addEventListener("resize", () => {
	cnv.width  = Math.floor(cnv.getBoundingClientRect().width );
	cnv.height = Math.floor(cnv.getBoundingClientRect().height);
});

cnv.dispatchEvent(new Event("resize"));

