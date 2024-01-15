"use strict";

function howToOpenGameError() {
	const toast = new Toast.Warning();

	toast.message = "WIP. Open the mock game using the card's play button.";
	toast.send();
}

function featureUnavailableError() {
	const toast = new Toast.Error();

	toast.message = "This feature is currently unavailable.";
	toast.send();
}
