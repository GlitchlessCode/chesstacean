/** @type {HTMLCanvasElement} */
const cnv = document.getElementById("chessboard");
const ctx = cnv.getContext("2d");

const size = 8;

// draw vertical board lines

const lineWidth = 2;
const spacing = cnv.width - (lineWidth + lineWidth * size) / size;

for (let i = 0; i < size; i++) {
}

// draw horizontal board lines
