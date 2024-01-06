import "./modules/ca.chesstacean.components.js";
import "./web-components/registry.js";

import { ConnectionManager } from "./modules/ca.chesstacean.network.js";
window.ConnectionManager = ConnectionManager;

const nav = document.querySelector("body > nav");
const main = document.querySelector("body > main");

const gameWindow = document.getElementById("game-window");

window.openGameWindow = async () => {
  gameWindow.classList.add("active");

  setTimeout(() => {
    nav.style.display = "none";
    main.style.display = "none";
  }, 200); // transition time of .2s
};

window.closeGameWindow = () => {
  gameWindow.classList.remove("active");

  nav.removeAttribute("style");
  main.removeAttribute("style");
};

/** @type {HTMLCanvasElement} */
const cnv = document.getElementById("game-board");
const ctx = cnv.getContext("2d");

const gridWidth = 8;
const gridHeight = 8;

const lineWidth = 2;

requestAnimationFrame(update);

function update() {
  ctx.clearRect(0, 0, cnv.width, cnv.height);

  // draw tiles

  const gridboxWidth = cnv.width / gridWidth;
  const gridboxHeight = cnv.height / gridHeight;

  const gridboxSize = Math.min(gridboxWidth, gridboxHeight);

  for (let i = 0; i < gridHeight; i++) {
    for (let j = i % 2; j < gridWidth; j += 2) {
      const x = gridboxSize * j;
      const y = gridboxSize * i;

      ctx.rect(x, y, gridboxSize, gridboxSize);
      ctx.fillStyle = "#101010";
      ctx.fill();
    }
  }

  const boardPosition = {
    top: 0,
    left: 0,
    right: gridboxSize * gridWidth,
    bottom: gridboxSize * gridHeight,
  };

  // draw lines

  // vertical

  for (let i = 0; i < gridWidth - 1; i++) {
    const x = gridboxSize * (i + 1);

    drawLine(x, 0, x, gridboxSize * gridHeight);
  }

  // horizontal

  for (let i = 0; i < gridHeight - 1; i++) {
    const y = gridboxSize * (i + 1);

    drawLine(0, y, gridboxSize * gridWidth, y);
  }

  // draw borders

  drawLine(
    boardPosition.left,
    boardPosition.top,
    boardPosition.left,
    boardPosition.bottom
  );
  drawLine(boardPosition.left, boardPosition.top, boardPosition.right, boardPosition.top);
  drawLine(
    boardPosition.right,
    boardPosition.top,
    boardPosition.right,
    boardPosition.bottom
  );
  drawLine(
    boardPosition.left,
    boardPosition.bottom,
    boardPosition.right,
    boardPosition.bottom
  );

  // draw numbering and lettering

  const labelMargin = 8;

  ctx.font = "12px Inter";
  ctx.fillStyle = "#DDDDDD";

  // vertical numbering

  ctx.textAlign = "left";
  ctx.textBaseline = "top";

  const numberingX = boardPosition.left + labelMargin - lineWidth;

  for (let i = 0; i < gridHeight; i++) {
    const label = gridHeight - i;
    const numberingY = gridboxSize * i + labelMargin - lineWidth;

    ctx.fillText(label, numberingX, numberingY);
  }

  // horizontal numbering

  ctx.textAlign = "right";
  ctx.textBaseline = "bottom";

  const letteringY = boardPosition.bottom - labelMargin + lineWidth;

  for (let i = 0; i < gridWidth; i++) {
    const label = i + 1;
    const letteringX = gridboxSize * (i + 1) - labelMargin + lineWidth;

    ctx.fillText(label, letteringX, letteringY);
  }

  // TODO: ADD ZOOM TO CHESSBOARD
  // TODO: ADD LETTERING HORIZONTAL SUPPORT FOR GRIDWIDTHS 26 AND UNDER
  // TODO: MAKE NUMBERING/LETTERING + LABEL MARGIN SCALE WITH TILE SIZE
}

function drawLine(x1, y1, x2, y2) {
  ctx.beginPath();
  ctx.moveTo(x1, y1);
  ctx.lineTo(x2, y2);

  ctx.strokeStyle = "#666666";
  ctx.lineWidth = lineWidth;
  ctx.stroke();
}

cnv.addEventListener("resize", () => {
  cnv.width = Math.floor(cnv.getBoundingClientRect().width);
  cnv.height = Math.floor(cnv.getBoundingClientRect().height);
});

cnv.dispatchEvent(new Event("resize"));
