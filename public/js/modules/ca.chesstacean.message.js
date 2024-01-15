class ParseError extends Error {
  /**
   * @param {string} message
   * @param {...Error} [errors]
   */
  constructor(message, ...errors) {
    super();

    if (errors.length == 0) {
      this.name = "ParseError";
      this.message = message;
    } else {
      this.name = "ParseErrors";
      this.message = message;
      errors.forEach((pe) => {
        this.message += `\n    and ${pe.message}`;
      });
    }
  }
}

class Types {
  static String = Symbol("String");
  static Number = Symbol("Nubmer");

  static verify = {
    [this.String]: function (maybe_str) {
      if (typeof maybe_str == "string")
        return Ok(maybe_str.replace('\\"', '"'));
      return Err(new TypeError("Captured value is not of type string"));
    },
    [this.Number]: function (maybe_num) {
      const num = Number(maybe_num);
      if (isFinite(num)) return Ok(num);
      return Err(new TypeError("Captured value is not of type number"));
    },
  };
}

class Definition {
  get count() {
    return 1;
  }

  constructor() {}

  /**
   * @template {Definition} F
   * @param {F} other
   * @returns {And<this, F>}
   */
  and(other) {
    other = this.#verify(other);
    return new And(this, other);
  }

  /**
   * @template {Definition} F
   * @param {F} other
   * @returns {Xor<this, F>}
   */
  xor(other) {
    other = this.#verify(other);
    return new Xor(this, other);
  }

  /**
   * @template {Definition} F
   * @param {F} other
   * @returns {With<F>}
   */
  with(other) {
    other = this.#verify(other);
    return new With(this, other);
  }

  /**
   * @template {symbol} F
   * @param {F} other
   * @returns {Capture}
   */
  capture(other) {
    return new Capture(this, other);
  }

  /**
   * @template {symbol} F
   * @param {F} other
   * @returns {Optional}
   */
  capture_optional(other) {
    return new Optional(this, other);
  }

  /**
   * @param {Definition | string} maybe_str
   * @returns
   */
  #verify(maybe_str) {
    if (maybe_str instanceof Definition) return maybe_str;
    return new Named(maybe_str);
  }

  /**
   * @typedef {Result<Object<string, any>, ParseError>} MatchResult
   */

  /**
   * @abstract
   * @param {Token[]} tokens
   * @returns {MatchResult}
   */
  match(tokens) {}
}

class Named extends Definition {
  #name;
  get name() {
    return this.#name;
  }

  /**
   * @param {string} input
   */
  constructor(input) {
    super();
    this.#name = input;
  }

  /**
   * @param {Token[]} tokens
   * @returns {Result<{name:string, content:Token[]|string|null}, ParseError>}
   */
  extract(tokens) {
    const idx = tokens.findIndex((t) => t.name == this.#name);

    if (idx == -1)
      return Err(new ParseError(`Named property '${this.#name}' missing`));

    const [content] = tokens[idx];
    return Ok({ name: this.#name, content });
  }
}

/**
 * @template {Definition} T
 * @template {Definition} U
 */
class And extends Definition {
  get count() {
    return this.#first.count + this.#second.count;
  }

  #first;
  #second;
  /**
   * @param {T} first
   * @param {U} second
   */
  constructor(first, second) {
    super();
    this.#first = first;
    this.#second = second;
  }

  /**
   * @param {Token[]} tokens
   * @returns {MatchResult}
   */
  match(tokens) {
    const results = [this.#first.match(tokens), this.#second.match(tokens)];
    const errs = results.filter((r) => r.is_err()).map((e) => e.unwrap_err());
    if (this.count !== tokens.length) {
      return Err(new ParseError("Incorrect Token Count", ...errs));
    } else if (errs.length !== 0) {
      return Err(
        new ParseError("Error encountered in required branch", ...errs)
      );
    }

    const output = [];
    results
      .map((r) => r.unwrap())
      .map((o) => {
        Object.entries(o).forEach((e) => output.push(e));
      });

    return Ok(Object.fromEntries(output));
  }
}

/**
 * @template {Definition} T
 * @template {Definition} U
 */
class Xor extends Definition {
  #first;
  #second;
  /**
   * @param {T} first
   * @param {U} second
   */
  constructor(first, second) {
    super();
    this.#first = first;
    this.#second = second;
  }

  /**
   * @param {Token[]} tokens
   * @returns {MatchResult}
   */
  match(tokens) {
    const results = [this.#first.match(tokens), this.#second.match(tokens)];
    const errs = results.filter((r) => r.is_err()).map((e) => e.unwrap_err());
    if (errs.length == 2) {
      return Err(new ParseError("Only errors encountered in xor", ...errs));
    }

    const output = [];
    const result = results.filter((r) => r.is_ok());

    if (result.length > 1)
      return Err(
        new ParseError("More than one possibility encountered in xor", ...errs)
      );

    result
      .map((r) => r.unwrap())
      .map((o) => {
        Object.entries(o).forEach((e) => output.push(e));
      });

    return Ok(Object.fromEntries(output));
  }
}

/**
 * @template {Definition} U
 */
class With extends Definition {
  #self;
  #other;
  /**
   * @param {Named} self
   * @param {U} other
   */
  constructor(self, other) {
    super();
    if (!(self instanceof Named))
      throw new TypeError("self must be of type Named");
    this.#self = self;
    this.#other = other;
  }

  /**
   * @param {Token[]} tokens
   * @returns {MatchResult}
   */
  match(tokens) {
    const extracted_token = this.#self.extract(tokens);
    if (extracted_token.is_err())
      return Err(
        new ParseError(
          `Could not extract name '${this.#self.name}'`,
          extracted_token.unwrap_err()
        )
      );

    const token = extracted_token.unwrap();
    if (!(token.content instanceof Array))
      return Err(new ParseError("Token array not found"));

    const result = this.#other.match(token.content);
    if (result.is_err())
      return Err(
        new ParseError("Could not parse sub-tokens", result.unwrap_err())
      );

    return Ok({ [token.name]: result.unwrap() });
  }
}

class Capture extends Definition {
  #self;
  #type;
  /**
   * @param {Named} self
   * @param {symbol} type
   */
  constructor(self, type) {
    super();
    if (!(self instanceof Named))
      throw new TypeError("self must be of type Named");
    this.#self = self;
    this.#type = type;
  }

  /**
   * @param {Token[]} tokens
   * @returns {MatchResult}
   */
  match(tokens) {
    const extracted_token = this.#self.extract(tokens);
    if (extracted_token.is_err())
      return Err(
        new ParseError(
          `Could not extract name '${this.#self.name}'`,
          extracted_token.unwrap_err()
        )
      );

    const token = extracted_token.unwrap();
    if (typeof token.content !== "string")
      return Err(new ParseError("Required capture not found"));

    const captured_value = Types.verify[this.#type](token.content);
    if (captured_value.is_err())
      return Err(
        new ParseError(
          "Target value not of same capture type",
          captured_value.unwrap_err()
        )
      );

    return Ok({ [token.name]: captured_value.unwrap() });
  }
}

class Optional extends Definition {
  #self;
  #type;
  /**
   * @param {Named} self
   * @param {symbol} type
   */
  constructor(self, type) {
    super();
    if (!(self instanceof Named))
      throw new TypeError("self must be of type Named");
    this.#self = self;
    this.#type = type;
  }

  /**
   * @param {Token[]} tokens
   * @returns {MatchResult}
   */
  match(tokens) {
    const extracted_token = this.#self.extract(tokens);
    if (extracted_token.is_err())
      return Err(
        new ParseError(
          `Could not extract name '${this.#self.name}'`,
          extracted_token.unwrap_err()
        )
      );

    const token = extracted_token.unwrap();
    if (token.content instanceof Array)
      return Err(new ParseError("Optional capture not found"));

    if (token.content == null) return Ok({ [token.name]: null });

    const captured_value = Types.verify[this.#type](token.content);
    if (captured_value.is_err())
      return Err(
        new ParseError(
          "Target value not of same capture type",
          captured_value.unwrap_err()
        )
      );

    return Ok({ [token.name]: captured_value.unwrap() });
  }
}

/**
 * @param {string} name
 * @returns {Named}
 */
function define(name) {
  return new Named(name);
}

const context = define("context").with(
  define("message")
    .capture(Types.String)
    .and(define("affects").capture(Types.String))
);

const error = define("Error").with(context);
const code = define("code").capture(Types.String);

const wserror = define("WsError").with(
  define("context").capture_optional(Types.String)
);
const wsconnected = define("WsConnected").with(
  define("display").capture(Types.String)
);
const wsevent = define("WsEvent").with(
  define("event").with(
    define("LobbyCreated")
      .with(code)
      .xor(define("LobbyClosed").with(code))
      .xor(define("JoinedLobby").with(code))
      .xor(define("LeftLobby").with(code))
      .xor(define("LobbyStarted").with(code))
      .xor(define("JoinedQueue"))
      .xor(define("LeftQueue"))
      .xor(define("Matched").with(code))
      .xor(define("JoinedAsSpectator").with(code))
  )
);

class Status {
  static Success = Symbol("Success");
  static Failure = Symbol("Failure");
  static Partial = Symbol("Partial");

  /**
   * @typedef Context
   * @property {string} context
   */
  /**
   * @param {{Success: Context}|{Failure: Context}|{Partial: Context}} status
   */
  static from(status) {
    switch (Object.keys(status)[0]) {
      case "Success": {
        return new this(this.Success, status.Success.context);
      }
      case "Failure": {
        return new this(this.Failure, status.Failure.context);
      }
      case "Partial": {
        return new this(this.Partial, status.Partial.context);
      }
    }
  }

  get kind() {
    return this.#kind;
  }
  get context() {
    return this.#context;
  }

  #kind;
  #context;
  /**
   * @param {symbol} kind
   */
  constructor(kind, context) {
    this.#kind = kind;
    this.#context = context;
  }
}

class Message {
  static DEFINITIONS = error.xor(wserror).xor(wsconnected).xor(wsevent);

  static Error = Symbol("Error");

  static WsError = Symbol("WsError");
  static WsConnected = Symbol("WsConnected");
  static WsEvent = Symbol("WsEvent");

  static from(match) {
    switch (Object.keys(match)[0]) {
      case "Error": {
        const self = new this(this.Error);
        self.body.context = match.Error.context;
        return self;
      }
      case "WsError": {
        const self = new this(this.WsError);
        self.body.context = match.WsError.context;
        return self;
      }
      case "WsConnected": {
        const self = new this(this.WsConnected);
        self.body.display = match.WsConnected.display;
        return self;
      }
      case "WsEvent": {
        const self = new this(this.WsEvent);
        self.body.event = match.WsEvent.event;
      }
    }
  }

  get kind() {
    return this.#kind;
  }

  #kind;
  /** @type {Object<string, any>} */
  body;
  /**
   * @param {symbol} kind
   */
  constructor(kind) {
    this.#kind = kind;
    this.body = {};
  }
}

export { Message };

import { Result, Ok, Err } from "./ca.chesstacean.result.js";
import { Token } from "./ca.chesstacean.serde_json.js";
