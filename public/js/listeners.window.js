"use strict";

window.addEventListener("resize", () => {
	canvas.cnv.width  = Math.floor(canvas.cnv.getBoundingClientRect().width );
	canvas.cnv.height = Math.floor(canvas.cnv.getBoundingClientRect().height);

	requestAnimationFrame(update);
});

window.dispatchEvent(new Event("resize"));

window.addEventListener("keydown", e => {
	if (e.ctrlKey && e.code === "Space")
		board.reversePov = !board.reversePov;
});
