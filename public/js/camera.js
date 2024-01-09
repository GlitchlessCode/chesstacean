"use strict";

class Camera {
    /** x coordinate as offset from center of board */
    x;

    /** y coordinate as offset from center of board */
    y;

    zoom;

    /**
     * @param {number} x
     * @param {number} y
     * @param {number} zoom
     */
    constructor(x, y, zoom) {
        this.x    = x;
        this.y    = y;
        this.zoom = zoom;
    }
}

export default Camera;
