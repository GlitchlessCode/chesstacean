class UnwrapError extends Error {
  /**
   * @param {string} message
   */
  constructor(message) {
    super();
    this.name = "UnwrapError";
    this.message = message;
  }
}

/**
 * @template T, E
 */
class Result {
  static Ok = Symbol("Ok");
  static Err = Symbol("Err");

  #data;
  #kind;

  /**
   * @param {T|E} content
   * @param {symbol} kind
   */
  constructor(content, kind) {
    this.#data = [content];
    this.#kind = kind;
  }

  get kind() {
    return this.#kind;
  }

  [Symbol.iterator]() {
    let index = -1;
    let data = this.#data;

    return {
      next: () => ({ value: data[++index], done: !(index in data) }),
    };
  }

  is_ok() {
    return this.#kind == Result.Ok;
  }

  is_err() {
    return this.#kind == Result.Err;
  }

  /**
   * ### Returns the contained `Ok` value.
   *
   * Because this function may throw, its use is generally discouraged.
   * Instead, prefer to use switch statements and handle the `Err` case
   * explicitly, or call [`unwrap_or`], or [`unwrap_or_else`].
   *
   * # Throws
   * Throws if the value is an `Err`, with a error message provided by
   * the `Err`'s value.
   *
   * @returns {T}
   * @throws {UnwrapError}
   */
  unwrap() {
    switch (this.kind) {
      case Result.Ok:
        return this.#data[0];
      case Result.Err:
        throw new UnwrapError("Could not unwrap, Result contains Err: " + this.#data[0]);
    }
  }

  /**
   * ### Returns the contained `Ok` value or a provided default.
   *
   * @param {T} or
   * @returns {T}
   */
  unwrap_or(or) {
    switch (this.kind) {
      case Result.Ok:
        return this.#data[0];
      case Result.Err:
        return or;
    }
  }

  /**
   * ### Returns the contained `Ok` value or computes it from a closure.
   *
   * @param {(error:T)=>T} callback
   * @returns {T}
   */
  unwrap_or_else(callback) {
    if (typeof callback !== "function")
      throw new UnwrapError("Could not unwrap, F is not of type function");
    switch (this.kind) {
      case Result.Ok:
        return this.#data[0];
      case Result.Err:
        return callback(this.#data[0]);
    }
  }

  /**
   * ### Returns the contained `Err` value.
   *
   * # Throws
   * Throws if the value is an `Ok`, with a message provided by
   * the `Ok`'s value.
   *
   * @returns {E}
   */
  unwrap_err() {
    switch (this.kind) {
      case Result.Err:
        return this.#data[0];
      case Result.Ok:
        throw new UnwrapError("Could not unwrap, Result contains Ok: " + this.#data[0]);
    }
  }
}

/**
 * @template T
 * @param {T} T
 * @returns {Result<T, E>}
 */
function Ok(T) {
  return new Result(T, Result.Ok);
}

/**
 * @template E
 * @param {E} E
 * @returns {Result<T, E>}
 */
function Err(E) {
  return new Result(E, Result.Err);
}

export { Ok, Err, Result, UnwrapError };
