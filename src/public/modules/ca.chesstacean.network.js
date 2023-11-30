import EventEmitter from "./ca.chesstacean.event.js";
import { Result, Ok, Err } from "./ca.chesstacean.result.js";

/**
 * @typedef {{error: Function, [x:string]:Function}} Handler
 */

/**
 * @template {{rx: Handler, tx: Handler}} T
 */
class ConnectionManager extends EventEmitter {
  /** @type {WebSocket} */
  #connection;
  /**
   * @param {URL} url
   * @param {ConnectionProtocol<T>} protocol
   */
  constructor(url, protocol) {
    this.#connection = new WebSocket(url, protocol.version);
    this.#connection.addEventListener;
  }

  #handleMessage() {}
}

/**
 * @template {{rx: Handler, tx: Handler}} T
 */
class ConnectionProtocol {
  /** @type {string} */
  #version;
  /** @type {T} */
  #template;

  /**
   * @param {string} version
   * @param {T} template
   */
  constructor(version, template) {
    if (!isObject(template)) throw new TypeError("template must be of type Object");

    if (!verify(template.rx)) new TypeError("rx is not a valid Handler");
    if (!verify(template.tx)) new TypeError("tx is not a valid Handler");

    this.#version = version;
    this.#template = template;
  }

  get version() {
    return this.#version;
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

/**
 * @param {Handler} handler
 * @returns {boolean}
 */
function verify(handler) {
  if (!isObject(handler)) return false;
  if (typeof handler.error !== "function") return false;
  for (const func of template.rx.values()) {
    if (typeof func !== "function") return false;
  }
  return true;
}
