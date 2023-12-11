import { html, css } from "../util.js";

export default class PlayerCardElement extends HTMLElement {
  static #shadowTemplate = html`
    <template>
      <div class="turn-marker"></div>
      <div class="avatar"></div>
      <div class="user">
        <slot name="display" class="displayname">NO_DISPLAY_NAME</slot>
        <slot name="handle" class="handle">NO_HANDLE</slot>
      </div>
      <div class="clock">
        <div style="width: 100%" id="time-bar">
          <div id="time-txt"></div>
        </div>
      </div>
      <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path
          d="M12 20C11.45 20 10.9792 19.8042 10.5875 19.4125C10.1958 19.0208 10 18.55 10 18C10 17.45 10.1958 16.9792 10.5875 16.5875C10.9792 16.1958 11.45 16 12 16C12.55 16 13.0208 16.1958 13.4125 16.5875C13.8042 16.9792 14 17.45 14 18C14 18.55 13.8042 19.0208 13.4125 19.4125C13.0208 19.8042 12.55 20 12 20ZM12 14C11.45 14 10.9792 13.8042 10.5875 13.4125C10.1958 13.0208 10 12.55 10 12C10 11.45 10.1958 10.9792 10.5875 10.5875C10.9792 10.1958 11.45 10 12 10C12.55 10 13.0208 10.1958 13.4125 10.5875C13.8042 10.9792 14 11.45 14 12C14 12.55 13.8042 13.0208 13.4125 13.4125C13.0208 13.8042 12.55 14 12 14ZM12 8C11.45 8 10.9792 7.80417 10.5875 7.4125C10.1958 7.02083 10 6.55 10 6C10 5.45 10.1958 4.97917 10.5875 4.5875C10.9792 4.19583 11.45 4 12 4C12.55 4 13.0208 4.19583 13.4125 4.5875C13.8042 4.97917 14 5.45 14 6C14 6.55 13.8042 7.02083 13.4125 7.4125C13.0208 7.80417 12.55 8 12 8Z"
        />
      </svg>
    </template>
  `;

  static #shadowStyle = css`
    :host {
      --avatar-size: calc(var(--game-margin) * 2);
      --color-this-card: var(--color-midbackground);

      display: grid;
      grid-template-rows: var(--avatar-size) 1fr;
      grid-template-columns: var(--game-margin) var(--avatar-size) 1fr min-content;
    }

    :host(.active) {
      --color-this-card: var(--color-accent);
    }

    :host > div.turn-marker {
      --turn-marker-width: calc(var(--game-margin) / 4);
      --turn-marker-border-radius: calc(var(--turn-marker-width) / 2);

      grid-row: 1 / 3;
      grid-column: 1;

      width: var(--turn-marker-width);
      height: 100%;

      transition-property: all;
      transition-duration: 0.2s;

      border-top-right-radius: var(--turn-marker-border-radius);
      border-bottom-right-radius: var(--turn-marker-border-radius);
      background-color: var(--color-this-card);
    }

    :host > div.avatar {
      width: 100%;
      aspect-ratio: 1/1;

      grid-row: 1;
      grid-column: 2;

      transition-property: all;
      transition-duration: 0.2s;

      border-radius: 8px;
      background-color: var(--color-this-card);
    }

    :host > div.user {
      font-family: var(--font-family);
      font-weight: 500;

      grid-row: 1;
      grid-column: 3;

      margin: 0 calc(var(--game-margin) / 2);
    }

    :host > div.user > slot.displayname {
      color: var(--color-foreground);
      font-size: 1.125rem;
    }

    :host > div.user > slot.handle {
      color: var(--color-midforeground);
      font-size: 1rem;
    }

    :host > div.clock {
      --clock-border-width: 4px;

      grid-row: 2;
      grid-column: 2 / 5;

      margin-top: calc(var(--game-margin) / 2);

      border-color: var(--color-this-card);
      border-style: solid;
      border-width: var(--clock-border-width);
      border-radius: 8px;

      transition-property: all;
      transition-duration: 0.2s;

      box-sizing: border-box;
    }

    :host > div.clock > div {
      --vertical-padding: calc(7.5px - var(--clock-border-width));
      --horizontal-padding: calc(10px - var(--clock-border-width));

      transition-property: all;
      transition-duration: 0.2s;

      padding: var(--vertical-padding) var(--horizontal-padding);
      background-color: var(--color-this-card);

      box-sizing: border-box;
    }

    :host > div.clock > div > div {
      font-size: 0.875rem;
      font-family: var(--font-family);
      font-weight: 900;

      background-clip: text;
      -webkit-background-clip: text;
      background-color: var(--color-background);
      filter: invert(1);
    }

    :host > svg {
      width: 24px;
      cursor: pointer;
      aspect-ratio: 1/1;

      transition-property: all;
      transition-duration: 0.2s;

      fill: var(--color-foreground);
    }

    :host > svg:hover {
      fill: var(--color-accent);
    }

    :host > svg:active {
      scale: 0.85;
    }
  `;

  static observedAttributes = ["time"];

  #shadowRoot;

  /** @type {{[x:string]:HTMLElement}} */
  #attributeElements;

  constructor() {
    super();

    this.#shadowRoot = this.attachShadow({ mode: "closed", slotAssignment: "named" });
    this.#shadowRoot.adoptedStyleSheets = [PlayerCardElement.#shadowStyle];
    this.#shadowRoot.append(
      document.importNode(PlayerCardElement.#shadowTemplate.content, true)
    );

    this.#attributeElements = {
      bar: this.#shadowRoot.getElementById("time-bar"),
      txt: this.#shadowRoot.getElementById("time-txt"),
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
    switch (name) {
      case "time": {
        const match = new_value.match(/^(?<a>\d+(?:\.\d+)?)\/(?<b>\d+(?:\.\d+)?)$/);
        if (match instanceof Array && Object.hasOwn(match, "groups")) {
          let seconds = Number(match.groups.a);
          let percent = ((seconds / Number(match.groups.b)) * 100).toFixed(1) || 0;
          this.#attributeElements.bar.style = `width: ${percent}%`;
          this.#attributeElements.txt.innerText = `${
            seconds > 3599
              ? `${(Math.floor(seconds / 3600) % 60).toString().padStart(2, 0)}:`
              : ""
          }${Math.floor((seconds / 60) % 60)
            .toString()
            .padStart(2, 0)}:${Math.floor(seconds % 60)
            .toString()
            .padStart(2, 0)}${
            seconds % 1 != 0
              ? `.${(seconds % 1).toFixed(2).toString().slice(2).padStart(2, 0)}`
              : ""
          }`;
        } else {
          this.#attributeElements.bar.style = "width: 100%";
          this.#attributeElements.txt.innerText = String.fromCharCode(160);
        }
        break;
      }
    }
  }
}

customElements.define("player-card", PlayerCardElement);
