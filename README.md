# Boilermaker

> DRAFT! This is a draft for version `0.1.0-alpha1` and is highly likely to change!

Boilermaker is a project template system, designed primarily for programming projects but
it can be used for pretty much anything as it's language-agnostic. The system provides some 
basic rules and layout for [structure](#structure), [variable interpolation](#variables) and 
[configuration](#configuration). Past this, it does little more than apply these rules to 
some text files (the [`template`](#templates)), producing a new `project` ready to use. 

It is written in the Rust programming language, which makes it pretty fast. However, it is not
intenended to only be used for Rust projects, regardless of how it started out.

A `Boilermaker Template`, as a noun, not the system itself, is comprised of 3 main components:

- structure: directories, text files, etc. Known as the "template" (lowercase 't') 
- configuration
- variables

All three of these together comprise a `Boilermaker Template` (capital `T`).

## Index 

1. [Structure](#structure)
2. [Templates](#templates)
3. [Configuration](#configuration)
4. [Variables](#variables)
5. [Development](#dev)

<a name="structure"></a>
# Structure 

> _You can see a basic example in this repo at `examples/hello-world`.

The structure of a Boilermaker Template is pretty straight forward. You have the  following as a 
directory that can be cloned and then processed:

```
TEMPLATE_NAME/
    boilermaker.toml
    template/
        ... file or full directory tree of files ...
```

That's it! The `boilermaker.toml` file is for project configuration and variables that will be 
interpolated into the text files under the `template/` directory. What exists under the template 
directory is completely up to you. By default, all text files will be processed through the 
rendering engine, executing all logic and replacing all variable placeholders with their values.

<a name="templates"></a>
# Templates

The `template/` directory can contain anything you wish. However, any text files (by default) 
will be processed with a `Jinja2`-compatible template engine. This allows you to put some basic 
logic and variables in your text files to be rendered to the final Project.

<a name="configuration"></a>
# Configuration

Configuration is done at one of 3 levels, depending on what it is you want to configure. 

## The Template

For a Boilermaker Template, a `boilermaker.toml` file exists in the root directory for that 
Template. This file only affects the text files under the root directory.

## Global Configuration

For global configuration of Boilermaker itself, you provide key-value pairs in 
`~/.config/boilermaker/boilermaker.toml`. This file _does not_ affect individual Templates but 
rather controls configuration for the Boilermaker runtime.

## Inline Configuration

Some options are available to pass as BASH-style options to the `boil` command.

> TODO: decide on command line-level options for overriding global/default config.

## Default Configuration

If neither the global nor inline configuration options are provided, a set of defaults is applied. 
They are as follows:

> TODO: document default config

> TODO: add YAML as an option for boilermaker.toml

<a name="variables"></a>
# Variables 

Variables for a Template live in the Template's local `boilermaker.toml` file, located in the root 
of the Template directory. Inside this file, you add key-values pairs to the 
`[boilermaker.variables]` section like so:

```toml
...

[boilermaker.variables]
a = "1"
b = "2"
...
```

<a name="dev"></a>
# Development

> TODO: document dev once a final implementation is decicded.