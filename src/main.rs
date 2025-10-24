use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::time::Instant;
use jounce_compiler::{Compiler, deployer, BuildTarget}; // FIX: Corrected the import path
use jounce_compiler::watcher::{FileWatcher, WatchConfig, CompileStats};
use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;
use jounce_compiler::js_emitter::JSEmitter;

#[derive(ClapParser)]
#[command(name = "jnc", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Compiles a Jounce file
    Compile {
        path: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        minify: bool,
        #[arg(short, long)]
        profile: bool,
    },
    /// Creates a new Jounce project
    New {
        name: String,
    },
    /// Initialize a new Jounce project in the current directory
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Start a local development server
    Serve {
        #[arg(short, long, default_value = "8000")]
        port: u16,
        #[arg(long)]
        open: bool,
    },
    /// Diagnose common issues with your Jounce setup
    Doctor,
    /// Builds and deploys the project to a cloud provider
    Deploy {
        #[arg(long, default_value = "production")]
        env: String,
    },
    /// Watch files and auto-recompile on changes
    Watch {
        #[arg(default_value = "src")]
        path: PathBuf,
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,
        #[arg(short, long)]
        clear: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Start development server with HMR
    Dev {
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Run tests
    Test {
        #[arg(short, long)]
        watch: bool,
        #[arg(short, long)]
        verbose: bool,
        #[arg(short, long)]
        filter: Option<String>,
        #[arg(default_value = "tests")]
        path: PathBuf,
    },
    /// Format Jounce source files
    Fmt {
        #[arg(short, long)]
        check: bool,
        #[arg(short, long)]
        write: bool,
        path: Option<PathBuf>,
    },
    /// Lint Jounce source files
    Lint {
        #[arg(short, long)]
        fix: bool,
        path: Option<PathBuf>,
    },
    /// Build the project for production
    Build {
        #[arg(short, long)]
        release: bool,
    },
    /// Package manager commands
    Pkg {
        #[command(subcommand)]
        command: PkgCommands,
    },
}

#[derive(clap::Subcommand)]
enum PkgCommands {
    /// Initialize a new package manifest (jounce.toml)
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Install all dependencies
    Install,
    /// Add a dependency
    Add {
        name: String,
        #[arg(short, long)]
        version: Option<String>,
        #[arg(long)]
        dev: bool,
    },
    /// Remove a dependency
    Remove {
        name: String,
    },
    /// Update dependencies to latest compatible versions
    Update,
    /// Login to the package registry
    Login,
    /// Register a new account
    Register,
    /// Publish the current package to the registry
    Publish,
    /// Search for packages in the registry
    Search {
        query: String,
    },
    /// Display dependency tree
    Tree,
    /// Check for outdated dependencies
    Outdated,
    /// List all installed packages
    List,
    /// Show detailed information about a package
    Info {
        name: String,
    },
    /// Show build cache statistics
    Cache,
    /// Clear build cache
    Clean,
    /// Audit dependencies for security vulnerabilities
    Audit,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { path, output, minify, profile } => {
            use jounce_compiler::lexer::Lexer;
            use jounce_compiler::parser::Parser;
            use jounce_compiler::js_emitter::JSEmitter;
            use jounce_compiler::js_minifier::JSMinifier;

            let compile_start = Instant::now();

            println!("🔥 Compiling full-stack application: {}", path.display());
            if minify {
                println!("   🗜️  Minification: enabled");
            }
            if profile {
                println!("   📊 Profiling: enabled");
            }
            println!("   📦 Output: server.js + client.js + app.wasm\n");

            // Read source code
            let io_start = Instant::now();
            let source_code = match fs::read_to_string(&path) {
                Ok(code) => code,
                Err(e) => {
                    eprintln!("❌ Error reading file '{}': {}", path.display(), e);
                    return;
                }
            };
            let io_time = io_start.elapsed();

            // Parse the source
            println!("   Parsing...");
            let lex_start = Instant::now();
            let mut lexer = Lexer::new(source_code.clone());
            let lex_time = lex_start.elapsed();

            let parse_start = Instant::now();
            let mut parser = Parser::new(&mut lexer);
            let mut program = match parser.parse_program() {
                Ok(p) => {
                    println!("   ✓ Parsed {} statements", p.statements.len());
                    p
                }
                Err(e) => {
                    eprintln!("❌ Parsing failed: {:?}", e);
                    return;
                }
            };
            let parse_time = parse_start.elapsed();

            // Merge imported modules into the AST
            let module_start = Instant::now();
            use jounce_compiler::module_loader::ModuleLoader;
            let mut module_loader = ModuleLoader::new("aloha-shirts");
            if let Err(e) = module_loader.merge_imports(&mut program) {
                eprintln!("❌ Module import failed: {}", e);
                return;
            }
            let module_time = module_start.elapsed();

            // Generate JavaScript bundles
            println!("   Generating JavaScript bundles...");
            let codegen_start = Instant::now();
            let emitter = JSEmitter::new(&program);
            let mut server_js = emitter.generate_server_js();
            let mut client_js = emitter.generate_client_js();
            let codegen_time = codegen_start.elapsed();

            let stats = emitter.stats();
            println!("   ✓ Split: {} server, {} client, {} shared functions",
                stats.server_functions, stats.client_functions, stats.shared_functions);

            // Minify if requested
            let mut minify_time = std::time::Duration::ZERO;
            if minify {
                println!("   Minifying JavaScript...");
                let minify_start = Instant::now();
                let minifier = JSMinifier::new();

                let server_minified = minifier.minify(&server_js);
                let client_minified = minifier.minify(&client_js);

                let server_stats = minifier.stats(&server_js, &server_minified);
                let client_stats = minifier.stats(&client_js, &client_minified);

                println!("   ✓ server.js: {} → {} bytes (-{:.1}%)",
                    server_stats.original_size, server_stats.minified_size, server_stats.reduction_percent);
                println!("   ✓ client.js: {} → {} bytes (-{:.1}%)",
                    client_stats.original_size, client_stats.minified_size, client_stats.reduction_percent);

                server_js = server_minified;
                client_js = client_minified;
                minify_time = minify_start.elapsed();
            }

            // Compile to WASM
            println!("   Compiling to WebAssembly...");
            let wasm_start = Instant::now();
            let compiler = Compiler::new();
            let (wasm_bytes, css_output) = match compiler.compile_source_with_css(&source_code, BuildTarget::Client) {
                Ok((bytes, css)) => {
                    println!("   ✓ Generated WASM module ({} bytes)", bytes.len());
                    if !css.is_empty() {
                        println!("   ✓ Generated CSS output ({} bytes)", css.len());
                    }
                    (bytes, css)
                }
                Err(e) => {
                    eprintln!("\n❌ Compilation failed:\n");
                    let diagnostic_output = Compiler::display_error(&e, Some(&source_code), &path.to_string_lossy());
                    eprintln!("{}", diagnostic_output);
                    return;
                }
            };
            let wasm_time = wasm_start.elapsed();

            // Determine output directory
            let output_dir = output.unwrap_or_else(|| PathBuf::from("dist"));
            if let Err(e) = fs::create_dir_all(&output_dir) {
                eprintln!("❌ Failed to create output directory: {}", e);
                return;
            }

            // Write output files
            println!("\n   Writing output files...");
            let write_start = Instant::now();

            let server_path = output_dir.join("server.js");
            if let Err(e) = fs::write(&server_path, server_js) {
                eprintln!("❌ Failed to write server.js: {}", e);
                return;
            }
            println!("   ✓ {}", server_path.display());

            let client_path = output_dir.join("client.js");
            if let Err(e) = fs::write(&client_path, client_js) {
                eprintln!("❌ Failed to write client.js: {}", e);
                return;
            }
            println!("   ✓ {}", client_path.display());

            let wasm_path = output_dir.join("app.wasm");
            if let Err(e) = fs::write(&wasm_path, wasm_bytes) {
                eprintln!("❌ Failed to write app.wasm: {}", e);
                return;
            }
            println!("   ✓ {}", wasm_path.display());

            // Write CSS output (Phase 7.5)
            if !css_output.is_empty() {
                let css_path = output_dir.join("styles.css");
                if let Err(e) = fs::write(&css_path, css_output) {
                    eprintln!("❌ Failed to write styles.css: {}", e);
                    return;
                }
                println!("   ✓ {}", css_path.display());
            }

            // Create index.html
            let html_content = generate_index_html();
            let html_path = output_dir.join("index.html");
            if let Err(e) = fs::write(&html_path, html_content) {
                eprintln!("⚠️  Warning: Failed to write index.html: {}", e);
            } else {
                println!("   ✓ {}", html_path.display());
            }
            let write_time = write_start.elapsed();

            let total_time = compile_start.elapsed();

            // Display profiling report if requested
            if profile {
                println!("\n📊 Profiling Results");
                println!("====================");
                println!("  File I/O:      {:>8.2?}  ({:>5.1}%)", io_time, (io_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
                println!("  Lexing:        {:>8.2?}  ({:>5.1}%)", lex_time, (lex_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
                println!("  Parsing:       {:>8.2?}  ({:>5.1}%)", parse_time, (parse_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
                println!("  Modules:       {:>8.2?}  ({:>5.1}%)", module_time, (module_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
                println!("  Codegen:       {:>8.2?}  ({:>5.1}%)", codegen_time, (codegen_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
                if minify {
                    println!("  Minification:  {:>8.2?}  ({:>5.1}%)", minify_time, (minify_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
                }
                println!("  WASM:          {:>8.2?}  ({:>5.1}%)", wasm_time, (wasm_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
                println!("  Writing:       {:>8.2?}  ({:>5.1}%)", write_time, (write_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
                println!("  {}", "─".repeat(38));
                println!("  Total:         {:>8.2?}  ({:>5.0}%)", total_time, 100.0);
                println!();
            }

            println!("\n✨ Compilation complete! ({:.2?})", total_time);
            println!("   Run: cd {} && node server.js", output_dir.display());
        }
        Commands::New { name } => {
            // FIX: Added logic for creating a new project
            if let Err(e) = create_new_project(&name) {
                eprintln!("❌ Error creating new project: {}", e);
                process::exit(1);
            }
            println!("✅ Project '{}' created successfully! 🚀", name);
        }
        Commands::Init { path } => {
            println!("🚀 Initializing Jounce project...");
            if let Err(e) = init_project(&path) {
                eprintln!("❌ Initialization failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Serve { port, open } => {
            println!("🌐 Starting local development server on port {}...", port);
            if let Err(e) = serve_project(port, open) {
                eprintln!("❌ Server failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Doctor => {
            println!("🏥 Running Jounce diagnostics...\n");
            run_doctor();
        }
        Commands::Deploy { env } => {
            println!("🚀 Starting deployment to '{}'...", env);
            if let Err(e) = deployer::deploy_project() {
                eprintln!("❌ Deployment failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Watch { path, output, clear, verbose } => {
            println!("👀 Watching {} for changes...", path.display());
            if let Err(e) = watch_and_compile(path, output, clear, verbose) {
                eprintln!("❌ Watch failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Dev { port } => {
            println!("🚀 Starting development server on port {}...", port);
            if let Err(e) = start_dev_server(port) {
                eprintln!("❌ Dev server failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Test { watch, verbose, filter, path } => {
            if watch {
                println!("🧪 Running tests in watch mode...");
            } else {
                println!("🧪 Running tests...");
            }
            if let Err(e) = run_tests(path, watch, verbose, filter) {
                eprintln!("❌ Tests failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Fmt { check, write, path } => {
            let target = path.unwrap_or_else(|| PathBuf::from("src"));

            // Determine mode
            let mode = if check {
                FormatMode::Check
            } else if write {
                FormatMode::Write
            } else {
                FormatMode::Print
            };

            if let Err(e) = format_code(target, mode) {
                eprintln!("❌ Formatting failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Lint { fix, path } => {
            let target = path.unwrap_or_else(|| PathBuf::from("src"));
            if fix {
                println!("🔧 Linting and fixing {}...", target.display());
            } else {
                println!("🔍 Linting {}...", target.display());
            }
            if let Err(e) = lint_code(target, fix) {
                eprintln!("❌ Linting failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Build { release } => {
            if release {
                println!("📦 Building project (release mode)...");
            } else {
                println!("📦 Building project (debug mode)...");
            }
            if let Err(e) = build_project(release) {
                eprintln!("❌ Build failed: {}", e);
                process::exit(1);
            }
        }
        Commands::Pkg { command } => {
            use jounce_compiler::package_manager::PackageManager;

            match command {
                PkgCommands::Init { path } => {
                    let pkg_mgr = PackageManager::new(&path);
                    let project_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("my-package");

                    if let Err(e) = pkg_mgr.init(project_name, vec!["Developer <dev@example.com>".to_string()]) {
                        eprintln!("❌ Init failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Install => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.install() {
                        eprintln!("❌ Install failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Add { name, version, dev } => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    let version_req = version.as_deref().unwrap_or("^1.0.0");
                    if let Err(e) = pkg_mgr.add_dependency(&name, version_req, dev) {
                        eprintln!("❌ Add failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Remove { name } => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.remove_dependency(&name) {
                        eprintln!("❌ Remove failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Update => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.update() {
                        eprintln!("❌ Update failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Login => {
                    let mut pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.login() {
                        eprintln!("❌ Login failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Register => {
                    let mut pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.register() {
                        eprintln!("❌ Registration failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Publish => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.publish() {
                        eprintln!("❌ Publish failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Search { query } => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.search(&query) {
                        eprintln!("❌ Search failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Tree => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.tree() {
                        eprintln!("❌ Tree failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Outdated => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.outdated() {
                        eprintln!("❌ Outdated check failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::List => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.list() {
                        eprintln!("❌ List failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Info { name } => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.info(&name) {
                        eprintln!("❌ Info failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Cache => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.cache_stats() {
                        eprintln!("❌ Cache stats failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Clean => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.clean_cache() {
                        eprintln!("❌ Cache clean failed: {}", e);
                        process::exit(1);
                    }
                }
                PkgCommands::Audit => {
                    let pkg_mgr = PackageManager::new(&PathBuf::from("."));
                    if let Err(e) = pkg_mgr.audit() {
                        eprintln!("❌ Audit failed: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }
}

// The create_new_project function is unchanged
fn create_new_project(name: &str) -> std::io::Result<()> {
    let root = PathBuf::from(name);
    if root.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "A directory with this name already exists.",
        ));
    }

    fs::create_dir_all(root.join("src/components"))?;
    fs::create_dir_all(root.join("src/server"))?;

    fs::write(
        root.join("jounce.toml"),
        format!(
            "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n",
            name
        ),
    )?;

    fs::write(
        root.join(".gitignore"),
        "/dist\n/target\n",
    )?;

    fs::write(
        root.join("src/main.jnc"),
        format!("// Welcome to Jounce!\n\ncomponent App() {{\n    return <h1>\"Hello, {}!\"</h1>;\n}}\n", name),
    )?;
    
    fs::write(
        root.join("src/types.jnc"),
        "// Define your shared data structures here.\n",
    )?;

    Ok(())
}

fn watch_and_compile(
    path: PathBuf,
    output: PathBuf,
    clear: bool,
    verbose: bool
) -> Result<(), String> {
    // Create watch configuration
    let config = WatchConfig {
        path: path.clone(),
        output_dir: output.clone(),
        debounce_ms: 150,
        clear_console: clear,
        verbose,
    };

    // Create file watcher
    let mut watcher = FileWatcher::new(config)?;
    watcher.watch()?;

    // Initial compilation
    println!("🔥 Jounce Watch Mode");
    println!("   Path: {}", path.display());
    println!("   Output: {}", output.display());
    println!();

    let compile_result = compile_file(&path, &output, verbose);
    display_compile_result(&compile_result, clear);

    println!("\n👀 Watching for changes... (Ctrl+C to stop)\n");

    // Watch loop
    loop {
        // Wait for file change (with debouncing)
        if let Some(changed_path) = watcher.wait_for_change() {
            if verbose {
                println!("[{}] File changed", changed_path.display());
            }

            // Clear console if requested
            if clear {
                print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes to clear screen
            }

            // Determine what file to compile
            let target_path = if changed_path.is_file() {
                changed_path
            } else {
                path.clone()
            };

            println!("⚡ Recompiling...");
            let compile_result = compile_file(&target_path, &output, verbose);
            display_compile_result(&compile_result, clear);

            println!("\n👀 Watching for changes... (Ctrl+C to stop)\n");
        }
    }
}

fn compile_file(path: &PathBuf, output_dir: &PathBuf, verbose: bool) -> CompileStats {
    let start = Instant::now();
    let mut stats = CompileStats::default();

    // Read source file
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("✗ Failed to read file: {}", e);
            stats.success = false;
            stats.duration_ms = start.elapsed().as_millis() as u64;
            return stats;
        }
    };

    // Compile
    let mut lexer = Lexer::new(source.clone());
    let mut parser = Parser::new(&mut lexer);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("✗ Parser error: {:?}", e);
            stats.success = false;
            stats.duration_ms = start.elapsed().as_millis() as u64;
            return stats;
        }
    };

    // Generate JavaScript
    let emitter = JSEmitter::new(&program);
    let server_js = emitter.generate_server_js();
    let client_js = emitter.generate_client_js();

    // Create output directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("✗ Failed to create output directory: {}", e);
        stats.success = false;
        stats.duration_ms = start.elapsed().as_millis() as u64;
        return stats;
    }

    // Write output files
    let server_path = output_dir.join("server.js");
    let client_path = output_dir.join("client.js");

    if let Err(e) = fs::write(&server_path, server_js) {
        eprintln!("✗ Failed to write server.js: {}", e);
        stats.success = false;
        stats.duration_ms = start.elapsed().as_millis() as u64;
        return stats;
    }

    if let Err(e) = fs::write(&client_path, client_js) {
        eprintln!("✗ Failed to write client.js: {}", e);
        stats.success = false;
        stats.duration_ms = start.elapsed().as_millis() as u64;
        return stats;
    }

    stats.compiled = 1;
    stats.success = true;
    stats.duration_ms = start.elapsed().as_millis() as u64;

    if verbose {
        println!("  → {}", server_path.display());
        println!("  → {}", client_path.display());
    }

    stats
}

fn display_compile_result(stats: &CompileStats, _clear: bool) {
    if stats.success {
        println!("✓ Compiled successfully ({}ms)", stats.duration_ms);
        if stats.compiled > 0 || stats.cached > 0 {
            println!(
                "  Files: {} compiled{}",
                stats.compiled,
                if stats.cached > 0 {
                    format!(", {} cached", stats.cached)
                } else {
                    String::new()
                }
            );
        }
    } else {
        println!("✗ Compilation failed ({}ms)", stats.duration_ms);
    }
}

fn start_dev_server(port: u16) -> std::io::Result<()> {
    println!("✅ Development server starting...");
    println!("   📦 HTTP Server: http://localhost:{}", port);
    println!("   🔥 HMR Server: ws://localhost:3002/hmr");
    println!("   👀 File watcher: Active\n");

    // Start file watcher in background
    let watch_thread = std::thread::spawn(|| {
        let _ = watch_and_compile(
            PathBuf::from("src"),
            PathBuf::from("dist"),
            false,
            false
        );
    });

    // Start HMR server
    let hmr_thread = std::thread::spawn(|| {
        let _ = std::process::Command::new("node")
            .arg("scripts/hmr-server.js")
            .spawn();
    });

    // Start HTTP server
    println!("🌐 Starting HTTP server...");
    let http_result = std::process::Command::new("python3")
        .arg("serve.py")
        .spawn();

    if let Ok(mut child) = http_result {
        println!("✨ Dev server running! Press Ctrl+C to stop.\n");
        let _ = child.wait();
    }

    let _ = watch_thread.join();
    let _ = hmr_thread.join();

    Ok(())
}

fn run_tests(
    test_path: PathBuf,
    watch_mode: bool,
    verbose: bool,
    filter: Option<String>,
) -> std::io::Result<()> {
    use jounce_compiler::test_framework::{TestDiscovery, TestRunner, generate_assertion_library};

    // Check if test directory exists
    if !test_path.exists() {
        println!("ℹ️  No tests directory found. Creating {}...", test_path.display());
        fs::create_dir_all(&test_path)?;
        fs::write(
            test_path.join("example_test.jnc"),
            r#"// Example test file
// Functions starting with "test_" are automatically discovered and run

fn add(a: int, b: int) -> int {
    return a + b;
}

fn test_addition() {
    let result = add(2, 3);
    assert_eq(result, 5, "2 + 3 should equal 5");
}

fn test_subtraction() {
    let result = 10 - 3;
    assert_eq(result, 7, "10 - 3 should equal 7");
}
"#
        )?;
        println!("✅ Created {}/example_test.jnc", test_path.display());
        println!("\n💡 Run 'jnc test' again to execute tests");
        return Ok(());
    }

    // Discover tests
    let discovery = TestDiscovery::new();
    let suite = match discovery.discover_tests(&test_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Test discovery failed: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Test discovery failed: {}", e)
            ));
        }
    };

    // Filter tests if requested
    let filtered_tests: Vec<_> = if let Some(ref pattern) = filter {
        suite.tests.iter()
            .filter(|t| t.name.contains(pattern))
            .cloned()
            .collect()
    } else {
        suite.tests.clone()
    };

    if filtered_tests.is_empty() {
        println!("⚠️  No tests found matching filter criteria");
        return Ok(());
    }

    // Create filtered suite
    let filtered_suite = jounce_compiler::test_framework::TestSuite {
        tests: filtered_tests,
        total_files: suite.total_files,
    };

    // Print test discovery summary
    let runner = TestRunner::new(filtered_suite);
    runner.print_summary();

    if verbose {
        println!();
        for test in &runner.suite.tests {
            let async_marker = if test.is_async { " (async)" } else { "" };
            println!("  • {}{}", test.name, async_marker);
        }
    }

    // Generate test runner code
    println!("\n🧪 Executing tests...\n");

    // Compile all test files to JavaScript
    let temp_dir = PathBuf::from("dist");
    fs::create_dir_all(&temp_dir)?;

    // Combine stdlib modules and test source files into one
    let mut combined_source = String::new();

    // Include stdlib modules needed for tests
    use jounce_compiler::stdlib::{
        json::JSON_DEFINITION,
        time::TIME_DEFINITION,
        crypto::CRYPTO_DEFINITION,
        // fs::FS_DEFINITION,  // Temporarily disabled pending parser enhancements
    };

    combined_source.push_str(JSON_DEFINITION);
    combined_source.push_str("\n\n");
    combined_source.push_str(TIME_DEFINITION);
    combined_source.push_str("\n\n");
    combined_source.push_str(CRYPTO_DEFINITION);
    combined_source.push_str("\n\n");
    // combined_source.push_str(FS_DEFINITION);
    // combined_source.push_str("\n\n");

    // Add test source files
    for test in &runner.suite.tests {
        if let Ok(test_source) = fs::read_to_string(&test.file_path) {
            combined_source.push_str(&test_source);
            combined_source.push_str("\n\n");
        }
    }

    // Save combined source for debugging if verbose
    if verbose {
        fs::write(temp_dir.join("combined_source.jnc"), &combined_source)?;
        println!("📝 Combined source saved to dist/combined_source.jnc");
    }

    // Parse and compile combined Jounce code to JavaScript
    let mut lexer = Lexer::new(combined_source.clone());
    let mut parser = Parser::new(&mut lexer);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Failed to parse test files: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Parser error: {:?}", e)
            ));
        }
    };

    // Generate JavaScript (use client-side generation to avoid server boilerplate)
    let emitter = JSEmitter::new(&program);
    let client_js = emitter.generate_client_js();

    if verbose {
        // Save unfiltered version for debugging
        fs::write(temp_dir.join("test_runner_unfiltered.js"), &client_js)?;
        println!("📝 Unfiltered JavaScript saved to dist/test_runner_unfiltered.js");
    }

    // Strip out import statements and RPC client code (not needed for tests)
    let lines: Vec<&str> = client_js.lines().collect();
    let mut filtered_lines = Vec::new();
    let mut skip_until_blank = false;

    for line in lines {
        // Skip import statements
        if line.starts_with("import {") || line.starts_with("import ") && line.contains(" from ") {
            continue;
        }
        // Skip RPC client setup comments and code
        if line.contains("// RPC Client Setup") ||
           line.contains("// Auto-generated RPC client stubs") ||
           line.contains("const client = new RPCClient") {
            continue;
        }
        // Skip browser-only sections (window/document event listeners and their bodies)
        if line.contains("window.addEventListener") ||
           line.contains("document.addEventListener") {
            skip_until_blank = true;
            continue;
        }
        if skip_until_blank {
            if line.trim().is_empty() {
                skip_until_blank = false;
            }
            continue;
        }
        // Skip sourcemap comments
        if line.contains("//# sourceMappingURL") {
            continue;
        }
        // Skip UI initialization sections
        if line.contains("// UI Components") || line.contains("// Initialize application") {
            continue;
        }

        // Remove "export " prefix from function declarations
        let cleaned_line = if line.starts_with("export function ") {
            &line[7..]  // Remove "export " prefix
        } else if line.starts_with("export async function ") {
            &line[7..]  // Remove "export " prefix
        } else {
            line
        };

        filtered_lines.push(cleaned_line);
    }
    let test_functions_js = filtered_lines.join("\n");

    // Build final test runner
    let mut test_js = String::new();
    test_js.push_str(&generate_assertion_library());
    test_js.push_str("\n\n");
    test_js.push_str(&test_functions_js);
    test_js.push_str("\n\n");
    test_js.push_str(&runner.generate_runner_code_js());

    // Write executable test file
    let test_runner_path = temp_dir.join("test_runner.js");
    fs::write(&test_runner_path, test_js)?;

    if verbose {
        println!("📝 Test runner generated at {}", test_runner_path.display());
    }

    // Execute tests with Node.js
    let output = process::Command::new("node")
        .arg(&test_runner_path)
        .output();

    match output {
        Ok(result) => {
            // Print stdout
            if !result.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&result.stdout));
            }

            // Print stderr
            if !result.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&result.stderr));
            }

            // Check exit code
            if !result.status.success() {
                println!("\n❌ Some tests failed");
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to execute tests: {}", e);
            eprintln!("\n💡 Make sure Node.js is installed and available in your PATH");
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Node.js not found"
            ));
        }
    }

    // Watch mode
    if watch_mode {
        println!("\n👀 Watching for changes... (Ctrl+C to stop)");
        if let Err(e) = watch_and_compile(
            test_path,
            PathBuf::from("dist"),
            false,
            verbose
        ) {
            eprintln!("Watch failed: {}", e);
        }
    }

    Ok(())
}

/// Formatting mode for the format command
#[derive(Debug, Clone, Copy, PartialEq)]
enum FormatMode {
    /// Check if files need formatting (exit with error if they do)
    Check,
    /// Format files and write changes to disk
    Write,
    /// Print formatted output to stdout (default)
    Print,
}

/// Result of formatting a file
enum FormatResult {
    Changed,
    Unchanged,
}

fn format_code(path: PathBuf, mode: FormatMode) -> std::io::Result<()> {
    use jounce_compiler::formatter::{Formatter, FormatterConfig};
    use jounce_compiler::lexer::Lexer;
    use jounce_compiler::parser::Parser;
    let mut formatted_count = 0;
    let mut error_count = 0;
    let mut total_count = 0;

    // Print mode header
    match mode {
        FormatMode::Check => println!("🔍 Checking formatting for {}...", path.display()),
        FormatMode::Write => println!("✨ Formatting {}...", path.display()),
        FormatMode::Print => {}, // No header for print mode
    }

    if path.is_file() {
        total_count += 1;
        match format_file(&path, mode) {
            Ok(FormatResult::Changed) => formatted_count += 1,
            Ok(FormatResult::Unchanged) => {},
            Err(_) => error_count += 1,
        }
    } else if path.is_dir() {
        // Recursively walk directory
        visit_dirs(&path, &mut |entry_path: &PathBuf| {
            if entry_path.extension().map_or(false, |ext| ext == "jnc") {
                total_count += 1;
                match format_file(entry_path, mode) {
                    Ok(FormatResult::Changed) => formatted_count += 1,
                    Ok(FormatResult::Unchanged) => {},
                    Err(_) => error_count += 1,
                }
            }
        })?;
    } else {
        eprintln!("❌ Path not found: {}", path.display());
        process::exit(1);
    }

    // Print summary
    match mode {
        FormatMode::Check => {
            if formatted_count > 0 {
                eprintln!("⚠️  {} file(s) need formatting", formatted_count);
                process::exit(1);
            } else if total_count > 0 {
                println!("✅ All {} file(s) are properly formatted", total_count);
            }
        }
        FormatMode::Write => {
            if formatted_count > 0 {
                println!("✅ Formatted {} file(s)", formatted_count);
            } else if total_count > 0 {
                println!("✅ All {} file(s) already properly formatted", total_count);
            }
            if error_count > 0 {
                eprintln!("⚠️  {} file(s) had errors", error_count);
            }
        }
        FormatMode::Print => {
            // No summary for print mode
        }
    }

    Ok(())
}

fn format_file(path: &PathBuf, mode: FormatMode) -> std::io::Result<FormatResult> {
    use jounce_compiler::formatter::{Formatter, FormatterConfig};
    use jounce_compiler::lexer::Lexer;
    use jounce_compiler::parser::Parser;

    let content = fs::read_to_string(path)?;

    // Parse the file
    let mut lexer = Lexer::new(content.clone());
    let mut parser = Parser::new(&mut lexer);
    let ast = match parser.parse_program() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("  ❌ Parse error in {}: {:?}", path.display(), e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Parse error: {:?}", e),
            ));
        }
    };

    // Format the AST
    let config = FormatterConfig::default();
    let mut formatter = Formatter::with_config(config);
    let formatted = formatter.format_program(&ast);

    // Check if content changed
    if content.trim() == formatted.trim() {
        return Ok(FormatResult::Unchanged);
    }

    // Handle based on mode
    match mode {
        FormatMode::Check => {
            println!("  ⚠️  {} needs formatting", path.display());
            Ok(FormatResult::Changed)
        }
        FormatMode::Write => {
            fs::write(path, &formatted)?;
            println!("  ✨ Formatted {}", path.display());
            Ok(FormatResult::Changed)
        }
        FormatMode::Print => {
            // Print formatted output to stdout
            print!("{}", formatted);
            Ok(FormatResult::Changed)
        }
    }
}

/// Visit all files in a directory recursively
fn visit_dirs(dir: &PathBuf, cb: &mut dyn FnMut(&PathBuf)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&path);
            }
        }
    }
    Ok(())
}

fn lint_code(path: PathBuf, fix: bool) -> std::io::Result<()> {
    let mut issues = 0;
    let mut fixed = 0;

    if path.is_file() {
        let result = lint_file(&path, fix)?;
        issues += result.0;
        fixed += result.1;
    } else {
        for entry in fs::read_dir(path)?.flatten() {
            let entry_path = entry.path();
            if entry_path.extension().map_or(false, |ext| ext == "jnc") {
                let result = lint_file(&entry_path, fix)?;
                issues += result.0;
                fixed += result.1;
            }
        }
    }

    if fix {
        println!("✅ Fixed {} issue(s)", fixed);
    }

    if issues > 0 {
        println!("⚠️  {} issue(s) found", issues);
        if !fix {
            println!("💡 Run with --fix to automatically fix issues");
        }
    } else {
        println!("✅ No issues found");
    }

    Ok(())
}

fn lint_file(path: &PathBuf, fix: bool) -> std::io::Result<(usize, usize)> {
    let content = fs::read_to_string(path)?;
    let mut issues = 0;
    let mut fixed = 0;

    // Check for common issues
    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;

        // Check trailing whitespace
        if line.ends_with(' ') || line.ends_with('\t') {
            issues += 1;
            println!("  ⚠️  {}:{} - Trailing whitespace", path.display(), line_num);
        }

        // Check line length
        if line.len() > 100 {
            issues += 1;
            println!("  ⚠️  {}:{} - Line too long ({} > 100)", path.display(), line_num, line.len());
        }
    }

    if fix && issues > 0 {
        // Remove trailing whitespace
        let fixed_content: String = content.lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path, fixed_content)?;
        fixed = issues;
    }

    Ok((issues, fixed))
}

fn build_project(release: bool) -> std::io::Result<()> {
    let dist_dir = PathBuf::from("dist");
    fs::create_dir_all(&dist_dir)?;

    println!("📦 Building all components...\n");

    let src_dir = PathBuf::from("src");
    let mut compiled = 0;
    let mut errors = 0;

    for entry in fs::read_dir(src_dir)?.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "jnc") {
            println!("  Compiling {}...", path.file_name().unwrap().to_string_lossy());

            if let Ok(source) = fs::read_to_string(&path) {
                let compiler = Compiler::new();
                let target = if release { BuildTarget::Client } else { BuildTarget::Client };

                match compiler.compile_source(&source, target) {
                    Ok(wasm_bytes) => {
                        let output_name = path.file_stem().unwrap().to_string_lossy();
                        let output_path = dist_dir.join(format!("{}.wasm", output_name));
                        fs::write(&output_path, wasm_bytes)?;
                        compiled += 1;
                        println!("    ✅ → {}", output_path.display());
                    }
                    Err(e) => {
                        errors += 1;
                        println!("    ❌ Error: {}", e);
                    }
                }
            }
        }
    }

    println!("\n📊 Build complete:");
    println!("   ✅ Compiled: {} file(s)", compiled);
    if errors > 0 {
        println!("   ❌ Errors: {} file(s)", errors);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Build failed with errors"
        ));
    }

    println!("\n✨ Build artifacts in dist/");

    Ok(())
}

// New CLI commands

fn init_project(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use jounce_compiler::package_manager::PackageManager;

    let pkg_mgr = PackageManager::new(path);
    let project_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-package");

    pkg_mgr.init(project_name, vec!["Developer <dev@example.com>".to_string()])?;

    println!("✅ Initialized Jounce project in {}", path.display());
    println!("   Created jounce.toml");
    println!("\n💡 Next steps:");
    println!("   1. Edit jounce.toml to add package metadata");
    println!("   2. Run 'jnc build' to compile your project");
    println!("   3. Run 'jnc serve' to start a local development server");

    Ok(())
}

fn serve_project(port: u16, open: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ Starting local development server...");
    println!("   📂 Serving from: ./dist");
    println!("   🌐 URL: http://localhost:{}", port);

    // Check if dist directory exists
    let dist_dir = PathBuf::from("dist");
    if !dist_dir.exists() {
        println!("\n⚠️  dist/ directory not found. Building project first...\n");
        build_project(true)?;
    }

    if open {
        // Open browser
        let url = format!("http://localhost:{}", port);
        #[cfg(target_os = "macos")]
        let _ = process::Command::new("open").arg(&url).spawn();
        #[cfg(target_os = "linux")]
        let _ = process::Command::new("xdg-open").arg(&url).spawn();
        #[cfg(target_os = "windows")]
        let _ = process::Command::new("cmd").arg("/C").arg("start").arg(&url).spawn();
    }

    // Start simple HTTP server
    println!("\n✨ Server running! Press Ctrl+C to stop.\n");

    let result = process::Command::new("python3")
        .arg("-m")
        .arg("http.server")
        .arg(port.to_string())
        .arg("--directory")
        .arg("dist")
        .spawn();

    if let Ok(mut child) = result {
        let _ = child.wait();
    } else {
        return Err("Failed to start HTTP server. Make sure python3 is installed.".into());
    }

    Ok(())
}

fn generate_index_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Jounce App</title>
    <link rel="stylesheet" href="styles.css">
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            margin: 0;
            padding: 20px;
            background: #f5f5f5;
        }
        #app {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
    </style>
</head>
<body>
    <div id="app">
        <h1>Loading Jounce App...</h1>
    </div>
    <script type="module" src="client.js"></script>
</body>
</html>"#.to_string()
}

fn run_doctor() {
    println!("🏥 Jounce Doctor - Checking your setup...\n");

    let mut issues = 0;
    let mut warnings = 0;

    // Check Rust installation
    print!("  Checking Rust... ");
    if let Ok(output) = process::Command::new("rustc").arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✅ {}", version.trim());
        } else {
            println!("❌ FAILED");
            issues += 1;
        }
    } else {
        println!("❌ NOT FOUND");
        issues += 1;
    }

    // Check Cargo
    print!("  Checking Cargo... ");
    if let Ok(output) = process::Command::new("cargo").arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✅ {}", version.trim());
        } else {
            println!("❌ FAILED");
            issues += 1;
        }
    } else {
        println!("❌ NOT FOUND");
        issues += 1;
    }

    // Check Node.js (optional for HMR)
    print!("  Checking Node.js... ");
    if let Ok(output) = process::Command::new("node").arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✅ {}", version.trim());
        } else {
            println!("⚠️  FAILED (optional)");
            warnings += 1;
        }
    } else {
        println!("⚠️  NOT FOUND (optional - needed for HMR)");
        warnings += 1;
    }

    // Check Python (optional for dev server)
    print!("  Checking Python... ");
    if let Ok(output) = process::Command::new("python3").arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✅ {}", version.trim());
        } else {
            println!("⚠️  FAILED (optional)");
            warnings += 1;
        }
    } else {
        println!("⚠️  NOT FOUND (optional - needed for 'jnc serve')");
        warnings += 1;
    }

    // Check project structure
    print!("  Checking project structure... ");
    if PathBuf::from("jounce.toml").exists() {
        println!("✅ jounce.toml found");
    } else {
        println!("⚠️  No jounce.toml (run 'jnc init' to create one)");
        warnings += 1;
    }

    print!("  Checking src/ directory... ");
    if PathBuf::from("src").exists() {
        println!("✅ src/ directory exists");
    } else {
        println!("⚠️  No src/ directory");
        warnings += 1;
    }

    // Summary
    println!("\n📊 Summary:");
    if issues == 0 && warnings == 0 {
        println!("   ✅ All checks passed! Your Jounce setup looks good.");
    } else {
        if issues > 0 {
            println!("   ❌ {} critical issue(s) found", issues);
        }
        if warnings > 0 {
            println!("   ⚠️  {} warning(s)", warnings);
        }
    }

    if issues > 0 {
        println!("\n💡 Recommendations:");
        println!("   - Install Rust from: https://rustup.rs/");
        println!("   - Rust and Cargo are required for Jounce to work");
    }

    if warnings > 0 {
        println!("\n💡 Optional improvements:");
        println!("   - Install Node.js for HMR support: https://nodejs.org/");
        println!("   - Install Python for 'jnc serve' command");
        println!("   - Run 'jnc init' to create a new project");
    }
}