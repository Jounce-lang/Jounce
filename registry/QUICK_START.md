# Jounce Registry - Quick Start Deployment

## 🚀 Deploy in 3 Steps

### Step 1: Login to Fly.io

```bash
flyctl auth login
```

This will open your browser for authentication.

### Step 2: Run the Deployment Script

```bash
cd registry
./deploy.sh
```

The script will automatically:
- ✅ Create PostgreSQL database
- ✅ Create Fly.io app
- ✅ Attach database
- ✅ Set environment secrets
- ✅ Create storage volume
- ✅ Deploy the registry
- ✅ Run migrations
- ✅ Test the deployment

**Deployment takes ~5-10 minutes** (first time builds Rust from scratch)

### Step 3: Update Client to Use Production

After deployment, update the client:

**File:** `src/package_manager/registry.rs`

**Change:**
```rust
let base_url = std::env::var("RAVEN_REGISTRY")
    .unwrap_or_else(|_| "https://jounce-registry.fly.dev/api/v1".to_string());
```

Then rebuild:
```bash
cargo build --release
```

---

## 🧪 Test the Deployment

### Test 1: Health Check
```bash
curl https://jounce-registry.fly.dev/health
# Should return: OK
```

### Test 2: Register a User
```bash
curl -X POST https://jounce-registry.fly.dev/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"testpass123"}'
```

### Test 3: Search Packages
```bash
curl https://jounce-registry.fly.dev/api/v1/packages/search?q=raven
```

---

## 📊 Monitoring

### View Logs
```bash
flyctl logs --app jounce-registry
```

### Check Status
```bash
flyctl status --app jounce-registry
```

### SSH into Container
```bash
flyctl ssh console --app jounce-registry
```

### Database Console
```bash
flyctl postgres connect -a jounce-registry-db
```

---

## 🔧 Management Commands

### Redeploy After Changes
```bash
flyctl deploy --app jounce-registry
```

### Scale Resources
```bash
# Increase memory
flyctl scale memory 512 --app jounce-registry

# Increase CPU
flyctl scale vm shared-cpu-2x --app jounce-registry
```

### Restart App
```bash
flyctl apps restart jounce-registry
```

---

## 🌐 Production URLs

- **Registry**: https://jounce-registry.fly.dev
- **API**: https://jounce-registry.fly.dev/api/v1
- **Health**: https://jounce-registry.fly.dev/health

---

## 💰 Cost

**Free Tier:**
- PostgreSQL: Free (256MB, shared-cpu)
- Web App: Free (256MB RAM, shared-cpu, auto-sleep)
- Volume: Free (1GB)
- **Total: $0/month**

**If you exceed free tier:**
- ~$4/month total

---

## 🐛 Troubleshooting

### App Won't Start
```bash
# Check logs
flyctl logs --app jounce-registry

# Common issues:
# - Database connection failed → Check DATABASE_URL secret
# - Port binding → Ensure app listens on 0.0.0.0:8080
# - Missing migrations → Run SQL schema
```

### Database Issues
```bash
# Test connection
flyctl postgres connect -a jounce-registry-db

# Check tables
\dt
```

### Out of Memory
```bash
# Increase memory
flyctl scale memory 512 --app jounce-registry
```

---

## 🎉 You're Live!

Once deployed, you can:
1. Use the registry with `raven pkg` commands
2. Publish your packages
3. Share packages with others
4. Build the Jounce ecosystem!

---

**Questions?** Check the full deployment guide in `DEPLOYMENT.md`
