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

pub fn () {
	dbg!("Hello from Galdr!");
}
```

Make the script executable with: `chmod +x hello.rn`. Run the script!

```
> ./scripts/hello.rn
"Hello from Galdr!"
```

