#!/usr/bin/env python3
"""
Jounce Development HTTP Server
Serves WASM modules with proper MIME types and CORS support
Run with: python3 serve.py
"""

import http.server
import socketserver
import os

PORT = 8000

class WasmHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add WASM MIME type
        if self.path.endswith('.wasm'):
            self.send_header('Content-Type', 'application/wasm')
        # Enable CORS for local development
        self.send_header('Access-Control-Allow-Origin', '*')
        super().end_headers()

    def log_message(self, format, *args):
        # Colorful logging
        print(f"🌐 {self.address_string()} - {format % args}")

if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    with socketserver.TCPServer(("", PORT), WasmHandler) as httpd:
        print(f"""
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║   🚀 Jounce Development Server                            ║
║                                                            ║
║   Running at: http://localhost:{PORT}                       ║
║                                                            ║
║   Dist files: http://localhost:{PORT}/dist/                 ║
║   Examples:   http://localhost:{PORT}/examples/             ║
║   Tests:      http://localhost:{PORT}/tests/                ║
║                                                            ║
║   Press Ctrl+C to stop                                     ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
        """)

        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n\n👋 Server stopped")
