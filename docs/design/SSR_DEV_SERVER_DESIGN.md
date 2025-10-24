# SSR Dev Server & Production Build Design

**Status**: Design document for Tasks 3 & 4 (deferred)
**Created**: 2025-10-24
**Tasks 1-2**: ✅ Complete (JSX conversion & hydration)

---

## Task 3: SSR Dev Server

### Overview
Create a development server that serves SSR-rendered pages with hot module replacement (HMR) and file watching for rapid development.

### Requirements

#### Core Features
- HTTP server serving SSR-rendered HTML
- File watching for auto-regeneration
- Hot module replacement (HMR)
- WebSocket connection for live reload
- Development error overlay
- Source map support

#### Server Architecture
```
┌─────────────────────────────────────────────────┐
│           SSR Dev Server (Port 3000)            │
├─────────────────────────────────────────────────┤
│  HTTP Server (actix-web / axum)                 │
│  ├─ SSR Renderer (existing code)                │
│  ├─ Static File Server (/assets, /dist)         │
│  └─ WebSocket Server (HMR)                      │
├─────────────────────────────────────────────────┤
│  File Watcher (notify crate)                    │
│  ├─ Watch *.jnc files                           │
│  ├─ Detect changes                              │
│  ├─ Recompile on change                         │
│  └─ Trigger WS reload                           │
├─────────────────────────────────────────────────┤
│  HMR Runtime (client-side)                      │
│  ├─ WebSocket client                            │
│  ├─ Apply component updates                     │
│  └─ Preserve state during reload                │
└─────────────────────────────────────────────────┘
```

### Implementation Plan

#### Phase 1: Basic HTTP Server
```rust
// src/dev_server.rs

use actix_web::{web, App, HttpResponse, HttpServer};
use std::path::PathBuf;

pub struct DevServer {
    port: u16,
    root: PathBuf,
    ssr_renderer: SSRRenderer,
}

impl DevServer {
    pub fn new(port: u16, root: PathBuf) -> Self {
        Self {
            port,
            root,
            ssr_renderer: SSRRenderer::new(),
        }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(render_page))
                .route("/{path:.*}", web::get().to(render_page))
                .service(actix_files::Files::new("/assets", "./assets"))
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}

async fn render_page(path: web::Path<String>) -> HttpResponse {
    // 1. Resolve path to .jnc file
    // 2. Compile and render with SSR
    // 3. Return HTML with HMR script
    todo!()
}
```

#### Phase 2: File Watcher
```rust
use notify::{Watcher, RecursiveMode, recommended_watcher};
use std::sync::mpsc::channel;

pub struct FileWatcher {
    paths: Vec<PathBuf>,
    on_change: Box<dyn Fn(PathBuf) + Send>,
}

impl FileWatcher {
    pub fn watch(&mut self) -> notify::Result<()> {
        let (tx, rx) = channel();
        let mut watcher = recommended_watcher(tx)?;

        for path in &self.paths {
            watcher.watch(path, RecursiveMode::Recursive)?;
        }

        for event in rx {
            match event {
                Ok(event) => {
                    if event.kind.is_modify() {
                        for path in event.paths {
                            (self.on_change)(path);
                        }
                    }
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }

        Ok(())
    }
}
```

#### Phase 3: WebSocket HMR
```rust
use actix_web_actors::ws;

pub struct HMRWebSocket {
    clients: Vec<ws::WebsocketContext<Self>>,
}

impl HMRWebSocket {
    pub fn broadcast_reload(&self, file: &str) {
        let message = json!({
            "type": "reload",
            "file": file,
            "timestamp": SystemTime::now()
        });

        for client in &self.clients {
            client.text(message.to_string());
        }
    }
}
```

#### Phase 4: HMR Client Runtime
```javascript
// runtime/hmr-client.js

class HMRClient {
    constructor() {
        this.ws = new WebSocket('ws://localhost:3000/__hmr');
        this.setupListeners();
    }

    setupListeners() {
        this.ws.onmessage = (event) => {
            const data = JSON.parse(event.data);

            switch (data.type) {
                case 'reload':
                    this.handleReload(data);
                    break;
                case 'error':
                    this.showErrorOverlay(data.error);
                    break;
            }
        };
    }

    handleReload(data) {
        console.log(`[HMR] Reloading: ${data.file}`);

        // Try to preserve state
        const state = this.captureState();

        // Reload page
        window.location.reload();

        // Restore state after reload
        sessionStorage.setItem('__HMR_STATE__', JSON.stringify(state));
    }

    captureState() {
        // Capture all component state
        return {
            url: window.location.href,
            scroll: window.scrollY,
            state: window.__INITIAL_STATE__,
        };
    }

    showErrorOverlay(error) {
        const overlay = document.createElement('div');
        overlay.className = 'hmr-error-overlay';
        overlay.innerHTML = `
            <div class="error-content">
                <h1>Compilation Error</h1>
                <pre>${error}</pre>
                <button onclick="this.parentElement.remove()">Dismiss</button>
            </div>
        `;
        document.body.appendChild(overlay);
    }
}

// Auto-start in development
if (process.env.NODE_ENV === 'development') {
    new HMRClient();
}
```

### CLI Integration

```rust
// In src/main.rs

Commands::Dev {
    path,
    port,
    open,
    watch,
} => {
    use jounce_compiler::dev_server::DevServer;

    println!("🚀 Starting dev server on port {}", port);

    let server = DevServer::new(port, path);

    if watch {
        server.enable_watch_mode();
    }

    if open {
        // Open browser automatically
        open::that(format!("http://localhost:{}", port))?;
    }

    server.start().await?;
}
```

### Configuration

```toml
# jounce.toml
[dev]
port = 3000
host = "127.0.0.1"
open = true
watch = true
hmr = true

[dev.proxy]
"/api" = "http://localhost:8080"
```

### Dependencies to Add

```toml
[dependencies]
# Web server
actix-web = "4.4"
actix-web-actors = "4.2"
actix-files = "0.6"

# Or alternative: axum
axum = "0.7"

# File watching
notify = "6.1"

# WebSockets
tokio-tungstenite = "0.21"

# Browser opening
open = "5.0"
```

### Testing Strategy

1. **Unit Tests**: Test file watcher, WebSocket server
2. **Integration Tests**: Full dev server with mock files
3. **Manual Tests**: Real browser testing with HMR

### Success Criteria

- ✅ Server starts on specified port
- ✅ Renders SSR pages on request
- ✅ Detects file changes within 100ms
- ✅ Reloads browser automatically
- ✅ Shows compilation errors in overlay
- ✅ Preserves component state on reload (when possible)

---

## Task 4: Production SSR Build

### Overview
Optimize SSR for production with streaming, caching, and performance enhancements.

### Requirements

#### Core Features
- Streaming SSR (send HTML as it's generated)
- Multi-level caching (in-memory, Redis)
- Static page pre-rendering
- Code splitting for hydration bundles
- Asset optimization (minify, compress)
- CDN integration

#### Production Architecture
```
┌─────────────────────────────────────────────────┐
│         Production SSR Stack                    │
├─────────────────────────────────────────────────┤
│  Load Balancer (nginx / traefik)               │
│  ├─ SSL Termination                             │
│  ├─ Rate Limiting                               │
│  └─ Static Asset Caching                        │
├─────────────────────────────────────────────────┤
│  SSR Server Cluster (Node 1-N)                  │
│  ├─ Streaming Renderer                          │
│  ├─ In-Memory Cache (LRU)                       │
│  └─ Redis Cache (shared)                        │
├─────────────────────────────────────────────────┤
│  CDN Layer (CloudFront / Fastly)                │
│  ├─ Edge Caching                                │
│  ├─ Geo-Distribution                            │
│  └─ HTTPS/2 Push                                │
└─────────────────────────────────────────────────┘
```

### Implementation Plan

#### Phase 1: Streaming SSR
```rust
// src/ssr_stream.rs

use futures::stream::{Stream, StreamExt};
use pin_project::pin_project;

#[pin_project]
pub struct SSRStream {
    vnode: VNode,
    buffer: String,
    position: usize,
}

impl SSRStream {
    pub fn new(vnode: VNode) -> Self {
        Self {
            vnode,
            buffer: String::new(),
            position: 0,
        }
    }

    pub fn render_chunk(&mut self, chunk_size: usize) -> Option<String> {
        // Render next chunk of HTML
        if self.position >= self.buffer.len() {
            return None;
        }

        let chunk = self.buffer[self.position..].chars()
            .take(chunk_size)
            .collect::<String>();

        self.position += chunk.len();
        Some(chunk)
    }
}

impl Stream for SSRStream {
    type Item = Result<Bytes, Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let chunk = self.render_chunk(8192); // 8KB chunks
        match chunk {
            Some(html) => Poll::Ready(Some(Ok(Bytes::from(html)))),
            None => Poll::Ready(None),
        }
    }
}
```

#### Phase 2: Caching Layer
```rust
// src/ssr_cache.rs

use lru::LruCache;
use std::sync::{Arc, RwLock};
use redis::Client;

pub struct SSRCache {
    // L1: In-memory LRU cache
    memory_cache: Arc<RwLock<LruCache<String, CachedPage>>>,

    // L2: Redis distributed cache
    redis_client: Option<Client>,

    // Configuration
    ttl: Duration,
    max_size: usize,
}

#[derive(Clone)]
pub struct CachedPage {
    pub html: String,
    pub headers: HashMap<String, String>,
    pub timestamp: SystemTime,
    pub etag: String,
}

impl SSRCache {
    pub fn new(max_size: usize, redis_url: Option<&str>) -> Self {
        let redis_client = redis_url.map(|url| {
            Client::open(url).expect("Failed to connect to Redis")
        });

        Self {
            memory_cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(max_size).unwrap()
            ))),
            redis_client,
            ttl: Duration::from_secs(3600), // 1 hour
            max_size,
        }
    }

    pub fn get(&self, key: &str) -> Option<CachedPage> {
        // Try L1 cache
        if let Some(page) = self.memory_cache.write().unwrap().get(key) {
            return Some(page.clone());
        }

        // Try L2 cache (Redis)
        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_connection() {
                if let Ok(data) = redis::cmd("GET").arg(key).query::<String>(&mut conn) {
                    if let Ok(page) = serde_json::from_str::<CachedPage>(&data) {
                        // Populate L1 cache
                        self.memory_cache.write().unwrap().put(key.to_string(), page.clone());
                        return Some(page);
                    }
                }
            }
        }

        None
    }

    pub fn set(&self, key: String, page: CachedPage) {
        // Set L1 cache
        self.memory_cache.write().unwrap().put(key.clone(), page.clone());

        // Set L2 cache (Redis)
        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_connection() {
                let data = serde_json::to_string(&page).unwrap();
                let _: Result<(), _> = redis::cmd("SETEX")
                    .arg(key)
                    .arg(self.ttl.as_secs())
                    .arg(data)
                    .query(&mut conn);
            }
        }
    }

    pub fn generate_cache_key(path: &str, query: &str, headers: &HeaderMap) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        query.hash(&mut hasher);

        // Include relevant headers in cache key
        if let Some(cookie) = headers.get("cookie") {
            cookie.hash(&mut hasher);
        }

        format!("ssr:{:x}", hasher.finish())
    }
}
```

#### Phase 3: Static Pre-rendering
```rust
// src/ssr_prerender.rs

pub struct StaticPrerenderer {
    routes: Vec<String>,
    output_dir: PathBuf,
}

impl StaticPrerenderer {
    pub fn prerender_all(&self) -> Result<(), Error> {
        for route in &self.routes {
            let html = self.render_route(route)?;
            let output_path = self.output_dir.join(
                format!("{}/index.html", route.trim_start_matches('/'))
            );

            fs::create_dir_all(output_path.parent().unwrap())?;
            fs::write(output_path, html)?;
        }

        Ok(())
    }

    fn render_route(&self, route: &str) -> Result<String, Error> {
        // Render route to static HTML
        todo!()
    }
}
```

#### Phase 4: Build Command
```bash
# Build for production
jnc build --ssr \
  --output dist \
  --prerender routes.txt \
  --optimize \
  --minify \
  --compress

# Output structure:
dist/
├── index.html (pre-rendered)
├── about/
│   └── index.html (pre-rendered)
├── assets/
│   ├── app.js (hydration bundle, minified)
│   ├── app.css (minified)
│   └── vendors.js (code-split)
└── server/
    └── ssr-server (production binary)
```

### Performance Optimizations

#### 1. Component-Level Caching
```rust
pub fn render_with_cache(
    component: &Component,
    cache: &SSRCache,
) -> String {
    let cache_key = component.get_cache_key();

    if let Some(cached) = cache.get(&cache_key) {
        return cached.html;
    }

    let html = render_component(component);
    cache.set(cache_key, CachedPage::new(html.clone()));
    html
}
```

#### 2. Lazy Hydration
```javascript
// Only hydrate components when they enter viewport
const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const component = entry.target.getAttribute('data-component');
            JounceHydration.hydrate(entry.target, component);
            observer.unobserve(entry.target);
        }
    });
});
```

#### 3. Edge Caching Headers
```rust
pub fn add_cache_headers(response: &mut Response) {
    response.headers_mut().insert(
        "Cache-Control",
        HeaderValue::from_static("public, max-age=3600, s-maxage=86400")
    );
    response.headers_mut().insert(
        "CDN-Cache-Control",
        HeaderValue::from_static("max-age=86400")
    );
}
```

### Deployment

#### Docker Container
```dockerfile
# Dockerfile.ssr
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin ssr-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/ssr-server /usr/local/bin/
COPY --from=builder /app/dist /app/dist

EXPOSE 3000
CMD ["ssr-server", "--port", "3000"]
```

#### Kubernetes Deployment
```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: jounce-ssr
spec:
  replicas: 3
  selector:
    matchLabels:
      app: jounce-ssr
  template:
    spec:
      containers:
      - name: ssr-server
        image: jounce/ssr-server:latest
        ports:
        - containerPort: 3000
        env:
        - name: REDIS_URL
          value: "redis://redis-service:6379"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
```

### Dependencies to Add

```toml
[dependencies]
# Streaming
futures = "0.3"
pin-project = "1.1"

# Caching
lru = "0.12"
redis = { version = "0.24", optional = true }

# Compression
flate2 = "1.0"
brotli = "3.4"

# Optimization
minifier = "0.3"
```

### Success Criteria

- ✅ Streaming SSR sends first byte < 50ms
- ✅ Cache hit ratio > 80% for repeated requests
- ✅ Static pages pre-rendered correctly
- ✅ Hydration bundle < 50KB gzipped
- ✅ TTI (Time to Interactive) < 2s on 3G
- ✅ Lighthouse score > 95

---

## Timeline Estimate

### Task 3: SSR Dev Server
- **Phase 1** (Basic server): 2-3 days
- **Phase 2** (File watcher): 1-2 days
- **Phase 3** (WebSocket HMR): 2-3 days
- **Phase 4** (HMR client): 1-2 days
- **Testing & Polish**: 2 days
- **Total**: ~2 weeks

### Task 4: Production SSR
- **Phase 1** (Streaming): 2-3 days
- **Phase 2** (Caching): 3-4 days
- **Phase 3** (Pre-rendering): 2-3 days
- **Phase 4** (Build system): 2-3 days
- **Deployment & Docs**: 2-3 days
- **Total**: ~2-3 weeks

**Combined Total**: ~4-5 weeks

---

## References

- [React Server Components](https://react.dev/reference/react/server)
- [Next.js ISR](https://nextjs.org/docs/basic-features/data-fetching/incremental-static-regeneration)
- [Vite HMR](https://vitejs.dev/guide/api-hmr.html)
- [Actix Web](https://actix.rs/)
- [Axum Framework](https://github.com/tokio-rs/axum)
