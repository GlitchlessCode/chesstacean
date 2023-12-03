export default class EventEmitter {
  /** @type {Object<string, Set<Function>>} */
  #events;

  constructor() {
    this.#events = {};
  }

  /**
   * @param {string} event Event name
   * @param {Function} listener Listener function
   */
  on(event, listener) {
    if (!(this.#events[event] instanceof Set)) this.#events[event] = new Set();
    this.#events[event].add(listener);
    return this;
  }

  /**
   * @param {string} event Event name
   * @param {Function} listener Listener function
   */
  removeListener(event, listener) {
    if (this.#events[event] instanceof Set) this.#events[event].delete(listener);
    return this;
  }

  /**
   * @param {string} event Event name
   * @param  {...any} args Additional arguments
   * @returns {boolean} Had listeners?
   */
  emit(event, ...args) {
    if (!(this.#events[event] instanceof Set)) return false;
    if (this.#events[event].size == 0) return false;

    this.#events[event].forEach(function call(fn) {
      fn(...args);
    });

    return true;
  }

  /**
   * @param {string} event Event name
   * @param {Function} listener Listener function
   */
  once(event, listener) {
    const fn = (...args) => {
      this.removeListener(event, fn);
      listener.apply(this, args);
    };
    return this.on(event, fn);
  }
}
