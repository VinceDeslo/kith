# Kith

This is a small Terminal UI I have been wanting for some time to quickly access various PostgreSQL databases across Teleport.
This comes from a frustration that `tsh`'s output for database instances is a table of text and not structured data (JSON, etc.).

The program pulls down the database instances your user has access to, parses it and offers prompts to navigate through your instances. 
The goal is enable you to easily see the details of an instance and to connect without needing to memorize user permissions or database names.

### How it works

The program's flow follows three main steps in a cyclical fashion:
1. Render the terminal UI based on data structure values.
2. Handle events generated by the user (Key pushes).
3. Update state of the data structures.

Due to the single objective of the program, this simple game loop is sufficient.

Once all connection values are filled out, the program spawns a new terminal with a Teleport database session and breaks itself.

### Feature Set 

- [x] Automatic login to Teleport based on provided environment variables.
- [x] Database instance search functionality via user input.
- [x] Database detail rendering.
- [x] Database connection prompts (with user selection, database name input, and confirmation).

### Releasing

Releases are handled via GitHub Actions leveraging [release-plz](https://release-plz.ieni.dev/docs).
Currently distribution is only done through [crates.io](https://crates.io/crates/kith), this is less overhead than building and zipping binaries for various architectures (this also assumes you have Rust and Cargo installed).

### Installing

```
cargo install kith
```

### Running

With the full command:

```
KITH_TSH_PROXY="<YOUR-PROXY-VALUE>" KITH_TSH_CLUSTER="<YOUR-CLUSTER-VALUE>" kith
```

You can also set it and forget it with a small shell alias. Here's an example for zsh:

```
echo 'alias kith="KITH_TSH_PROXY="<YOUR-PROXY-VALUE>" KITH_TSH_CLUSTER="<YOUR-CLUSTER-VALUE>" kith"' >> ~/.zshrc
source ~/.zshrc
```

Then run with:

```
kith
```

### Running locally

Clone the repo and fill out your Teleport values under `.env`. See `.example.env` for variable names.

```
make run
make info
make debug
```

### Uninstalling

```
cargo uninstall kith
```

### Notes

This small tool is barely an MVP: 
- Various UX bugs remain to be squashed (eg. crashes from faulty user input).
- Debug logging breaks the TUI.
- Only MacOS is supported due to an explicit usage of AppleScript to launch DB connections in a separate terminal window.
- Code has not been cleaned (`rustfmt` should probably be used eventually).

Once this tool is useable for my daily workflow, I will most likely stop developping it in order to move on to other
miscellaneous side projects. If you wish to add any functionality, feel free to fork the repo!

### Screenshots / Recordings

Here's an example of the TUI when using fake data locally:

![Demo Gif](https://github.com/VinceDeslo/kith/blob/main/demo/kith-local-demo.gif)
