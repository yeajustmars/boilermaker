# Boilermaker

A language-agnostic project template system.

> NOTE: This is a draft for version `0.1.0-alpha1` and is highly likely to change!

## Index

1. [Structure](#structure)
2. [Templates](#templates)
3. [Configuration](#configuration)
4. [Variables](#variables)
5. [Development](#dev)

Boilermaker is a language-agnostic project template system. It was primarily designed
for programming projects but can be used for pretty much anything. The system provides some
basic rules and layout for [structure](#structure), [variable interpolation](#variables) and
[configuration](#configuration). Past this, it does little more than apply these rules to
some text files (the [`template`](#templates)), producing a new `project` ready to use.

It's written in Rust so it's pretty fast, and it uses a `Jinja2`-compatible template engine for
rendering the text files. Configuration is done in TOML or YAML.

> _TODO_: Allow YAML for Configuration files.

A `Boilermaker Template` (capital T) is comprised of 3 main components:

- [structure](#structure): directories, text files, etc, known as the "template" (lowercase 't')
- [configuration](#configuration): settings that control how the template is processed
- [variables](#variables): key-value pairs that are interpolated into the text files. These can
    also be provided at runtime via command line or global config.

<a name="structure"></a>
# Structure

> _You can see a basic example in this repo at `examples/hello-world`._

The structure of a Boilermaker Template is pretty straight forward. You have the  following as a
directory that can be cloned and then processed:

```
TEMPLATE_NAME/
    boilermaker.toml
    LANGUAGE_1/
        ... file or directory tree of files ...
    LANGUAGE_N/
        ... file or directory tree of files ...
```

That's about it!

The `boilermaker.toml` file is for project configuration and variables that will be
interpolated into the text files under the `template/LANGUAGE` directory. What exists under the
template directory is completely up to you. By default, all text files will be processed through
the rendering engine, executing all logic and replacing all variable placeholders with their values.

## A note on the `LANGUAGE` subdirectory

As you can see above, the format for a Boilermaker Template is `TEMPLATE_NAME/LANGUAGE`. This one
restriction of requiring this directory means that you can have implementations of your template
in multiple langauges. This directory must exist and it must be lowercase so that the system
knows how to find the template files. Regardless of language, a single `boilermaker.toml`
file will work for all language implementations.

<a name="templates"></a>
# Templates

The `LANGUAGE/` directory can contain anything you wish. However, any text files (by default)
will be processed with a `Jinja2`-compatible template engine. This allows you to put logic
and variables in your text files to be rendered to the final Project.

<a name="configuration"></a>
# Configuration

Configuration is done at one of 3 levels, depending on what it is you want to configure.

## Global Configuration

For global configuration of Boilermaker itself, you provide key-value pairs in
`~/.config/boilermaker/boilermaker.toml`. This file _does not_ affect individual Templates but
rather controls configuration for the Boilermaker runtime.

## Template configuration

For a Boilermaker Template, a `boilermaker.toml` file exists in the root directory for that
Template. This file only affects the text files under the root directory.

## Runtime Configuration

All declared variables are available to pass as BASH-style options when creating a project from
a Template. For example, if you have a variable `project_name` declared in your Template's
`boilermaker.toml`, you can override its value at runtime like so:

```bash
boilermaker new TEMPLATE --var some_var="Some Value" --var another_var=123
```

> TODO: decide on command line-level options for overriding global/default config.

## Default Configuration

If neither the global nor runtime configuration options are provided, a set of defaults is applied.
They are as follows:

> TODO: document default config

> TODO: add YAML as an option for boilermaker.toml

<a name="variables"></a>
# Variables

Variables for a Template live in the Template's local `boilermaker.toml` file, located in the root
of the Template directory. Inside this file, you add key-values pairs to the
`[variables]` section like so:

```toml
...

[variables]
a = "1"
b = "2"
...
```

<a name="dev"></a>
# Development

> TODO: document dev once a final implementation is decicded.
