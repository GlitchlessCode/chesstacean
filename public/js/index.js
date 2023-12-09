import "./modules/ca.chesstacean.components.js";

const gameWindow = document.getElementById("game-window");

window.openGameWindow = () => {
	gameWindow.classList.add("active");
}

window.closeGameWindow = () => {
	gameWindow.classList.remove("active");
}

