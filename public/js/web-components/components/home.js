import { html, css } from "../util.js";

export default class HomeElement extends HTMLElement {
  static #shadowTemplate = html` <template> </template> `;

  static #shadowStyle = css``;

  #shadowRoot;

  constructor() {
    super();

    this.#shadowRoot = this.attachShadow({ mode: "closed", slotAssignment: "named" });
    this.#shadowRoot.adoptedStyleSheets = [HomeElement.#shadowStyle];
    this.#shadowRoot.append(
      document.importNode(HomeElement.#shadowTemplate.content, true)
    );
  }
}

customElements.define("c-home", HomeElement);
