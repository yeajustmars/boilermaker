# Template Structure

Boilermaker is a fairly simple tool, but it does have a few moving parts. This page is meant to give an overview of the structure of what comprises a Boilermaker template, and how the different pieces fit together.

## Template Files

The core of Boilermaker. A template is made up of one or more files, which are used to generate the final output. These file are any text file that may, or may not, have variables and logic interpolated in them. Boilermaker uses [minijinja](https://github.com/mitsuhiko/minijinja) as its templating engine. This should be fairly familiar to anyone who has ever used Jinja2 in Python, Django, Selmer in Clojure, or any other templating engine that uses the same syntax. Of which, there's a lot of them out there!

Because of this choice in engines, Boilermaker templates can contain logic, variables and macros (functions). This allows for a lot of flexibility in how you can structure your templates, and how you can generate your output. You can have as many or as few template files as you need, and they can be organized in any way you see fit.

## Languages

Another core principal of Boilermaker is that it is language-agnostic. This means that you can use any programming language you want to write your templates in, as long as they can be parsed by the template engine. One rule of Boilermaker is that languages live as separate subdiredctories within the same Boilermaker Template. That will look something like this:

```
my-template/
├── python/
│   ├── main.py
│   └── utils.py
└── javascript/
    ├── main.js
    └── utils.js
├── boilermaker.toml
└── README.md
```

> _Here, we're using Python and Javascript but you could use any language(s) you want!_

While this structure can be potentially wasteful, in that you may be repeating logic or structure from language to language, it also guarantees that you can have a template that has a `single purpose`, in multiple languages. This is a core principle of Boilermaker, and is meant to encourage users to think about their templates in terms of the output they want to generate, rather than the specific language they want to use.

## Variables

Variables are declared in one of three places:

1. The `boilermaker.toml` file, which is the configuration file for the template. This is where you can declare any variables that are needed for the template, as well as any default values for those variables.
2. The command line, when running the `boil` command. This is where you can override any variables that are declared in the `boilermaker.toml` file
3. An imported file. You can think of this in the context of `.env` files. They will also override the default vars declared in the `boilermaker.toml` file, but they will be overridden by any variables declared on the command line.

## Logic

Anything availabe in minijinja is at your disposal. Period.

## File Paths and Variables

Boilermaker also allows a special syntax for interpolating variables into file paths. This is done using the `___x___` or `---x---` syntax. These two are interchangeable, and only exist for readability for the puposes of dash- or underscore-delimited filenames. This allows you to have dynamic file paths, which can be very useful for generating output that has a specific naming convention.

For instance, let's say you have this in your `boilermaker.toml` file:

```toml
[variables]
app_name = "my-app"
```

and this directory structure in your template:

```
my-template/
├── python
    ├── ___app_name___/
    │   ├── main.py
    │   └── utils.py
├── boilermaker.toml
```

After rendering the template, your project would look like this:

```
my-project/
├── my-app/
│   ├── main.py
│   └── utils.py
```

> _Note that `app_name` is interpolated directly into the file path._

_NOTE_: Unlike template files, Boilermaker does _not_ support logic in file paths. This is a conscious decision, as it encourages users to think about their templates in terms of the output they want to generate, rather than the specific directory stuctures. However, since you have minijinja, you can always optionally include templates based on a predicate, therefore further controlling what files get generated in the output project.

## Configuration

Configuration for Boilermaker is done is one of 4 places:

1. The `boilermaker.toml` file in the template directory, which controls the behavior of that specific template.
3. An imported file, which can be used to override any configuration options declared in the `boilermaker.
2. The command line, when running the `boil` command, which can be used to override any configuration options declared in the `boilermaker.toml` files.
4. The global `~/.boilermaker/boilermaker.toml` file, which controls Boilermaker's behavior.

For the purposes of this doc, we'll focus on everything but `#4`, the global Boilermaker config, as that's for controlling Boilermaker itself, versus controlling the behavior of a specific template. For more information on the global config, see the [Configuration](/docs/configuration#global) page.
