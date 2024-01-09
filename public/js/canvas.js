"use strict";

import Board from "./board.js";
import Camera from "./camera.js";
import Point from "./point.js";

class Canvas {
	#canvas;
	#context;

	get width() {
		return this.#canvas.width;
	}

	get height() {
		return this.#canvas.height;
	}

	/**
	 * @param {Point}  first
	 * @param {Point}  final
	 * @param {number} width
	 */
	line(first, final, width) {
		this.#context.beginPath();
		this.#context.moveTo(first.x, first.y);
		this.#context.lineTo(final.x, final.y);

		this.#context.strokeStyle = "#666666";
		this.#context.lineWidth   = width;

		this.#context.stroke();
	}

	/**
	 * @param {Point}  point
	 * @param {number} width
	 * @param {number} height
	 */
	rect(point, width, height) {
		console.log(1, point.x, point.y, width, height);

		this.#context.fillStyle = "#101010";

		this.#context.rect(point.x, point.y, width, height);
		this.#context.fill();
	}

	/**
	 * @param {string} text
	 * @param {number} x
	 * @param {number} y
	 * @param {string} font
	 * @param {string} align
	 * @param {string} baseline
	 */
	text(text, x, y, font, align, baseline) {
		this.#context.font      = font;
		this.#context.fillStyle = "#DDDDDD";

		this.#context.textAlign    = align;
		this.#context.textBaseline = baseline;

		this.#context.fillText(text, x, y);
	}

	clear() {
		this.#context.clearRect(0, 0, this.#canvas.width, this.#canvas.height);
	}

	/**
	 * @param {CanvasImageSource} image
	 * @param {number} x
	 * @param {number} y
	 * @param {number} width
	 * @param {number} height
	 */
	image(image, x, y, width, height) {
		this.#context.drawImage(image, x, y, width, height);
	}

	/**
	 * @param {HTMLCanvasElement} canvas
	 * @param {Board}  board
	 * @param {Camera} camera
	 */
	constructor(canvas, board, camera) {
		this.#canvas  = canvas;
		this.#context = canvas.getContext("2d");

		// resize

		window.addEventListener("resize", () => {
			canvas.width  = Math.floor(canvas.getBoundingClientRect().width );
			canvas.height = Math.floor(canvas.getBoundingClientRect().height);
		});

		window.dispatchEvent(new Event("resize"));

		// zooming

		canvas.addEventListener("wheel", e => {
			e.preventDefault();

			// make zooming in faster the more zoomed out you are
			// and slower the more zoomed in you are
			const factor = (Math.max(board.gridwidth, board.gridheight) - camera.zoom) / Math.min(board.gridwidth, board.gridheight);

			camera.zoom -= Math.sign(e.deltaY) * factor;
			if (camera.zoom < 0) camera.zoom = 0;

			// - 2 ensures a minimum number of tiles
			const max = Math.max(board.gridwidth, board.gridheight) - 2;

			if (camera.zoom > max)
				camera.zoom = max;
		});

		// dragging

		/** @type {{x: number, y: number} | false} */
		let dragging = false;

		canvas.addEventListener("mousedown", e => {
			const rect = canvas.getBoundingClientRect();

			dragging = {
				x: e.clientX - rect.left + camera.x,
				y: e.clientY - rect.top  + camera.y,
			};
		});

		canvas.addEventListener("mousemove", e => {
			if (!dragging)
				return;

			const rect = canvas.getBoundingClientRect();

			canvas.setAttribute("width",  rect.width );
			canvas.setAttribute("height", rect.height);

			camera.x = dragging.x - (e.clientX - rect.left);
			camera.y = dragging.y - (e.clientY - rect.top );
		});

		// stop dragging regardless of if on canvas anymore or not
		addEventListener("mouseup", () => dragging = false);
	}
}

export default Canvas;
