# Completion

Boilermaker comes with the ability to generate command autocompletion for you. These are the `boil completion generate SHELL` commands, for instance `boil completion generate bash`, `boil completion generate zsh`, etc.

## Printing to the screen

This is the simplest approach and gives the user the most control of how they want to use the generated autocomplete code. It will simply print the completion code to the screen.

```bash
boil completion generate bash

boil completion generate zsh
```

## Saving to file

If you want to save the autocomplete code to a file, pass the path in the `--file` option.

For instance, if you want to `source` the file in your `.bashrc` file:

```bash
boil completion generate bash --file ~/.config/boilermaker/completion.bash
```

Then in your `~/.bashrc` file, add the following:

```bash
BOILERMAKER_COMPLETION_FILE="${HOME}/.config/boilermaker/completion.bash"
if [ -f "$BOILERMAKER_COMPLETION_FILE" ]; then
  source "$BOILERMAKER_COMPLETION_FILE"
fi
```

And reload your environment.

```bash
source ~/.bashrc
```
