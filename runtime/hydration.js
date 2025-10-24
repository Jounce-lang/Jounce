/**
 * Jounce Hydration Runtime
 * Enables client-side interactivity for server-rendered components
 */

(function() {
    'use strict';

    // Hydration state
    const HydrationRuntime = {
        // Initial state from server
        initialState: window.__INITIAL_STATE__ || {},

        // Component registry
        components: new Map(),

        // Event listeners to attach
        eventListeners: [],

        /**
         * Register a component for hydration
         * @param {string} name - Component name
         * @param {Function} component - Component function
         */
        register(name, component) {
            this.components.set(name, component);
        },

        /**
         * Hydrate a server-rendered element
         * @param {string} selector - CSS selector for the root element
         * @param {string} componentName - Name of component to hydrate
         * @param {Object} props - Component props
         */
        hydrate(selector, componentName, props = {}) {
            const root = document.querySelector(selector);
            if (!root) {
                console.warn(`[Hydration] Root element not found: ${selector}`);
                return;
            }

            const component = this.components.get(componentName);
            if (!component) {
                console.warn(`[Hydration] Component not registered: ${componentName}`);
                return;
            }

            // Mark as hydrated
            root.setAttribute('data-hydrated', 'true');

            // Get initial state for this component
            const state = this.initialState[componentName] || {};

            // Attach event listeners from data attributes
            this.attachEventListeners(root);

            // Call component with props and state
            try {
                component.call(null, { ...props, ...state });
                console.log(`[Hydration] ✅ Hydrated component: ${componentName}`);
            } catch (error) {
                console.error(`[Hydration] ❌ Failed to hydrate ${componentName}:`, error);
            }
        },

        /**
         * Attach event listeners based on data attributes
         * @param {Element} root - Root element to scan
         */
        attachEventListeners(root) {
            // Find all elements with data-on-* attributes
            const elements = root.querySelectorAll('[data-on-click], [data-on-input], [data-on-change], [data-on-submit]');

            elements.forEach(el => {
                // Handle click events
                if (el.hasAttribute('data-on-click')) {
                    const handlerName = el.getAttribute('data-on-click');
                    const handler = this.getEventHandler(handlerName);
                    if (handler) {
                        el.addEventListener('click', handler);
                        console.log(`[Hydration] Attached click handler: ${handlerName}`);
                    }
                }

                // Handle input events
                if (el.hasAttribute('data-on-input')) {
                    const handlerName = el.getAttribute('data-on-input');
                    const handler = this.getEventHandler(handlerName);
                    if (handler) {
                        el.addEventListener('input', handler);
                        console.log(`[Hydration] Attached input handler: ${handlerName}`);
                    }
                }

                // Handle change events
                if (el.hasAttribute('data-on-change')) {
                    const handlerName = el.getAttribute('data-on-change');
                    const handler = this.getEventHandler(handlerName);
                    if (handler) {
                        el.addEventListener('change', handler);
                        console.log(`[Hydration] Attached change handler: ${handlerName}`);
                    }
                }

                // Handle submit events
                if (el.hasAttribute('data-on-submit')) {
                    const handlerName = el.getAttribute('data-on-submit');
                    const handler = this.getEventHandler(handlerName);
                    if (handler) {
                        el.addEventListener('submit', (e) => {
                            e.preventDefault();
                            handler(e);
                        });
                        console.log(`[Hydration] Attached submit handler: ${handlerName}`);
                    }
                }
            });
        },

        /**
         * Get event handler function by name
         * @param {string} name - Handler name
         * @returns {Function|null}
         */
        getEventHandler(name) {
            // Look for handler in global scope or registered handlers
            if (typeof window[name] === 'function') {
                return window[name];
            }
            return null;
        },

        /**
         * Hydrate all components on the page
         */
        hydrateAll() {
            // Find all elements with data-component attribute
            const elements = document.querySelectorAll('[data-component]');

            elements.forEach(el => {
                const componentName = el.getAttribute('data-component');
                const propsJson = el.getAttribute('data-props');
                const props = propsJson ? JSON.parse(propsJson) : {};

                const id = el.id || el.getAttribute('data-hydration-id');
                if (id) {
                    this.hydrate(`#${id}`, componentName, props);
                } else {
                    // Hydrate by element reference
                    this.hydrateElement(el, componentName, props);
                }
            });
        },

        /**
         * Hydrate a specific element
         * @param {Element} element - Element to hydrate
         * @param {string} componentName - Component name
         * @param {Object} props - Component props
         */
        hydrateElement(element, componentName, props) {
            const component = this.components.get(componentName);
            if (!component) {
                console.warn(`[Hydration] Component not registered: ${componentName}`);
                return;
            }

            element.setAttribute('data-hydrated', 'true');
            const state = this.initialState[componentName] || {};

            this.attachEventListeners(element);

            try {
                component.call(null, { ...props, ...state });
                console.log(`[Hydration] ✅ Hydrated component: ${componentName}`);
            } catch (error) {
                console.error(`[Hydration] ❌ Failed to hydrate ${componentName}:`, error);
            }
        },

        /**
         * Progressive hydration - hydrate when element is visible
         * @param {string} selector - CSS selector
         * @param {string} componentName - Component name
         * @param {Object} props - Component props
         */
        hydrateWhenVisible(selector, componentName, props = {}) {
            const observer = new IntersectionObserver((entries) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        this.hydrate(selector, componentName, props);
                        observer.disconnect();
                    }
                });
            });

            const element = document.querySelector(selector);
            if (element) {
                observer.observe(element);
            }
        },

        /**
         * Hydrate when idle (after page is fully loaded and interactive)
         * @param {string} selector - CSS selector
         * @param {string} componentName - Component name
         * @param {Object} props - Component props
         */
        hydrateWhenIdle(selector, componentName, props = {}) {
            if ('requestIdleCallback' in window) {
                requestIdleCallback(() => {
                    this.hydrate(selector, componentName, props);
                });
            } else {
                // Fallback for browsers without requestIdleCallback
                setTimeout(() => {
                    this.hydrate(selector, componentName, props);
                }, 1);
            }
        }
    };

    // Export to window
    window.JounceHydration = HydrationRuntime;

    // Auto-hydrate on DOMContentLoaded
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            console.log('[Hydration] Starting auto-hydration...');
            HydrationRuntime.hydrateAll();
        });
    } else {
        // Already loaded, hydrate immediately
        console.log('[Hydration] Starting immediate hydration...');
        HydrationRuntime.hydrateAll();
    }
})();
