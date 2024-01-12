"use strict";

/**
 * @typedef  {object} Canvas
 * @property {HTMLCanvasElement} cnv
 * @property {CanvasRenderingContext2D} ctx
 * @property {{{x: number, y: number} | false}} dragging
 * @property {(first: Point, final: Point, width: number) => undefined} line
 * @property {(point: Point, width: number, height: number) => undefined} rect
 * @property {(text: string, x: number, y: number, font: string, align: string, baseline: string) => undefined} text
 * @property {() => undefined} clear
 * @property {(image: CanvasImageSource, x: number, y: number, width: number, height: number) => undefined} image
 */

/** @type {Canvas} */
const canvas = {
	get width()  { return this.cnv.width  },
	get height() { return this.cnv.height },
};

canvas.dragging = false;

canvas.cnv = document.getElementById("game-board");
canvas.ctx = canvas.cnv.getContext("2d");

// define utility methods

canvas.line = function(first, final, width) {
	this.ctx.beginPath();
	this.ctx.moveTo(first.x, first.y);
	this.ctx.lineTo(final.x, final.y);

	this.ctx.strokeStyle = "#666666";
	this.ctx.lineWidth   = width;

	this.ctx.stroke();
};

canvas.rect = function(point, width, height) {
	this.ctx.fillStyle = "#101010";

	this.ctx.rect(point.x, point.y, width, height);
	this.ctx.fill();
};

canvas.text = function(text, x, y) {
	this.ctx.fillText(text, x, y);
};

canvas.clear = function() {
	this.ctx.clearRect(0, 0, this.cnv.width, this.cnv.height);
};

canvas.image = function(image, x, y, width, height) {
	this.ctx.drawImage(image, x, y, width, height);
};

// define canvas listeners

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
	if (!canvas.dragging)
		return;

	const rect = canvas.cnv.getBoundingClientRect();

	canvas.cnv.setAttribute("width",  rect.width );
	canvas.cnv.setAttribute("height", rect.height);

	camera.x = canvas.dragging.x - (e.clientX - rect.left);
	camera.y = canvas.dragging.y - (e.clientY - rect.top );

	requestAnimationFrame(update);
});

addEventListener("mouseup", () => canvas.dragging = false);

// define window listeners

window.addEventListener("resize", () => {
	canvas.cnv.width  = Math.floor(canvas.cnv.getBoundingClientRect().width );
	canvas.cnv.height = Math.floor(canvas.cnv.getBoundingClientRect().height);

	requestAnimationFrame(update);
});

window.dispatchEvent(new Event("resize"));
