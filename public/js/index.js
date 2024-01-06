"use strict";

import pieces from "./pieces.js";
import { Coordinate } from "./components.js";

// canvas setup

/** @type {HTMLCanvasElement} */
const cnv = document.getElementById("game-board");
const ctx = cnv.getContext("2d");

let firstFrame = true;

// board tracking

const fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

const gridWidth = 26;
const gridHeight = 8;

const lineThickness = 2;

// canvas movement

/** @type {{x: number, y: number} | false} */
let dragging = false;

let zoom = 0;

let cameraX = 0;
let cameraY = 0;

let maxOffsetX = 0;
let maxOffsetY = 0;

// zooming

cnv.addEventListener("wheel", (e) => {
  e.preventDefault();

  // make zooming in faster the more zoomed out you are
  // and slower the more zoomed in you are
  const factor = (Math.max(gridWidth, gridHeight) - zoom) / 8;

  zoom -= Math.sign(e.deltaY) * factor;
  if (zoom < 0) zoom = 0;

  // -2 ensures a minimum number of tiles
  const max = Math.max(gridWidth, gridHeight) - 2;

  if (zoom > max) zoom = max;

  requestAnimationFrame(update);
});

// dragging

cnv.addEventListener("mousedown", (e) => {
  const rect = cnv.getBoundingClientRect();

  dragging = {
    x: e.clientX - rect.left + cameraX,
    y: e.clientY - rect.top + cameraY,
  };
});

cnv.addEventListener("mousemove", (e) => {
  if (!dragging) return;

  const rect = cnv.getBoundingClientRect();

  cnv.setAttribute("width", rect.width);
  cnv.setAttribute("height", rect.height);

  cameraX = dragging.x - (e.clientX - rect.left);
  cameraY = dragging.y - (e.clientY - rect.top);

  // update frame

  requestAnimationFrame(update);
});

// stop dragging regardless of if on canvas anymore or not
addEventListener("mouseup", () => (dragging = false));

function update() {
  ctx.clearRect(0, 0, cnv.width, cnv.height);

  const scaledGridWidth = (() => {
    const scaledGridWidth = gridWidth - zoom;
    return scaledGridWidth < 0 ? 0 : scaledGridWidth;
  })();

  const scaledGridHeight = (() => {
    const scaledGridHeight = gridHeight - zoom;
    return scaledGridHeight < 0 ? 0 : scaledGridHeight;
  })();

  // calculate the size of each tile

  const tileSize = (() => {
    const tileWidth = cnv.width / scaledGridWidth;
    const tileHeight = cnv.height / scaledGridHeight;

    return Math.min(tileWidth, tileHeight);
  })();

  // calculate the board positions
  // center the grid within the board

  const board = {};

  board.top = (cnv.height - tileSize * gridHeight) / 2;
  board.left = (cnv.width - tileSize * gridWidth) / 2;
  board.right = cnv.width - board.left;
  board.bottom = cnv.height - board.top;

  maxOffsetX = Math.abs(board.left);
  maxOffsetY = Math.abs(board.top);

  // prevent dragging outside of border

  let prevCameraX = cameraX;
  let prevCameraY = cameraY;

  // cap the camera position at the newly calculated max offset
  if (Math.abs(cameraX) > maxOffsetX) cameraX = Math.sign(cameraX) * maxOffsetX;
  if (Math.abs(cameraY) > maxOffsetY) cameraY = Math.sign(cameraY) * maxOffsetY;

  // offset board positions by camera position

  board.top -= cameraY;
  board.left -= cameraX;
  board.right -= cameraX;
  board.bottom -= cameraY;

  // draw tiles

  ctx.fillStyle = "#101010";

  for (let col = 0; col < gridHeight; col++)
    // col % 2 is used to checker the board by switching the starting position
    for (let row = col % 2; row < gridWidth; row += 2) {
      const x = board.left + tileSize * row;
      const y = board.top + tileSize * col;

      ctx.rect(x, y, tileSize, tileSize);
    }

  ctx.fill();

  // draw vertical lines

  for (let i = 1; i < gridWidth; i++) {
    const x = board.left + tileSize * i;

    drawLine(new Coordinate(x, board.top), new Coordinate(x, board.bottom));
  }

  // draw horizontal lines

  for (let i = 1; i < gridHeight; i++) {
    const y = board.top + tileSize * i;

    drawLine(new Coordinate(board.left, y), new Coordinate(board.right, y));
  }

  // draw borders

  drawLine(
    new Coordinate(board.left, board.top),
    new Coordinate(board.left, board.bottom)
  );

  drawLine(new Coordinate(board.left, board.top), new Coordinate(board.right, board.top));

  drawLine(
    new Coordinate(board.right, board.top),
    new Coordinate(board.right, board.bottom)
  );

  drawLine(
    new Coordinate(board.left, board.bottom),
    new Coordinate(board.right, board.bottom)
  );

  // draw pieces

  // the pieces don't draw without this timeout until a second frame is called

  (() => {
    let row = 0;
    let col = 0;

    Array.from(fen).forEach((character) => {
      if (character === "/") {
        row++;
        col = 0;

        return;
      }

      if (Number.isInteger(+character)) {
        col += +character;
        return;
      }

      const x = board.left + col * tileSize;
      const y = board.top + row * tileSize;

      ctx.drawImage(pieces[character], x, y, tileSize, tileSize);

      col++;
    });
  })();

  // draw numbering and lettering

  const labelMargin = (5 / 60) * tileSize;

  ctx.font = `${(12 / 60) * tileSize}px Inter, sans-serif`;
  ctx.fillStyle = "#DDDDDD";

  // vertical numbering

  ctx.textAlign = "left";
  ctx.textBaseline = "top";

  const numberingX = board.left + labelMargin;

  for (let i = 0; i < gridHeight; i++) {
    const label = gridHeight - i;
    const numberingY = board.top + tileSize * i + labelMargin;

    ctx.fillText(label, numberingX, numberingY);
  }

  // horizontal numbering

  ctx.textAlign = "right";
  ctx.textBaseline = "bottom";

  const letteringY = board.bottom - labelMargin;

  for (let i = 0; i < gridWidth; i++) {
    const label = gridWidth <= 26 ? "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[i] : i + 1;
    const letteringX = board.left + tileSize * (i + 1) - labelMargin;

    ctx.fillText(label, letteringX, letteringY);
  }

  // the piece images dont render on the first frame for some reason,
  // so re-render the frame if this was the first

  if (firstFrame) {
    firstFrame = false;
    requestAnimationFrame(update);
  }
}

/**
 * @param {Coordinate} first
 * @param {Coordinate} final
 */
function drawLine(first, final) {
  ctx.beginPath();
  ctx.moveTo(first.x, first.y);
  ctx.lineTo(final.x, final.y);

  ctx.strokeStyle = "#666666";
  ctx.lineWidth = lineThickness;
  ctx.stroke();
}

cnv.addEventListener("resize", () => {
  cnv.width = Math.floor(cnv.getBoundingClientRect().width);
  cnv.height = Math.floor(cnv.getBoundingClientRect().height);

  requestAnimationFrame(update);
});

cnv.dispatchEvent(new Event("resize"));

requestAnimationFrame(update);
