import "./modules/ca.chesstacean.components.js";
import "./web-components/registry.js";

const nav        = document.querySelector("body > nav");
const main       = document.querySelector("body > main");

const gameWindow = document.getElementById("game-window");

window.openGameWindow = async () => {
	gameWindow.classList.add("active");

	setTimeout(() => {
		nav.style.display = "none";
		main.style.display = "none";
	}, 200); // transition time of .2s
}

window.closeGameWindow = () => {
	gameWindow.classList.remove("active");

	nav.removeAttribute('style');
	main.removeAttribute('style');
}
