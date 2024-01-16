"use strict";

const nav       = document.querySelector("body > nav");
const main      = document.querySelector("body > main");
const gamePopup = document.getElementById("game-popup");

const transitionTimeMs = 200;

function openGamePopup() {
	closeAllSideWindows();

	gamePopup.style.removeProperty("overflow-x");

	gamePopup.classList.add("active");

	setTimeout(() => {
		nav.style.display  = "none";
		main.style.display = "none";
	}, transitionTimeMs);
}

function closeGamePopup() {
	closeAllSideWindows();

	nav.style.removeProperty("display");
    main.style.removeProperty("display");

	gamePopup.classList.remove("active");

	setTimeout(() => {
		gamePopup.style.overflowX = "hidden";
	}, transitionTimeMs);
}

addEventListener("keydown", e => {
	if (e.code === "Escape")
		closeGamePopup();
});
