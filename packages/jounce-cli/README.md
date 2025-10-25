# jounce-cli

CLI utilities for Jounce applications - argument parsing, commands, colors, progress bars, and tables.

## Features

- Argument parsing (--flags, -short, commands)
- Command hierarchy with subcommands
- Colored output
- Progress bars
- Table formatting
- Interactive prompts

## Quick Start

```jounce
use jounce_cli::{CliApp, CliCommand, CliArgs};

let app = CliApp::new("myapp", "1.0.0")
    .add_command(
        CliCommand::new("build", "Build project")
            .with_handler(|args| println("Building..."))
    );

app.run(argv);
```

## License

MIT
