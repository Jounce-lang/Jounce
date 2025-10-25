# Development Server Guide - Running Jounce Apps

**The Definitive Guide to Running Jounce Applications**

---

## 🎯 Quick Start

### The Correct Way (Node.js)

```bash
# 1. Compile your Jounce app
jnc compile app.jnc

# 2. Run the generated server
cd dist
node server.js

# 3. Open http://localhost:3000
```

**That's it!** The Jounce compiler generates everything you need.

---

## 🚀 Three Ways to Run Jounce Apps

### Method 1: Production Server (Recommended for Testing)

**Use the auto-generated `server.js`**

```bash
# From your Jounce project root
cargo run -- compile examples/apps/01-hello-counter/main.jnc

# Start the server
cd dist
node server.js
```

**What you get:**
- ✅ Full HTTP server
- ✅ WebAssembly module loading
- ✅ RPC handler endpoints (@server/@client)
- ✅ Static file serving
- ✅ Configurable port (default: 3000)

**Port configuration:**
```bash
# Use a custom port
PORT=8080 node server.js

# Or set it in environment
export PORT=8080
node server.js
```

---

### Method 2: HMR Dev Server (Best for Development)

**Use the Hot Module Replacement server for live reload**

```bash
# Start the HMR server
node scripts/hmr-server.js

# Or if you have jnc in your PATH
jnc dev
```

**What you get:**
- ✅ **Auto-reload** when files change
- ✅ **WebSocket** live updates
- ✅ **Fast refresh** without full page reload
- ✅ **CSS hot reload** (update styles instantly)
- ✅ **File watcher** (monitors your source files)

**Configuration:**
```bash
# Custom port
PORT=4000 node scripts/hmr-server.js

# Custom watch directory
WATCH_DIR=./examples/apps node scripts/hmr-server.js
```

**How it works:**
1. Watches your `.jnc` files for changes
2. Auto-recompiles when you save
3. Sends WebSocket message to browser
4. Browser automatically refreshes

**Perfect for:**
- ✅ Active development
- ✅ Rapid iteration
- ✅ UI tweaking
- ✅ Live debugging

---

### Method 3: Static File Server (Simplest)

**For simple apps with no @server code**

If your app is purely client-side (no @server functions), you can use any static file server:

**Option A: Node.js `http-server`**
```bash
# Install globally (one time)
npm install -g http-server

# Serve from dist/
cd dist
http-server -p 3000

# Open http://localhost:3000
```

**Option B: npx (no install needed)**
```bash
cd dist
npx http-server -p 3000
```

**Option C: Python (legacy, not recommended)**
```bash
cd dist
python3 -m http.server 3000
```

**⚠️ Note**: Static servers work ONLY if:
- No @server functions in your code
- No RPC calls
- Purely client-side rendering
- No WebAssembly loading required

For most Jounce apps, use **Method 1 or 2**.

---

## 📊 Comparison Table

| Feature | Production Server | HMR Dev Server | Static Server |
|---------|------------------|----------------|---------------|
| **Command** | `node server.js` | `node scripts/hmr-server.js` | `http-server` |
| **Auto-reload** | ❌ No | ✅ Yes | ❌ No |
| **@server support** | ✅ Yes | ✅ Yes | ❌ No |
| **WebAssembly** | ✅ Yes | ✅ Yes | ⚠️ Limited |
| **RPC handlers** | ✅ Yes | ✅ Yes | ❌ No |
| **Live CSS reload** | ❌ No | ✅ Yes | ❌ No |
| **File watching** | ❌ No | ✅ Yes | ❌ No |
| **Best for** | Testing/Production | Development | Static sites only |

---

## 🛠️ Complete Development Workflow

### Full Workflow (Recommended)

```bash
# 1. Navigate to your project
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# 2. Create/edit your app
vim examples/apps/my-app/main.jnc

# 3. Compile
cargo run -- compile examples/apps/my-app/main.jnc

# 4. Start dev server with HMR
node scripts/hmr-server.js

# 5. Open browser
open http://localhost:3000

# 6. Edit files - see changes instantly!
```

---

## 🔍 Troubleshooting

### Problem: "Cannot find module '../dist/server-runtime.js'"

**Solution**: Make sure you're running from the `dist/` directory:
```bash
cd dist
node server.js
```

Or use absolute paths:
```bash
node /path/to/jounce/dist/server.js
```

---

### Problem: "Port 3000 is already in use"

**Solution**: Use a different port:
```bash
PORT=3001 node server.js
```

Or kill the process using port 3000:
```bash
# macOS/Linux
lsof -ti:3000 | xargs kill -9

# Windows
netstat -ano | findstr :3000
taskkill /PID <PID> /F
```

---

### Problem: "WebAssembly module not found"

**Solution**: Ensure `app.wasm` exists in `dist/`:
```bash
ls -la dist/app.wasm

# If missing, recompile:
cargo run -- compile your-app.jnc
```

---

### Problem: HMR not reloading

**Solutions**:
1. Check if the file watcher is running (should see "Watching..." message)
2. Ensure you're editing the source `.jnc` file (not the compiled output)
3. Check browser console for WebSocket connection errors
4. Try clearing browser cache (Cmd+Shift+R / Ctrl+Shift+R)

---

## 📦 Generated Files Explained

After running `jnc compile app.jnc`, you'll see:

```
dist/
├── index.html       # Entry point (open this in browser)
├── client.js        # Client-side JavaScript bundle
├── server.js        # Server with HTTP + RPC handlers
├── app.wasm         # WebAssembly module (compiled from Jounce)
├── styles.css       # Generated CSS from your app
├── server.js.map    # Source map for debugging server code
└── client.js.map    # Source map for debugging client code
```

**What each file does:**

- **`index.html`**: HTML entry point, loads `client.js` and `styles.css`
- **`client.js`**: All your @client code + shared code, runs in browser
- **`server.js`**: All your @server code + RPC handlers, runs in Node.js
- **`app.wasm`**: WebAssembly compiled from Jounce (for performance)
- **`styles.css`**: Generated CSS (from utility classes, CSS-in-JS, etc.)
- **`.map files`**: Source maps for debugging (map compiled JS back to .jnc)

---

## 🎨 Serving Apps in Production

### Option 1: Node.js (Recommended)

```bash
# Production build
cargo build --release
jnc compile --minify app.jnc

# Start production server
NODE_ENV=production node dist/server.js
```

### Option 2: Deploy to Vercel

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
cd dist
vercel

# Your app is live!
```

### Option 3: Deploy to Fly.io

```bash
# Install Fly CLI
brew install flyctl  # macOS
# or: curl -L https://fly.io/install.sh | sh

# Launch
fly launch

# Deploy
fly deploy
```

### Option 4: Docker

```dockerfile
# Dockerfile
FROM node:18-alpine
WORKDIR /app
COPY dist/ .
EXPOSE 3000
CMD ["node", "server.js"]
```

```bash
# Build and run
docker build -t my-jounce-app .
docker run -p 3000:3000 my-jounce-app
```

---

## 🚦 Environment Variables

Configure your server with environment variables:

```bash
# Port
PORT=3000                  # Server port (default: 3000)

# Environment
NODE_ENV=production        # production | development | test

# Logging
LOG_LEVEL=info            # trace | debug | info | warn | error

# CORS
CORS_ORIGIN=*             # Allow all origins (or specify domain)

# Database (if using @server with DB)
DATABASE_URL=postgres://localhost/mydb

# Custom variables (access in @server code)
API_KEY=your-api-key
SECRET_KEY=your-secret
```

**Example:**
```bash
PORT=8080 NODE_ENV=production node server.js
```

---

## 📖 Examples

### Example 1: Running App 1 (Hello Counter)

```bash
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# Compile
cargo run -- compile examples/apps/01-hello-counter/main.jnc

# Run
cd dist
node server.js

# Open http://localhost:3000
```

### Example 2: Multi-App Development

```bash
# Terminal 1: HMR server for App 1
cd examples/apps/01-hello-counter
cargo run --manifest-path ../../../Cargo.toml -- compile main.jnc
cd dist
node server.js

# Terminal 2: HMR server for App 2 (different port)
cd examples/apps/02-color-picker
cargo run --manifest-path ../../../Cargo.toml -- compile main.jnc
PORT=3001 node ../../01-hello-counter/dist/server.js
```

### Example 3: Watch Mode + Node Server

```bash
# Terminal 1: Watch and auto-compile
cargo run -- watch examples/apps/01-hello-counter/main.jnc

# Terminal 2: Run server
cd dist
node server.js

# Edit files in Terminal 3, see changes after refresh
```

---

## 🎯 Best Practices

### ✅ DO:

1. **Use Node.js servers** (not Python) for Jounce apps
2. **Use HMR dev server** during active development
3. **Run from `dist/` directory** to avoid path issues
4. **Check `dist/server.js`** if RPC calls fail
5. **Use environment variables** for configuration
6. **Commit compiled files** for quick testing (optional)

### ❌ DON'T:

1. **Don't use Python** (`serve.py`) for Jounce apps
2. **Don't edit compiled files** in `dist/` (edit source `.jnc` files)
3. **Don't skip WebAssembly** if your app uses it
4. **Don't use static servers** for apps with @server code
5. **Don't commit `dist/` to production** (regenerate on deploy)

---

## 🔗 Related Documentation

- **Compilation Guide**: See `GETTING_STARTED.md` for compilation basics
- **HMR Server Code**: See `scripts/hmr-server.js` for implementation
- **CLI Commands**: Run `jnc --help` for all available commands
- **Deployment**: See `DEPLOYMENT.md` (coming soon) for production tips

---

## 📝 Summary

**For Development:**
```bash
jnc compile app.jnc
node scripts/hmr-server.js
```

**For Testing:**
```bash
jnc compile app.jnc
cd dist && node server.js
```

**For Production:**
```bash
jnc compile --minify app.jnc
NODE_ENV=production node dist/server.js
```

**Remember**: Jounce is a Node.js project - always use Node.js servers! 🚀

---

**Questions?** Check the examples in `examples/apps/` or open an issue on GitHub.

**Last Updated**: October 24, 2025
**Jounce Version**: v0.8.0
