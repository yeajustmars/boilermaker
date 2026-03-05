# Configuration

Configuration for Boilermaker is done is one of 4 places:

1. The `boilermaker.toml` file in the template directory, which controls the behavior of that specific template.
3. An imported file, which can be used to override any configuration options declared in the `boilermaker.toml` file.
2. The command line, when running the `boil new` command, which can be used to override any configuration options declared in files.
4. The global `~/.boilermaker/boilermaker.toml` file, which controls Boilermaker's behavior.

## Template Configuration

The `boilermaker.toml` at the root of the template repository is the baseline configuration for a template. It has the following structure:

### Basic Variables

```toml
[project]
VAR = VALUE
...

[variables]
VAR = VALUE
...
```

An example from the [official Hello World template](https://github.com/yeajustmars/boil-hello-world) is as follows:

```toml
[project]
name = "boil-hello-world"
description = "Simple Hello World with Boilermaker"
version = "0.1.0"
default_lang = "bash"
repository = "https://github.com/yeajustmars/boil-hello-world"
authors = ["yeajustmars"]
keywords = ["boilermaker", "example", "hello-world"]
website = "https://github.com/yeajustmars/boil-hello-world"
license = "MIT"

[variables]
welcome_message = "Hello, World!"
```

- You can have as many variables as you want in the `[variables]` section and what they are named is entirely up to you.
- However, the variables in the `[project]` section are reserved keys for Boilermaker's internal use.

It's important to note that any valid TOML is supported, including nested values. The following is perfectly valid:

```toml
[project]
name = "boil-hello-world"
description = "Simple Hello World with Boilermaker"
version = "0.1.0"
default_lang = "bash"
repository = "https://github.com/yeajustmars/boil-hello-world"
authors = ["yeajustmars"]
keywords = ["boilermaker", "example", "hello-world"]
website = "https://github.com/yeajustmars/boil-hello-world"
license = "MIT"

[variables]
welcome_message = "Hello, World!"
my_nested_var = {
    a = 1
    b = true
    c = ["x", "y", "z"]
}
```

The variables can then be used in your template files using standard minijinja syntax. For instance, given the example above, you could interpolate these variables as follows:

```html
<h1>{{ welcome_message }}</h1>
<!--  is "Hello, World!" -->

<p>My nested variable: {{ my_nested_var.c[0] }}</p>
<!-- is "x" -->
```

> _Remember: we're using HTML as the template language here but Boilermaker is language-agnostic. This same syntax would work in a Python file, a BASH script or a Dockerfile. Anything!_

### Variable Profiles

Variable profiles are a way to group variables together for different use cases. For instance, you might have a set of variables that are only relevant for a specific language or framework. You can declare these variable profiles in your `boilermaker.toml` file as follows:

```toml
[variables]
welcome_message = "Hello, World!"
other_var = "This is a variable that is always available."

[variables.profile.python]
welcome_message = "Override, Python World!"

[variables.profile.rust]
welcome_message = "Override, Rusty World!"
```

- When running `boil new`, you can specify which profile to use with the `--profile` (or just `-p`) flag. For instance, if you wanted to use the Python profile, you would run the following command:

```bash
boil new my-template --profile python
```

> _NOTE: variable profiles only work for the `[variables]` section of the `boilermaker.toml` file. The `[project]` section is not affected. This is to ensure that a single template has a single project configuration._


## Extra Configuration Files

Sometimes it can be useful, especially in larger templates, to split up your configuration into multiple files. While you can use variable profiles to accomplish a similar task, it might be cleaner to break up your configuration into separate files

## Command Line Configuration

Command line configuration is done by passing `--var` (or just `-v`) arguments to the `boil new` command. This allows you to override any configuration options declared in the `boilermaker.toml` file or imported files.

For instance, if you have this in your `boilermaker.toml` file:

```toml
[variables]
app_name = "my-app"
```

You can override the `app_name` variable by running the following command:

```bash
boil new my-template --var app_name=awesome-app
```

## Global System Configuration

> _TODO: Add docs_

