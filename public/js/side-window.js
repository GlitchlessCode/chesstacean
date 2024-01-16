"use strict";

function toggleSideWindow(classname) {
	const el       = document.querySelector(`.side-window.${classname}`);
	const isActive = el.classList.contains("active");

	closeAllSideWindows();

	isActive ? el.classList.remove("active") : el.classList.add("active");
}

function closeAllSideWindows() {
	document.querySelectorAll(".side-window").forEach(el => {
		el.classList.remove("active");
	});
}
