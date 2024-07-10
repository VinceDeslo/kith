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

Due to single objective of the program, this simple game loop is sufficient.

Once all connection values are filled out, the program spawns a new terminal with a Teleport database session and breaks itself.

### Feature Set 

- [x] Automatic login to Teleport based on provided environment variables.
- [x] Database instance search functionality via user input.
- [x] Database detail rendering.
- [x] Database connection prompts (with user selection, database name input, and confirmation).
- [ ] TBD...

### Releasing

TBC...

### Installing

TBC...

### Running

Fill out your Teleport values under `.env`. See `.example.env` for variable names.

```
make run
make info
make debug
```

### Uninstalling

TBC...

### Notes

This small tool is currently a WIP. Various UX bugs remain to be squashed (eg. crashes from faulty user input).
Once this tool is useable for my daily workflow, I will most likely stop developping it in order to move on to other
miscellaneous side projects.

### Screenshots / Recordings

TBC... (This will require some test data, and I may be too lazy to mock such a thing)
