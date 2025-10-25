# DevBoard - Developer Dashboard & Portfolio

A beautiful, modern developer dashboard built with **Jounce** - showcasing the power of full-stack reactive programming compiled to WebAssembly.

![DevBoard Screenshot](https://via.placeholder.com/1200x600/1a1a2e/5a67d8?text=DevBoard+Dashboard)

## 🌟 Features

### Dashboard Section
- **Real-time GitHub Stats** - Fetch and display your GitHub profile metrics
  - Total repositories
  - Stars across all repos
  - Followers/Following
  - Estimated commit count
- **Beautiful stat cards** with hover animations
- **Live data fetching** from GitHub API

### Projects Showcase
- **Project cards** with rich metadata
- **Tech stack badges** showing technologies used
- **Status indicators** (Active, Completed, In Progress, Archived)
- **GitHub & Demo links** for each project
- **Star counts** and metrics

### Blog Section
- **Blog post cards** with excerpts
- **Tag system** for categorization
- **Read time estimates**
- **Publication dates**
- Markdown support (ready for full implementation)

### Contact Form
- **Full form validation**
- **Server-side processing** with Jounce #[server] functions
- **Success/Error messages**
- **Email integration ready** (SendGrid/Mailgun)

### Theme System
- **Dark/Light theme** toggle
- **Smooth transitions** between themes
- **Persistent theme** preference
- **Custom CSS variables** for easy customization

## 🚀 Tech Stack

- **Jounce** - Full-stack reactive programming language
- **WebAssembly** - High-performance compiled output
- **Modern CSS** - CSS Grid, Flexbox, Custom Properties
- **GitHub API** - Real-time data integration
- **Server Functions** - Seamless client-server communication

## 📂 Project Structure

```
devboard/
├── src/
│   └── main.jnc          # Main application code (600+ lines)
├── public/
│   ├── index.html          # HTML entry point
│   └── styles.css          # Modern CSS styling (500+ lines)
├── backend/                # Server-side code (auto-generated)
└── README.md              # This file
```

## 🛠️ Installation & Setup

### Prerequisites
- Jounce compiler installed
- Node.js 18+ (for development server)
- Git (for cloning)

### Quick Start

```bash
# Navigate to devboard directory
cd examples/devboard

# Compile the Jounce code
raven compile src/main.jnc --output public/

# Start development server
raven dev

# Open browser
open http://localhost:3000
```

### Manual Compilation

```bash
# Compile with optimizations
raven compile src/main.jnc \
    --output public/ \
    --optimize \
    --target client

# Compile server bundle
raven compile src/main.jnc \
    --output backend/ \
    --optimize \
    --target server
```

## 🎨 Customization

### Change GitHub Username

Edit `src/main.jnc`, line ~450:

```rust
let stats = fetch_github_stats("YOUR_GITHUB_USERNAME");
```

### Modify Projects

Edit the `get_projects()` server function in `src/main.jnc`:

```rust
#[server]
fn get_projects() -> Vec<Project> {
    return vec![
        Project {
            id: 1,
            name: "Your Project",
            description: "Project description",
            tech_stack: vec!["Rust", "React"],
            github_url: "https://github.com/...",
            demo_url: Some("https://..."),
            stars: 42,
            status: ProjectStatus::Active,
        },
        // Add more projects...
    ];
}
```

### Update Blog Posts

Modify `get_blog_posts()` in `src/main.jnc`:

```rust
#[server]
fn get_blog_posts() -> Vec<BlogPost> {
    return vec![
        BlogPost {
            id: 1,
            title: "Your Post Title",
            slug: "post-slug",
            excerpt: "Brief description...",
            content: "# Full Markdown Content",
            published_date: "2025-10-20",
            tags: vec!["rust", "web"],
            read_time: 10,
        },
    ];
}
```

### Customize Colors

Edit CSS variables in `public/styles.css`:

```css
:root.theme-dark {
    --bg-primary: #1a1a2e;      /* Main background */
    --accent: #5a67d8;          /* Primary accent color */
    --success: #48bb78;         /* Success messages */
    /* ... more variables */
}
```

## 📧 Email Integration

To enable contact form emails, add email service integration in `send_contact_message()`:

### SendGrid Example

```rust
#[server]
fn send_contact_message(message: ContactMessage) -> Result<String, String> {
    // Validate
    if message.email.trim() == "" {
        return Err("Email required");
    }

    // Send via SendGrid
    let client = HttpClient::new("https://api.sendgrid.com");
    let response = client.post("/v3/mail/send")
        .header("Authorization", "Bearer YOUR_API_KEY")
        .json({
            "personalizations": [{
                "to": [{"email": "your@email.com"}]
            }],
            "from": {"email": "noreply@devboard.app"},
            "subject": "New Contact from DevBoard",
            "content": [{
                "type": "text/plain",
                "value": message.message
            }]
        })
        .send();

    if response.status == 202 {
        return Ok("Message sent!");
    } else {
        return Err("Failed to send");
    }
}
```

## 🚢 Deployment

### Vercel

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
vercel deploy
```

### Netlify

```bash
# Install Netlify CLI
npm i -g netlify-cli

# Deploy
netlify deploy --prod
```

### Docker

```dockerfile
FROM node:18-alpine
WORKDIR /app
COPY public /app/public
COPY backend /app/backend
EXPOSE 3000
CMD ["node", "backend/server.js"]
```

```bash
docker build -t devboard .
docker run -p 3000:3000 devboard
```

## 🎯 Features Roadmap

- [ ] **Analytics Dashboard** - Track visitor metrics
- [ ] **RSS Feed** - Auto-generate blog RSS
- [ ] **Search Functionality** - Search projects and posts
- [ ] **Comments System** - Add comments to blog posts
- [ ] **Social Sharing** - Share buttons for posts
- [ ] **Performance Metrics** - Display page speed scores
- [ ] **Mobile App** - React Native version
- [ ] **Admin Panel** - Manage content via UI

## 📊 Code Statistics

```
Total Lines: ~1,100
Jounce Code: ~600 lines
CSS: ~500 lines
Components: 8 major components
Server Functions: 4 endpoints
```

## 🐛 Known Issues

- GitHub API rate limiting (60 requests/hour unauthenticated)
- WASM module size optimization in progress
- Blog markdown rendering (placeholder)

## 🤝 Contributing

This is a showcase app for Jounce. Contributions welcome!

1. Fork the repository
2. Create feature branch
3. Make changes
4. Submit pull request

## 📝 License

MIT License - Feel free to use for your own portfolio!

## 🙏 Acknowledgments

- **Jounce** - The language powering this app
- **GitHub API** - Real-time stats
- **Modern CSS** - Beautiful responsive design

## 📞 Support

- Jounce Docs: [GETTING_STARTED.md](../../GETTING_STARTED.md)
- Issues: [GitHub Issues](https://github.com/yourusername/jounce/issues)
- Discord: [Join Community](https://discord.gg/jounce)

---

**Built with ❤️ using Jounce**

*Showcasing the future of full-stack reactive development*
