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
 * @property {(x: number, y: number, width: number, height: number) => undefined} square
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

canvas.rect = function(x, y, width, height) {
	this.ctx.fillStyle = "#101010";

	this.ctx.rect(x, y, width, height);
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

canvas.square = function(x, y, width, height) {
	this.ctx.strokeStyle = "#FF582F";
	this.ctx.lineWidth = 5 * board.tilesize / 90;

	this.ctx.beginPath();
	this.ctx.rect(x, y, width, height);
	this.ctx.stroke();
}

canvas.circle = function(x, y, width, height) {
	this.ctx.strokeStyle = "#FF582F";
	this.ctx.lineWidth   = 5 * board.tilesize / 90;

	this.ctx.beginPath();
	this.ctx.ellipse(x, y, width / 2, height / 2, 0, 0, Math.PI * 2);
	this.ctx.stroke();
}
