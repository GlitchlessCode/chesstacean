import { html, css } from "../util.js";

export default class AppElement extends HTMLElement {
  static #shadowTemplate = html`
    <template>
      <div id="loader"></div>
      <div id="content">
        <slot id="slot"></slot>
      </div>
    </template>
  `;

  static #shadowStyle = css`
    :host {
      position: absolute;
      inset: 0;

      background-color: var(--color-midbackground);

      display: flex;

      overflow: hidden;
    }

    :host #loader {
      position: absolute;
      left: 0;
      right: 0;
      top: 0;
      bottom: 0;

      display: grid;
      justify-content: center;
      align-items: center;
    }

    :host #loader::after,
    :host #loader::before {
      content: "";

      box-sizing: border-box;

      width: 400px;
      height: 400px;

      border-radius: 50%;
      border: 4px solid var(--color-foreground);

      grid-row: 1;
      grid-column: 1;

      animation: loader 8s linear infinite;
    }

    :host #loader::after {
      animation-delay: -4s;
    }

    @keyframes loader {
      0% {
        transform: scale(0.1);
        opacity: 0;
      }
      20% {
        opacity: 1;
      }
      100% {
        transform: scale(1);
        opacity: 0;
      }
    }

    :host > #content {
      background-color: var(--color-background);
      position: absolute;
      left: 0;
      right: 0;
      top: 0;
      bottom: 0;
      opacity: 0;
      pointer-events: none;

      clip-path: polygon(0% 0%, 100% -100%, 100% -100%, 0% 0%);

      transition: 0;

      --showing: 0;
    }

    :host > #content.show {
      opacity: 1;
      pointer-events: all;
      --showing: 1;

      clip-path: polygon(0% 0%, 100% -100%, 100% 100%, 0% 200%);

      transition: clip-path 350ms ease-in-out;
    }

    :host > #content.show.hide {
      opacity: 1;
      transition: clip-path 200ms ease-in-out;

      clip-path: polygon(0% 200%, 100% 100%, 100% 100%, 0% 200%);

      --showing: 0;
    }
  `;

  #shadowRoot;

  /** @type {({type: "assign", content: HTMLElement}|{type:"clear"})[]} */
  #actionQueue;

  /** @type {HTMLSlotElement} */
  #contentSlot;
  /** @type {HTMLDivElement} */
  #contentDiv;

  constructor() {
    super();

    this.#shadowRoot = this.attachShadow({
      mode: "closed",
      slotAssignment: "manual",
    });
    this.#shadowRoot.adoptedStyleSheets = [AppElement.#shadowStyle];
    this.#shadowRoot.append(
      document.importNode(AppElement.#shadowTemplate.content, true)
    );

    this.#actionQueue = [];

    this.#contentSlot = this.#shadowRoot.getElementById("slot");
    this.#contentDiv = this.#shadowRoot.getElementById("content");

    this.#running = false;
  }

  clear() {
    this.#actionQueue.push({ type: "clear" });
    if (!this.#running) this.#run();
  }

  assign(node) {
    if (!(node instanceof HTMLElement))
      return TypeError("node must be an instance of type HTMLElement");
    this.#actionQueue.push({ type: "assign", content: node });
    if (!this.#running) this.#run();
  }

  #running;

  #run() {
    if (this.#running) return;

    this.#running = true;

    (async () => {
      while (this.#actionQueue.length > 0) {
        const action = this.#actionQueue.shift();
        switch (action.type) {
          case "assign": {
            await this.#assign(action.content);
            break;
          }
          case "clear": {
            await this.#clear();
            break;
          }
        }
      }

      this.#running = false;
    })();
  }

  /**
   *
   * @param {"show"|"hide"} param
   */
  async #transition(param) {
    const div = this.#contentDiv;
    const start = getComputedStyle(div).getPropertyValue("--showing");

    let resolver;
    const promise = new Promise((r) => (resolver = r));

    const fn = () => {
      const end = getComputedStyle(div).getPropertyValue("--showing");
      if (start !== end) {
        div.removeEventListener("transitionend", fn);
        resolver();
      }
    };
    div.addEventListener("transitionend", fn);

    this.#contentDiv.classList.add(param);
    await promise;

    await sleep(50);
  }

  async #clear() {
    if (this.#contentSlot.assignedNodes().length == 0) {
      await sleep(50);
      return;
    }
    await this.#transition("hide");
    this.innerHTML = "";
    this.#contentDiv.classList.remove("show", "hide");
  }

  async #assign(node) {
    if (this.#contentSlot.assignedNodes().length > 0) await this.#clear();
    this.append(node);
    this.#contentSlot.assign(node);
    await this.#transition("show");
  }
}

function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}

customElements.define("c-app", AppElement);
