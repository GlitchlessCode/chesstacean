import { Message } from "./ca.chesstacean.message.js";
import { Result, Ok, Err } from "./ca.chesstacean.result.js";

export class Token {
  static Value = Symbol("Value");
  static Object = Symbol("Object");
  static Null = Symbol("Null");

  name;
  /** @type {Token[]|string|null} */
  #content;
  /** @type {symbol} */
  #kind;

  get kind() {
    return this.#kind;
  }

  /**
   * @param {string} name
   * @param {Token[]|string|null} value
   */
  constructor(name, value) {
    this.name = name;
    this.#content = value;

    if (typeof value == "string") {
      this.#kind = Token.Value;
    } else if (value == null) {
      this.#kind = Token.Null;
    } else if (value instanceof Array) {
      this.#kind = Token.Object;
    } else {
      throw new TypeError("Invalid enum kind");
    }
  }

  [Symbol.iterator]() {
    let index = -1;
    let data = [this.#content];

    return {
      next: () => ({ value: data[++index], done: !(index in data) }),
    };
  }
}

class StrIter {
  /**
   * @param {string} string
   * @returns
   */
  static from(string) {
    return new this(string);
  }

  #str;
  #position;
  #len;
  consumed;

  get position() {
    return this.#position;
  }

  /**
   * @param {string} string
   */
  constructor(string) {
    this.#str = string;
    this.#len = string.length;
    this.#position = -1;
    this.consumed = false;
  }

  /**
   * @returns {{next: ()=>{value:string, done:boolean}}}
   */
  [Symbol.iterator]() {
    return {
      next: this.#next.bind(this),
    };
  }

  #next() {
    const value = this.#str[0];
    this.#str = this.#str.slice(1);
    this.#position++;
    const done = this.#position > this.#len;
    if (done && !this.consumed) {
      this.consumed = true;
      Object.freeze(this);
    }
    return { value, done };
  }

  /**
   * @param {number} count
   */
  advance_by(count) {
    for (let i = 0; i < count; i++) {
      this.#next();
    }
    return this;
  }

  peek() {
    return this.#str[0];
  }
}

class SerdeError extends Error {
  /**
   * @param {string} message
   */
  constructor(message) {
    super();
    this.name = "SerdeError";
    this.message = message;
  }
}

/**
 * ### Serializes an object
 *
 * Returns a `Result<string, SerdeError>`
 *
 * @param {Object} obj
 * @param {() => Result<Token[], Error>} obj.serialize
 * @returns {Result<string, SerdeError>}
 */
function serialize(obj) {
  try {
    if (typeof obj["serialize"] !== "function") throw new Error();
  } catch (error) {
    return Err(new SerdeError("obj must implement serialize"));
  }
  const tokens = obj.serialize();
  if (!(tokens instanceof Result) || tokens.is_err())
    return Err(new SerdeError("Serialization Error"));

  if (!validate_tokens(tokens.unwrap()))
    return Err(new SerdeError("Serialized result must be a valid Token tree"));

  return Ok(detokenize(tokens));
}

/**
 * ### Detokenizes a valid token tree
 *
 * Returns a `string`
 *
 * @param {Token[]} tokens
 * @returns {string}
 */
function detokenize(tokens) {
  const parts = [];

  for (const token of tokens) {
    switch (token.kind) {
      case Token.Value: {
        const [content] = token;
        parts.push(`"${token.name}":"${content}"`);
        break;
      }
      case Token.Object: {
        const [content] = token;
        const detokenized = detokenize(content);
        parts.push(`"${token.name}":${detokenized}`);
        break;
      }
      case Token.Null: {
        parts.push(`"${token.name}":null`);
        break;
      }
    }
  }

  return `{${parts.join(",")}}`;
}

/**
 * ### Validates a token tree
 *
 * Returns a boolean, indicating whether it is valid or not
 *
 * @param {Token[]} tokens
 * @returns {boolean}
 */
function validate_tokens(tokens) {
  if (!(tokens instanceof Array)) return false;
  for (const token of tokens) {
    if (!(token instanceof Token)) return false;
    switch (token.kind) {
      case Token.Value: {
        const [content] = token;
        if (typeof content !== "string") return false;
        break;
      }
      case Token.Object: {
        const [content] = token;
        const valid = validate_tokens(content);
        if (!valid) return false;
        break;
      }
      case Token.Null: {
        const [content] = token;
        if (content !== null) return false;
        break;
      }
      default:
        return false;
    }

    if (typeof token.name !== "string") return false;
  }
  return true;
}

/**
 * ### Deserializes a JSON string
 *
 * Returns a `Result<Message, SerdeError>`
 *
 * @param {string} json
 * @returns {Result<Message, SerdeError>}
 */
function deserialize(json) {
  // Tokenize, keeping the first token only
  const iter = new StrIter(json);
  const token_result = tokenize(iter);
  if (token_result.is_err()) return token_result;
  if (!iter.advance_by(2).consumed)
    return Err(`Excess junk after closing brace at position ${iter.position - 1}`);
  /** @type {Token[][]} */
  const [[token]] = token_result;

  const match = Message.DEFINITIONS.match([token]);
  if (match.is_err()) return match;

  const message = Message.from(match.unwrap());

  return Ok(message);
}

/**
 * ### Tokenizes a JSON string
 *
 * Returns a `Result<Token[], SerdeError>`
 *
 * @param {StrIter} json
 * @returns {Result<Token[], SerdeError>}
 */
function tokenize(json) {
  const tokens = [];
  // Check for opening brace
  const [opener] = json;
  if (opener !== "{")
    return Err(new SerdeError(`Missing opening brace at position ${json.position}`));

  while (true) {
    // Parse key
    const key_result = tokenize_quotes(json);
    if (key_result.is_err()) return key_result;
    const [key] = key_result;

    // Check for colon
    const [colon] = json;
    if (colon !== ":")
      return Err(new SerdeError(`Missing colon at position ${json.position}`));

    // Check for opening symbol
    const opener = json.peek();
    if (opener == '"') {
      // Parse value
      const value_result = tokenize_quotes(json);
      if (value_result.is_err()) return value_result;
      const value = value_result.unwrap();

      // Add Token to list
      tokens.push(new Token(key, value));
    } else if (opener == "n") {
      const [l1, l2, l3, l4] = json;
      if (l1 + l2 + l3 + l4 == "null") {
        tokens.push(new Token(key, null));
      } else {
        return new Err(`Missing opening brace at position ${json.position - 3}`);
      }
    } else {
      const token_result = tokenize(json);
      if (token_result.is_err()) return token_result;
      tokens.push(new Token(key, token_result.unwrap()));
    }

    // Check if continuing or closing
    const peek = json.peek();
    if (peek == ",") {
      json.advance_by(1);
    } else if (peek == "}") {
      break;
    } else {
      return Err(`Missing closing brace at position ${json.position + 1}`);
    }
  }

  // Skip closing brace
  json.advance_by(1);

  return Ok(tokens);
}

const valid_token = /[a-zA-Z0-9_]/;
/**
 * @param {StrIter} json
 * @returns {Result<string, SerdeError>}
 */
function tokenize_quotes(json) {
  const [open_quote] = json;
  if (open_quote !== '"')
    return Err(new SerdeError(`Missing opening quote at position ${json.position}`));

  let [char] = json;
  let str = "";
  while (valid_token.test(char) && char !== undefined) {
    str += char;
    [char] = json;
  }

  if (str.length == 0)
    return Err(new SerdeError(`Missing token at position ${json.position}`));

  if (char !== '"')
    return Err(new SerdeError(`Missing closing quote at position ${json.position}`));

  return Ok(str);
}

export { serialize, deserialize };

// /**
//  * @param {Token} token
//  */
// function count_tokens(token) {
//   if (token.kind == Token.Object) {
//     /** @type {Token[][]} */
//     const [content] = token;
//     let count = 1;
//     for (const tok of content) {
//       count += count_tokens(tok);
//     }
//     return count;
//   }
//   return 2;
// }
