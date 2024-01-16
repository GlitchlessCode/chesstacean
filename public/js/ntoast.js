'use strict';

/**
 * Settings to apply to the toast.
 * @typedef {object} ToastSettings
 *
 * @property {string} title - The title.
 * @property {string} message - The message.
 *
 * @property {string} iconAlt - The icon <img>'s alt attribute value.
 * @property {string} iconSrc - The icon <img>'s src attribute value.
 * @property {HTMLElement} iconElement - The icon's element. Has less precedence than iconSrc.
 *
 * @property {string} cssAccentColor - The CSS accent color value.
 * @property {string} cssBackgroundColor - The CSS background color value.
 *
 * @property {boolean} iconHidden - Whether or not to hide the icon.
 * @property {boolean} titleHidden - Whether or not to hide the title.
 * @property {boolean} messageHidden - Whether or not to hide the message.
 * @property {boolean} sidebarHidden - Whether or not to hide the sidebar.
 * @property {boolean} progressbarHidden - Whether or not to hide the progressbar.
 *
 * @property {number} durationMilliseconds - The amount of time that should be taken before exiting.
 * @property {string} position - The position.
 */

const toastWidthPixels = 368;
const toastMarginPixels = 16;
const toastMinHeightPixels = 84;

/**
 * Builds an svg element.
 * @param {string} d - The icon <path>'s d attribute value.
 * @returns {HTMLElement} The svg element.
 */
function buildIconElement(d) {
	const svg = document.createElement('svg');

	svg.setAttribute('viewbox', '0 0 16 16');
	svg.setAttribute('xmlns', 'http://www.w3.org/2000/svg');

	svg.innerHTML = `
		<path
			clip-rule="evenodd",
			fill-rule="evenodd",
			d="${d}"
		></path>
	`;

	return svg;
}

class Toast {

	// ---> Member Variables

	/**
	 * The title.
	 * @type {string}
	 */
	title = 'Notification';

	/**
	 * The title.
	 * @type {string}
	 */
	message = '[error: undefined message]';

	/**
	 * The icon <img>'s alt attribute value.
	 * @type {string}
	 */
	iconAlt;

	/**
	 * The icon <img>'s src attribute value.
	 * @type {string}
	 */
	iconSrc;

	/**
	 * The icon's element. Has less precedence than iconSrc.
	 * @type {string}
	 */
	iconElement = buildIconElement('M8 16C12.4183 16 16 12.4183 16 8C16 3.58172 12.4183 0 8 0C3.58172 0 0 3.58172 0 8C0 12.4183 3.58172 16 8 16ZM11.5 8L8.67573 10.8243C8.30254 11.1975 7.69746 11.1975 7.32426 10.8243C6.89784 10.3978 6.96814 9.68791 7.46991 9.35339L8 9H5.5C4.94772 9 4.5 8.55228 4.5 8C4.5 7.44772 4.94772 7 5.5 7H8L7.46991 6.64661C6.96814 6.31209 6.89784 5.60216 7.32426 5.17574C7.69746 4.80254 8.30254 4.80254 8.67573 5.17573L11.5 8Z');

	/**
	 * The CSS accent color value.
	 * @type {string}
	 */
	cssAccentColor = '#073B4C';

	/**
	 * The CSS background color value.
	 * @type {string}
	 */
	cssBackgroundColor = '#FFFFFF';

	/**
	 * Whether or not to hide the icon.
	 * @type {boolean}
	 */
	iconHidden = false;

	/**
	 * @type {boolean}
	 * Whether or not to hide the title.
	 */
	titleHidden = false;

	/**
	 * @type {boolean}
	 * Whether or not to hide the message.
	 */
	messageHidden = false;

	/**
	 * @type {boolean}
	 * Whether or not to hide the sidebar.
	 */
	sidebarHidden = false;

	/**
	 * @type {boolean}
	 * Whether or not to hide the progressbar.
	 */
	progressbarHidden = false;

	/**
	 * The amount of time that should be taken before exiting.
	 * @type {number}
	 */
	durationMilliseconds = 1500;

	/**
	 * The position.
	 * @type {string}
	 */
	position = Toast.positions.topLeft;

	// ---> Member Functions

	/**
	 * Sends the toast.
	 * @returns {HTMLElement} The toast element.
	 */
	async send() {
		// Build toast.

		const ntoastContainer = document.querySelector(`div.ntoast-container.ntoast-${this.position}`);

		const toast = document.createElement('div');
		toast.classList = `ntoast-card`;

		toast.style.setProperty('--accent-color', this.cssAccentColor);
		toast.style.setProperty('--background-color', this.cssBackgroundColor);
		toast.style.minHeight = toastMinHeightPixels + 'px';
		toast.style.left = toastMarginPixels + 'px';

		if (ntoastContainer.classList.contains('ntoast-top-left') || ntoastContainer.classList.contains('ntoast-top-right'))
			toast.style.top = -toastMinHeightPixels + 'px';
		else
			toast.style.bottom = -toastMinHeightPixels + 'px';

		toast.onclick = () => Toast.receive(toast);

		if (!this.sidebarHidden)
			toast.innerHTML += '<div class="ntoast-sidebar"></div>';

		if (!this.progressbarHidden && this.durationMilliseconds > 0)
			toast.innerHTML += '<div class="ntoast-progressbar" style="width: 100%"></div>'

		ntoastContainer.insertBefore(toast, ntoastContainer.firstChild);

		// Build content.

		const content = document.createElement('div');
		content.classList.add('ntoast-content');

		if (!this.iconHidden)
			content.innerHTML += this.iconSrc === undefined
				? this.iconElement.classList.add('ntoast-icon') || this.iconElement.outerHTML
				: `<img class="ntoast-icon" src=${this.iconSrc} alt="${this.iconAlt | 'icon'}"></img>`;

		toast.appendChild(content);

		// Build text content.

		const textContent = document.createElement('div');
		textContent.classList.add('ntoast-text-content');

		if (!this.titleHidden)
			textContent.innerHTML += `<div class="ntoast-title">${this.title}</div>`;

		if (!this.messageHidden)
			textContent.innerHTML += `<div class="ntoast-message">${this.message}</div>`;

		content.appendChild(textContent);

		// Animate entrance.

		const step = 5;
		const iterations = (toastMinHeightPixels + toastMarginPixels) / step;
		const onTop = ntoastContainer.classList.contains('ntoast-top-left') || ntoastContainer.classList.contains('ntoast-top-right');

		for (let i = 0; i < iterations; i++) {
			for (let i = 0; i < ntoastContainer.childElementCount; i++) {
				const card = Array.from(ntoastContainer.children)[i];

				if (onTop)
					card.style.top = +card.style.top.slice(0, card.style.top.length - 2) + step + 'px';
				else
					card.style.bottom = +card.style.bottom.slice(0, card.style.bottom.length - 2) + step + 'px';
			}
			await new Promise(r => setTimeout(r, 4));
		}

		if (this.durationMilliseconds > 0) {
			setTimeout(async () => {
				const step = 0.1;
				const progressbar = toast.querySelector('div.ntoast-progressbar');
				const sleepDuration = this.durationMilliseconds / 100 * step;
				for (let i = 0; i < 100 / step; i++) {
					await new Promise(r => setTimeout(r, sleepDuration));
					progressbar.style.width = +progressbar.style.width.slice(0, progressbar.style.width.length - 1) - step + '%';
				}
				Toast.receive(toast);
			}, 0);
	}

		return toast;
	}

	// ---> Constructor

	/** @param {ToastSettings} settings - The settings. */
	constructor(settings = {}) {
		for (const key in settings)
			this[key] = settings[key];
	}

	// -*- Static variables -*-

	static positions = {
		bottomLeft: 'bottom-left',
		bottomRight: 'bottom-right',
		topLeft: 'top-left',
		topRight: 'top-right',
	};

	// -*- Static Functions -*-

	/**
	 * Removes a toast.
	 * @param {HTMLElement} toast - The toast.
	 * @returns {void}
	 */
	static receive(toast) {
		setTimeout(async () => {
			// in case toast was deleted, in case it would no longer have a parent element attached to it
			if (toast.parentElement == null)
				return;

			const toastStep = 20;
			const vector = toast.parentElement.classList.contains('ntoast-top-right') || toast.parentElement.classList.contains('ntoast-bottom-right') ? toastStep : -toastStep;

			for (let i = 0; i < toastWidthPixels / toastStep; i++) {
				toast.style.left = (+toast.style.left.slice(0, toast.style.left.length - 2)) + vector + 'px';
				await new Promise(r => setTimeout(r, 4));
			}

			const ntoastContainer = toast.parentElement;
			const index = Array.from(ntoastContainer.children).indexOf(toast);
			const elements = Array.from(ntoastContainer.children).slice(index, ntoastContainer.childElementCount)
			toast.remove();

			const step = 5;
			const iterations = (toastMinHeightPixels + toastMarginPixels) / step;
			const onTop = ntoastContainer.classList.contains('ntoast-top-left') || ntoastContainer.classList.contains('ntoast-top-right');

			for (let i = 0; i < iterations; i++) {
				for (let i = 0; i < elements.length; i++) {
					const card = elements[i];

					if (onTop)
						card.style.top = +card.style.top.slice(0, card.style.top.length - 2) - step + 'px';
					else
						card.style.bottom = +card.style.bottom.slice(0, card.style.top.length - 2) - step + 'px';
				}
				await new Promise(r => setTimeout(r, 4));
			}
		}, 0);
	}
};

// Templates

Toast.Error = class extends Toast {

	// ---> Constructor

	/** @type {ToastSettings} settings - The settings. */
	constructor(settings = {}) {
		super(settings);

		this.title = settings.title || 'Error';
		this.cssAccentColor = settings.cssAccentColor || '#EF476F';
		this.iconElement = settings.iconElement || buildIconElement('M8 16C12.4183 16 16 12.4183 16 8C16 3.58172 12.4183 0 8 0C3.58172 0 0 3.58172 0 8C0 12.4183 3.58172 16 8 16ZM5.20711 10.7071L5.29289 10.7929C5.68342 11.1834 6.31658 11.1834 6.70711 10.7929L8 9.5L9.29289 10.7929C9.68342 11.1834 10.3166 11.1834 10.7071 10.7929L10.7929 10.7071C11.1834 10.3166 11.1834 9.68342 10.7929 9.29289L9.5 8L10.7929 6.70711C11.1834 6.31658 11.1834 5.68342 10.7929 5.29289L10.7071 5.20711C10.3166 4.81658 9.68342 4.81658 9.29289 5.20711L8 6.5L6.70711 5.20711C6.31658 4.81658 5.68342 4.81658 5.29289 5.20711L5.20711 5.29289C4.81658 5.68342 4.81658 6.31658 5.20711 6.70711L6.5 8L5.20711 9.29289C4.81658 9.68342 4.81658 10.3166 5.20711 10.7071Z');
	}
};

Toast.Warning = class extends Toast {

	// ---> Constructor

	/** @type {ToastSettings} settings - The settings. */
	constructor(settings = {}) {
		super(settings);

		this.title = settings.title || 'Warning';
		this.cssAccentColor = settings.cssAccentColor || '#FFD166';
		this.iconElement = settings.iconElement || buildIconElement('M8 16C12.4183 16 16 12.4183 16 8C16 3.58172 12.4183 0 8 0C3.58172 0 0 3.58172 0 8C0 12.4183 3.58172 16 8 16ZM4.12946 9.16876C3.65669 9.83063 4.12982 10.75 4.94319 10.75H11.0568C11.8702 10.75 12.3433 9.83063 11.8705 9.16876L8.81373 4.88923C8.41491 4.33088 7.58509 4.33088 7.18627 4.88923L4.12946 9.16876Z');
	}
};

Toast.Success = class extends Toast {

	// ---> Constructor

	/** @type {ToastSettings} settings - The settings. */
	constructor(settings = {}) {
		super(settings);

		this.title = settings.title || 'Success';
		this.cssAccentColor = settings.cssAccentColor || '#06D6A0';
		this.iconElement = settings.iconElement || buildIconElement('M8 16C12.4183 16 16 12.4183 16 8C16 3.58172 12.4183 0 8 0C3.58172 0 0 3.58172 0 8C0 12.4183 3.58172 16 8 16ZM5.46447 7.94975L6.87868 9.36396L10.4142 5.82843C10.8047 5.4379 11.4379 5.4379 11.8284 5.82843C12.219 6.21895 12.219 6.85212 11.8284 7.24264L7.58579 11.4853C7.19526 11.8758 6.5621 11.8758 6.17157 11.4853L4.05025 9.36396C3.65973 8.97344 3.65973 8.34027 4.05025 7.94975C4.44078 7.55922 5.07394 7.55922 5.46447 7.94975Z');
	}
};

Toast.Information = class extends Toast {

	// ---> Constructor

	/** @type {ToastSettings} settings - The settings. */
	constructor(settings = {}) {
		super(settings);

		this.title = settings.title || 'Information';
		this.cssAccentColor = settings.cssAccentColor || '#118AB2';
		this.iconElement = settings.iconElement || buildIconElement('M8 16C12.4183 16 16 12.4183 16 8C16 3.58172 12.4183 0 8 0C3.58172 0 0 3.58172 0 8C0 12.4183 3.58172 16 8 16ZM6.75 4.5V4.75C6.75 5.30228 7.19772 5.75 7.75 5.75H8.25C8.80228 5.75 9.25 5.30228 9.25 4.75V4.5C9.25 3.94772 8.80228 3.5 8.25 3.5H7.75C7.19772 3.5 6.75 3.94772 6.75 4.5ZM6.75 11.5C6.75 12.0523 7.19772 12.5 7.75 12.5H8.25C8.80228 12.5 9.25 12.0523 9.25 11.5V7.65C9.25 7.09771 8.80228 6.65 8.25 6.65H7.75C7.19772 6.65 6.75 7.09772 6.75 7.65V11.5Z');
	}
};

(() => {
	const head = document.getElementsByTagName('head')[0];
	const body = document.getElementsByTagName('body')[0];

	let item;

	// Inject dependencies.

	item = document.createComment("NToast injected code...");
	head.appendChild(item);

	item = document.createElement("link");
	item.setAttribute("rel", "preconnect");
	item.setAttribute("href", "https://fonts.googleapis.com");
	head.appendChild(item);

	item = document.createElement("link");
	item.setAttribute("rel", "preconnect");
	item.setAttribute("href", "https://fonts.gstatic.com");
	item.setAttribute("crossorigin", '');
	head.appendChild(item);

	item = document.createElement("link");
	item.setAttribute("rel", "stylesheet");
	item.setAttribute("href", "https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap");
	head.appendChild(item);

	item = document.createElement("link");
	item.setAttribute("rel", "stylesheet");
	item.setAttribute("href", "/css/ntoast.css");
	head.appendChild(item);

	// Inject boilerplate.

	item = document.createComment("NToast injected code...");
	body.appendChild(item);

	item = document.createElement("div");
	item.classList.add("ntoast-container", "ntoast-top-left");
	body.appendChild(item);

	item = document.createElement("div");
	item.classList.add("ntoast-container", "ntoast-top-right");
	body.appendChild(item);

	item = document.createElement("div");
	item.classList.add("ntoast-container", "ntoast-bottom-left");
	body.appendChild(item);

	item = document.createElement("div");
	item.classList.add("ntoast-container", "ntoast-bottom-right");
	body.appendChild(item);
})();
