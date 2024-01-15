"use strict";

function featureUnavailableError() {
	const toast = new Toast.Error();

	toast.message = "This feature is currently unavailable.";
	toast.send();
}
