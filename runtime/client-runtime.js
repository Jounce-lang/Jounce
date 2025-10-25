// Jounce Client Runtime
// Provides JSX rendering and client-side utilities

// Simple JSX createElement function (h function)
export function h(tag, props, ...children) {
    if (typeof tag === 'function') {
        return tag(props, children);
    }

    const element = document.createElement(tag);

    // Set properties
    if (props) {
        for (const [key, value] of Object.entries(props)) {
            if (key === 'className') {
                element.className = value;
            } else if (key === 'class') {
                element.className = value;
            } else if (key.startsWith('on')) {
                const eventName = key.substring(2).toLowerCase();
                element.addEventListener(eventName, value);
            } else if (key === 'style' && typeof value === 'object') {
                Object.assign(element.style, value);
            } else {
                element.setAttribute(key, value);
            }
        }
    }

    // Append children
    for (const child of children.flat()) {
        if (child === null || child === undefined) {
            continue;
        } else if (typeof child === 'string' || typeof child === 'number') {
            element.appendChild(document.createTextNode(String(child)));
        } else if (child instanceof Node) {
            element.appendChild(child);
        }
    }

    return element;
}

// Mount a component to the DOM
export function mountComponent(component, selector = '#app') {
    const container = document.querySelector(selector);
    if (!container) {
        console.error(`Mount target "${selector}" not found`);
        return;
    }

    // Clear existing content
    container.innerHTML = '';

    // Render component
    const rendered = typeof component === 'function' ? component() : component;

    if (rendered instanceof Node) {
        container.appendChild(rendered);
    } else {
        console.error('Component did not return a valid DOM node');
    }
}

// RPC Client for calling server functions
export class RPCClient {
    constructor(baseUrl = '') {
        this.baseUrl = baseUrl;
    }

    async call(functionName, params = {}) {
        const response = await fetch(`${this.baseUrl}/rpc/${functionName}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(params),
        });

        if (!response.ok) {
            throw new Error(`RPC call failed: ${response.statusText}`);
        }

        return await response.json();
    }
}

// Export for window.Jounce global
if (typeof window !== 'undefined') {
    window.Jounce = {
        h,
        mountComponent,
        RPCClient,
    };
}
