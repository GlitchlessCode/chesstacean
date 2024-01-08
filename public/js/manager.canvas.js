"use strict";

class Point {
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

class Canvas {
	#canvas;
	context;

	/**
	 * @param {Point}  first
	 * @param {Point}  final
	 * @param {number} width
	 */
	line(first, final, width) {
		// make path

		context.beginPath();
		context.moveTo(first.x, first.y);
		context.lineTo(final.x, final.y);

		// save context

		const strokeStyle = context.strokeStyle;
		const lineWidth   = context.lineWidth;

		// draw line

		context.strokeStyle = "#666666";
		context.lineWidth   = width;

		context.stroke();

		// load context

		context.strokeStyle = strokeStyle;
		context.lineWidth   = lineWidth;
	}

	/** @param {HTMLCanvasElement} canvas*/
	constructor(canvas) {
		this.#canvas = canvas;
		this.context = canvas.getContext("2D");
	}
}

const canvas = new Canvas(document.getElementById("game-board"));
