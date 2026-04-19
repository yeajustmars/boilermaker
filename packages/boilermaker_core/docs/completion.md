# Completion

Boilermaker comes with the ability to generate command autocompletion for you. These are the `boil completion gen-*` commands, for instance `boil completion gen-bash`.

## Printing to the screen

This is the simplest approach and gives the user the most control of how they want to use the generated autocomplete code. It will simply print the completion code to the screen.

```bash
boil completion gen-bash
```

## Saving to file

If you want to save the autocomplete code to a file, pass the path in the `--file` option. For instance, if you want to `source` the file in your `.bashrc` file:

```bash
boil completion gen-bash --file ~/.config/boilermaker/completion.bash
```

Then in your `~/.bashrc` file, add the following:

```bash
BOILERMAKER_COMPLETION_FILE="~/.config/boilermaker/completion.bash"
if [ -f "$BOILERMAKER_COMPLETION_FILE" ]; then
  source "$BOILERMAKER_COMPLETION_FILE"
fi
```

And reload your environment:

```bash
source ~/.bashrc
```
