import EventEmitter from "./ca.chesstacean.event.js";
import { Message } from "./ca.chesstacean.message.js";
import { Result, Ok, Err } from "./ca.chesstacean.result.js";
import { Token, serialize, deserialize } from "./ca.chesstacean.serde_json.js";

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
  /** @type {boolean} */
  #run;
  /** @type {URL} */
  #url;

  constructor() {
    super();
    this.#url = new URL(location.toString());
    this.#connection = new WebSocket(
      `${to_ws(this.#url)}//${this.#url.host}/ws/connect`
    );
    this.#connection.addEventListener("message", (message_event) => {
      this.#handle(message_event);
    });

    this.#ready = false;
    this.#run = false;
  }

  async connect() {
    if (this.#run) throw new Error("This function has already been run");
    this.#run = true;

    const token = await (
      await fetch(`${to_http(this.#url)}//${this.#url.host}/ws/token`)
    ).text();

    if (!this.ready) {
      await new Promise((resolve, reject) => {
        this.#connection.addEventListener(
          "open",
          () => {
            resolve(this);
          },
          { once: true }
        );
      });
    }

    this.#connection.send(token);
  }

  get ready() {
    this.#ready = this.#connection.readyState == 1;
    return this.#ready && this.#run;
  }

  /**
   * @param {MessageEvent} event
   */
  #handle(event) {
    this.emit("message", deserialize(event.data));
  }

  /**
   * @param {string} message
   * @returns {Result<null, MessageError>}
   */
  #send(message) {
    if (!this.ready)
      return Err(
        new MessageError(
          503,
          "Service Unavailable",
          "Could not successfully connect to the server!"
        )
      );

    try {
      this.#connection.send(message);
      return Ok(null);
    } catch (error) {
      return Err(new MessageError(400, "Bad Request", error.toString()));
    }
  }

  create_lobby() {
    /**
     * @param {import('./ca.chesstacean.message.js').Message} event
     */
    const fn = (event) => {
      if (event.kind == Message) {
      }
    };
    self.on("message", fn);
  }
}

/**
 * @param {URL} url
 */
function to_ws(url) {
  switch (url.protocol) {
    case "https:":
    case "wss:":
      return "wss:";
    default:
      return "ws:";
  }
}

function to_http(url) {
  switch (url.protocol) {
    case "https:":
    case "wss:":
      return "https:";
    default:
      return "http:";
  }
}

/**
 * @param {any} value
 */
function is_object(value) {
  if (value == null || !(typeof value == "object")) return false;
  if (value.__proto__ !== Object.prototype) return false;
  return true;
}

export { ConnectionManager, MessageError };
