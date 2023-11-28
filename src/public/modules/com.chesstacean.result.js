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

class Result {
  static Ok = Symbol("Ok");
  static Err = Symbol("Err");

  #data;
  #kind;

  /**
   * @template T
   * @param {T} T
   * @param {Symbol} kind
   */
  constructor(T, kind) {
    this.#data = [T];
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

  /**
   * @template T
   * @returns {T}
   * @throws {UnwrapError}
   */
  unwrap() {
    switch (this.kind) {
      case Result.Ok:
        return this.#data[0];
      case Result.Err:
        throw new UnwrapError(
          "Could not unwrap, Result contains Err: " + this.#data[0]
        );
    }
  }

  /**
   * @template E
   * @template T
   * @param {(error:E)=>T} callback
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
}

/**
 * @template T
 * @param {T} T
 */
function Ok(T) {
  return new Result(T, Result.Ok);
}

/**
 * @template T
 * @param {T} T
 */
function Err(T) {
  return new Result(T, Result.Err);
}

export { Ok, Err, Result };
