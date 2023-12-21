"use strict";

const nav       = document.querySelector("body > nav");
const main      = document.querySelector("body > main");
const gamePopup = document.getElementById("game-popup");

function openGamePopup() {
	gamePopup.classList.add("active");

	setTimeout(() => {
		nav.style.display  = "none";
		main.style.display = "none";
	}, 200); // transition time of .2s
}

function closeGamePopup() {
	gamePopup.classList.remove("active");

    nav.style.removeProperty("display");
    main.style.removeProperty("display");
}

addEventListener("keydown", e => {
	if (e.code === "Escape")
		closeGamePopup();
});
