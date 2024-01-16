"use strict";

/**
 * @typedef  {object} Camera
 * @property {number} x x coordinate as offset from center of board
 * @property {number} y y coordinate as offset from center of board
 * @property {number} z zoom where zero is fully zoomed out
 */

/** @type {Camera} */
const camera = {};

camera.x = 0;
camera.y = 0;
camera.z = 0;
