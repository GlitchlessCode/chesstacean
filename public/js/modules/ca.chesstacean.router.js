import { Message } from "./ca.chesstacean.message";
import { Err } from "./ca.chesstacean.result.js";

class RouterError extends Error {
  /**
   * @param {string} [message]
   */
  constructor(message) {
    super();
    this.message = message;
    this.name = "RouterError";
  }
}

class AbstractView {
  /**
   * @returns {Result<Message, RouterError>}
   */
  fetch() {
    return Err(new RouterError("Cannot fetch view abstraction"));
  }
}

export class Router {
  /** @type {Message[]} */
  #cache;

  constructor() {
    this.#cache = [];
  }
}
