# Template Structure

Boilermaker is a fairly simple tool, but it does have a few moving parts. This page is meant to give an overview of the structure of what comprises a Boilermaker template, and how the different pieces fit together.

## Template Files

The core of Boilermaker. A template is made up of one or more files, which are used to generate the final output (the "project"). These files are any text file that may, or may not, have variables and logic interpolated in them. Boilermaker uses [minijinja](https://github.com/mitsuhiko/minijinja) as its templating engine. This should be fairly familiar to anyone who has ever used Jinja2 in Python, Django, Selmer in Clojure, or any other templating engine that uses the same syntax. Of which, there's a lot of them out there!

Because of this choice in engines, Boilermaker templates can contain logic, variables and macros (functions). This allows for a lot of flexibility in how you can structure your templates, and how you can generate your output. You can have as many or as few template files as you need, and they can be organized in any way you see fit. You have the ability to `include` files based on logic or use `{% block %}`s to create resuable templates. The point is, it's pretty much up to you how you want to structure your templates. Boilermaker tries to stay out of your way, here.

## Languages

Another core principal of Boilermaker is that it is language-agnostic. This means that you can use any programming language you want to write your templates in, as long as they can be parsed by the template engine. One rule of Boilermaker is that languages live as separate diredctories within the same Boilermaker Template. That will look something like this:

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

> _Here, we're using Python and Javascript but you could use any language(s) you want._

While this structure can be potentially wasteful, in that you may be repeating logic or structure from language to language, it also nudges you in the direction of having a template with a `single purpose`. This is a core principle of Boilermaker, and is meant to encourage users to think about their templates in terms of the output they want to generate, rather than the specific language they want to use.

## Variables

Variables are declared in one of three places:

1. The `boilermaker.toml` file, which is the configuration file for the template. This is where you can declare any variables that are needed for the template, as well as any default values for those variables.
2. The command line, when running the `boil` command. This is where you can override any variables that are declared in the `boilermaker.toml` file
3. An imported file. You can think of this in the context of `.env` files. They will also override the default vars declared in the `boilermaker.toml` file, but they will be overridden by any variables declared on the command line.

> _TIP_: For a more in-depth look at variables, see the [Variables](/docs/variables) page.

## Logic

Anything availabe in minijinja is at your disposal. Period.

## File Path Interpolation

See the [File Paths](/docs/file-paths) page.

## Configuration

See the [Configuration](/docs/configuration) page.

