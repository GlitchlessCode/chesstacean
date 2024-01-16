"use strict";

function howToOpenGameError() {
	const toast = new Toast.Warning();

	toast.message = "WIP. Click play for mock game.";
	toast.send();
}

function featureUnavailableError() {
	const toast = new Toast.Error();

	toast.message = "This feature is currently unavailable.";
	toast.send();
}

// *TEMP, until server connection
function noPossibleMovesError() {
	const toast = new Toast.Information();

	toast.message = "Sample bot unable to make a move.";
	toast.send();
}
