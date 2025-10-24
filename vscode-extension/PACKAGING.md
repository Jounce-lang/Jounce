# Packaging & Publishing Guide

This guide explains how to package and publish the Jounce VS Code extension.

## Prerequisites

1. **Extension Icon** - Create `icon.png` (128x128 pixels)
2. **Repository URL** - Update `package.json` with repository information
3. **Publisher Account** - Create account on VS Code Marketplace (for publishing)

## Step 1: Update package.json

Add repository information to `package.json`:

```json
{
  "repository": {
    "type": "git",
    "url": "https://github.com/jounce/jounce.git"
  },
  "bugs": {
    "url": "https://github.com/jounce/jounce/issues"
  },
  "homepage": "https://github.com/jounce/jounce#readme"
}
```

## Step 2: Create Extension Icon

Create a 128x128 PNG icon and save as `icon.png` in this directory.

## Step 3: Package the Extension

```bash
# Install dependencies (if not already installed)
npm install

# Compile TypeScript
npm run compile

# Package the extension (creates .vsix file)
npm run package

# Or use vsce directly
npx vsce package
```

This will create a file like `jounce-0.1.0.vsix` that can be:
- Installed locally for testing
- Shared with others
- Published to the VS Code Marketplace

## Step 4: Test the Packaged Extension

### Install Locally
```bash
code --install-extension jounce-0.1.0.vsix
```

### Test in VS Code
1. Open a `.jnc` file
2. Verify syntax highlighting works
3. Test LSP features (completions, hover, go-to-def)
4. Test commands (compile, watch, format)
5. Check settings work correctly

## Step 5: Publish to Marketplace (Optional)

### Create Publisher Account
1. Go to https://marketplace.visualstudio.com/manage
2. Create a publisher account
3. Get a Personal Access Token (PAT) from Azure DevOps

### Publish
```bash
# Login with your publisher
npx vsce login <publisher-name>

# Publish the extension
npx vsce publish

# Or publish a specific version
npx vsce publish 0.1.0
```

### Update package.json Publisher
Change the `"publisher"` field in `package.json` from `"jounce"` to your actual publisher name.

## Versioning

Follow semantic versioning (semver):
- **Patch** (0.1.x): Bug fixes
- **Minor** (0.x.0): New features (backward compatible)
- **Major** (x.0.0): Breaking changes

Update version in `package.json` before each release.

## Pre-Publishing Checklist

- [ ] Extension icon created (`icon.png`)
- [ ] Repository URL in `package.json`
- [ ] All tests passing
- [ ] README.md is comprehensive
- [ ] CHANGELOG.md is updated
- [ ] LICENSE file exists
- [ ] TypeScript compiles without errors
- [ ] Extension tested locally
- [ ] No sensitive data in package
- [ ] Version number updated

## Package Contents

The `.vsix` file includes:
- ✅ Compiled JavaScript (`out/extension.js`)
- ✅ Syntax highlighting grammar (`syntaxes/raven.tmLanguage.json`)
- ✅ Language configuration (`language-configuration.json`)
- ✅ README and CHANGELOG
- ✅ LICENSE
- ✅ Icon (when created)
- ❌ Source TypeScript files (excluded by .vscodeignore)
- ❌ node_modules (excluded by .vscodeignore)

## File Size

Target package size: **< 5MB**

Check package size:
```bash
ls -lh jounce-*.vsix
```

## Troubleshooting

### Error: "Missing repository field"
Add repository URL to `package.json`.

### Error: "Missing icon"
Create `icon.png` (128x128) or remove icon reference from `package.json`.

### Error: "Publisher not found"
Create a publisher account at https://marketplace.visualstudio.com/manage

### Large package size
Check `.vscodeignore` to exclude unnecessary files.

## Next Steps

After packaging:
1. Test the `.vsix` file locally
2. Share with beta testers
3. Gather feedback
4. Fix any issues
5. Publish to Marketplace!

---

**Happy Publishing! 🚀**
