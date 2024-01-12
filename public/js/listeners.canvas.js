"use strict";

canvas.cnv.addEventListener("click", e => {
	// click event
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
