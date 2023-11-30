import EventEmitter from "./ca.chesstacean.event.js";
import { Result, Ok, Err } from "./ca.chesstacean.result.js";

class MessageError extends Error {
  /** @type {number} */
  code;
  /**
   * @param {number} code
   * @param {string} name
   * @param {string} message
   */
  constructor(code, name, message) {
    super();
    this.message = message;
    this.name = `MessageError [${code} ${name}]`;
    this.code = code;
  }
}

class ConnectionManager extends EventEmitter {
  /** @type {WebSocket} */
  #connection;
  /** @type {boolean} */
  #ready;
  /**
   * @param {URL} url
   * @returns {Promise<ConnectionManager>}
   */
  constructor(url) {
    super();
    this.#connection = new WebSocket(url);
    this.#ready = false;
  }

  get ready() {
    this.#ready = this.#connection.readyState == 1;
    return this.#ready;
  }

  #handle() {}

  /**
   * @param {Object} obj
   */
  #send(obj) {
    if (!this.ready)
      return Err(
        new MessageError(
          503,
          "Service Unavailable",
          "Could not successfully connect to the server!"
        )
      );
    if (!isObject(obj))
      return Err(
        new MessageError(
          400,
          "Bad Request",
          `obj was of type ${typeof obj}, must be of type Object`
        )
      );

    try {
      const message = JSON.stringify(obj);

      return Ok("Sent!");
    } catch (error) {
      return Err(
        new MessageError(
          400,
          "Bad Request",
          "Could not parse obj into valid JSON"
        )
      );
    }
  }
}
/**
 * @param {any} value
 */
function isObject(value) {
  if (value == null || !(typeof value == "object")) return false;
  if (value.__proto__ !== Object.prototype) return false;
  return true;
}

export { ConnectionManager, MessageError };
