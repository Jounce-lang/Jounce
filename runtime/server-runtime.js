// Jounce Server Runtime
// Provides HTTP server and RPC infrastructure for server-side code

const http = require('http');
const url = require('url');
const fs = require('fs');
const path = require('path');

class HttpServer {
    constructor(port = 3000) {
        this.port = port;
        this.rpcHandlers = new Map();
        this.server = null;
    }

    // Register an RPC handler
    rpc(name, handler) {
        this.rpcHandlers.set(name, handler);
    }

    // Start the HTTP server
    start() {
        this.server = http.createServer(async (req, res) => {
            const parsedUrl = url.parse(req.url, true);
            const pathname = parsedUrl.pathname;

            // Serve static files
            if (pathname === '/' || pathname === '/index.html') {
                this.serveFile(res, 'index.html', 'text/html');
            } else if (pathname === '/client.js') {
                this.serveFile(res, 'client.js', 'application/javascript');
            } else if (pathname === '/styles.css') {
                this.serveFile(res, 'styles.css', 'text/css');
            } else if (pathname === '/app.wasm') {
                this.serveFile(res, 'app.wasm', 'application/wasm');
            } else if (pathname.startsWith('/rpc/')) {
                // Handle RPC calls
                const rpcName = pathname.slice(5); // Remove '/rpc/' prefix
                await this.handleRPC(rpcName, req, res);
            } else {
                res.writeHead(404, { 'Content-Type': 'text/plain' });
                res.end('Not Found');
            }
        });

        this.server.listen(this.port, () => {
            console.log(`Server running at http://localhost:${this.port}`);
        });
    }

    // Serve a static file
    serveFile(res, filename, contentType) {
        const filePath = path.join(__dirname, filename);
        fs.readFile(filePath, (err, data) => {
            if (err) {
                res.writeHead(404, { 'Content-Type': 'text/plain' });
                res.end('File not found');
            } else {
                res.writeHead(200, { 'Content-Type': contentType });
                res.end(data);
            }
        });
    }

    // Handle RPC call
    async handleRPC(name, req, res) {
        const handler = this.rpcHandlers.get(name);
        if (!handler) {
            res.writeHead(404, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'RPC handler not found' }));
            return;
        }

        // Read request body
        let body = '';
        req.on('data', chunk => {
            body += chunk.toString();
        });

        req.on('end', async () => {
            try {
                const params = JSON.parse(body || '{}');
                const result = await handler(params);
                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify(result));
            } catch (error) {
                res.writeHead(500, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: error.message }));
            }
        });
    }
}

// WebAssembly loading utility
function loadWasm(wasmPath) {
    const wasmBytes = fs.readFileSync(wasmPath);
    const wasmModule = new WebAssembly.Module(wasmBytes);
    return new WebAssembly.Instance(wasmModule, {});
}

module.exports = {
    HttpServer,
    loadWasm
};
