# galdr

Galdr is an execution environment for the [Rune programming language](https://rune-rs.github.io/) that provides an interpreter (for making executable Rune scripts) and a library for writing terminal apps.

## Installation

> TODO, below is just an example

On macOS, you can install `galdr` through Homebrew:

```
brew install galdr
```

## Example

Create a new Rune script and add the shebang to direct it to be run through **galdr**.

```rune
#!/usr/bin/env galdr

pub fn main() {
	dbg!("Hello from Galdr!");
}
```

Make the script executable with: `chmod +x hello.rn`. Run the script!

```
> ./scripts/hello.rn
"Hello from Galdr!"
```

## Initial plan [wip]

Galdr will provide an execution environment that simplifies using Rune for terminal apps (the typical use case for bash, for example). 

* Shell interaction (cd, pwd, stdin/out/err, glob, env vars)
* CLI logic (structured arg parsing, colored output, prompting for various data types)
* Structured logging (with multiple output formats such as pretty, json, etc)
* `fetch`: rune module with compatible types and functions to the js fetch library

## Long term ideas

* Possibly a web server?
