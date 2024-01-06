import { html, css } from "../util.js";

export default class MoveLiElement extends HTMLElement {
  static #shadowTemplate = html`
    <template>
      <div id="from"></div>
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path d="M14 18L12.6 16.55L16.15 13H4V11H16.15L12.6 7.45L14 6L20 12L14 18Z" />
      </svg>
      <div id="to"></div>
      <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path
          d="M12 20C11.45 20 10.9792 19.8042 10.5875 19.4125C10.1958 19.0208 10 18.55 10 18C10 17.45 10.1958 16.9792 10.5875 16.5875C10.9792 16.1958 11.45 16 12 16C12.55 16 13.0208 16.1958 13.4125 16.5875C13.8042 16.9792 14 17.45 14 18C14 18.55 13.8042 19.0208 13.4125 19.4125C13.0208 19.8042 12.55 20 12 20ZM12 14C11.45 14 10.9792 13.8042 10.5875 13.4125C10.1958 13.0208 10 12.55 10 12C10 11.45 10.1958 10.9792 10.5875 10.5875C10.9792 10.1958 11.45 10 12 10C12.55 10 13.0208 10.1958 13.4125 10.5875C13.8042 10.9792 14 11.45 14 12C14 12.55 13.8042 13.0208 13.4125 13.4125C13.0208 13.8042 12.55 14 12 14ZM12 8C11.45 8 10.9792 7.80417 10.5875 7.4125C10.1958 7.02083 10 6.55 10 6C10 5.45 10.1958 4.97917 10.5875 4.5875C10.9792 4.19583 11.45 4 12 4C12.55 4 13.0208 4.19583 13.4125 4.5875C13.8042 4.97917 14 5.45 14 6C14 6.55 13.8042 7.02083 13.4125 7.4125C13.0208 7.80417 12.55 8 12 8Z"
        />
      </svg>
    </template>
  `;

  static #shadowStyle = css`
    :host {
      font-size: 24px;
      font-family: var(--font-family);
      font-weight: 500;

      display: flex;
      align-items: center;
      justify-content: space-around;

      padding: 24px;
      min-width: 200px;

      color: var(--color-midforeground);
      border-radius: 12px;
      background-color: var(--color-midbackground);
    }

    :host > svg {
      width: 24px;
      aspect-ratio: 1/1;

      fill: var(--color-midforeground);
    }

    :host > svg:last-child {
      cursor: pointer;
      transition-duration: 0.2s;
      transition-property: all;
    }

    :host > svg:last-child:hover {
      fill: var(--color-accent);
    }

    :host > svg:last-child:active {
      scale: 0.85;
    }
  `;

  static observedAttributes = ["from", "to"];

  #shadowRoot;

  /** @type {{[x:string]:HTMLElement}} */
  #attributeElements;

  constructor() {
    super();

    this.#shadowRoot = this.attachShadow({ mode: "closed", slotAssignment: "named" });
    this.#shadowRoot.adoptedStyleSheets = [MoveLiElement.#shadowStyle];
    this.#shadowRoot.append(
      document.importNode(MoveLiElement.#shadowTemplate.content, true)
    );

    this.#attributeElements = {
      from: this.#shadowRoot.getElementById("from"),
      to: this.#shadowRoot.getElementById("to"),
    };
  }

  /**
   * @param {string} name
   * @param {string} old_value
   * @param {string} new_value
   */
  attributeChangedCallback(name, old_value, new_value) {
    if (old_value == new_value) {
      return;
    }
    this.#attributeElements[name].innerText = new_value;
  }
}

customElements.define("move-li", MoveLiElement);
