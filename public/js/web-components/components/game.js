import { html, css } from "../util.js";

export default class GameElement extends HTMLElement {
  static #shadowTemplate = html` <template> </template> `;

  static #shadowStyle = css``;

  #shadowRoot;

  constructor() {
    super();

    this.#shadowRoot = this.attachShadow({ mode: "closed", slotAssignment: "named" });
    this.#shadowRoot.adoptedStyleSheets = [GameElement.#shadowStyle];
    this.#shadowRoot.append(
      document.importNode(GameElement.#shadowTemplate.content, true)
    );
  }
}

customElements.define("c-game", GameElement);
