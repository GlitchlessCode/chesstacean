"use strict";

function nudge() {
	const toast = new Toast.Information();

	toast.title   = "Nudge";
	toast.message = "[displayname2] says it's your turn.";
	toast.send();
}

// *TEMP for testing and display purposes
addEventListener("keydown", e => {
	if (e.shiftKey && e.code === "KeyN")
		nudge();
});
