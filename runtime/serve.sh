#!/bin/bash
# Simple HTTP server for testing Jounce apps locally

echo "🔥 Starting Jounce development server..."
echo "📂 Serving from: $(pwd)"
echo "🌐 Open http://localhost:8000 in your browser"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Use Python's built-in HTTP server
python3 -m http.server 8000
