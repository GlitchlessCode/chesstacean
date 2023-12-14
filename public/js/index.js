
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

requestAnimationFrame(update);

const borderWidth = 2;
const gridWidth = 8;
const gridHeight = 8;

function update() {
	ctx.clearRect(0, 0, cnv.width, cnv.height);

	// draw squares

	const gridboxWidth = cnv.width / gridWidth;
	const gridboxHeight = cnv.height / gridHeight;

	for (let i = 0; i < gridHeight; i++) {
		for (let j = i % 2; j < gridWidth; j += 2) {
			const x = gridboxWidth * j;
			const y = gridboxHeight * i;

			ctx.rect(x, y, gridboxWidth, gridboxHeight);
			ctx.fillStyle = "#101010";
			ctx.fill();
		}
	}

	// draw lines

	for (let i = 0; i < gridWidth - 1; i++) {
		const x = gridboxWidth * (i + 1);

		drawLine(x, 0, x, cnv.height);
	}

	for (let i = 0; i < gridHeight - 1; i++) {
		const y= gridboxHeight * (i + 1);

		drawLine(0, y, cnv.width, y);
	}

	// draw borders

	drawLine(borderWidth / 2, 0, 0, cnv.height);
	drawLine(0, borderWidth / 2, cnv.width , 0);
	drawLine(cnv.width - borderWidth / 2, 0, cnv.width, cnv.height);
	drawLine(0, cnv.height - borderWidth / 2, cnv.width, cnv.height);

	requestAnimationFrame(update);
}

function drawLine(x1, y1, x2, y2) {
	ctx.beginPath();
	ctx.moveTo(x1, y1);
	ctx.lineTo(x2, y2);

	ctx.strokeStyle = "#666666";
	ctx.lineWidth   = 2;
	ctx.stroke();
};

cnv.addEventListener("resize", () => {
	cnv.width  = Math.floor(cnv.getBoundingClientRect().width  * window.devicePixelRatio);
	cnv.height = Math.floor(cnv.getBoundingClientRect().height * window.devicePixelRatio);
});

cnv.dispatchEvent(new Event("resize"));

