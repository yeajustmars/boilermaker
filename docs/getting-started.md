# Getting Started

## Install Boilermaker

The best way to see how Boilermaker works is to try it out! If you haven't already, check out the [installation instructions](/docs/install) to get Boilermaker set up on your machine.

Next, let's install a template and run it. This will demonstrate the absolute basics of Boilermaker.

## Install a template

For this example, we'll use the [`boil-hello-world`](https://github.com/yeajustmars/boil-hello-world) template, which is a simple template that prints "Hello, World!".

> This template comes in a few languages and to run it you'll need the runtime/compiler for that language installed on your machine.

Some common languages that you likely already have installed are:

- BASH
- Node.js
- Python
- Rust?

For this, we'll use the most likely runtime which is the BASH shell itself.

> _NOTE_: This should also work for ZSH, Fish, etc, as long as the shell can interpret BASH scripts.

### Install the Boilermaker template

To install the template, run the following command:

```bash
boil install https://github.com/yeajustmars/boil-hello-world --lang=bash
```

> This tells Boilermaker, via the `boil` command to install the template to local disk so that we can then use it to create projects.

_The `--lang` option is used to specify which language of the template we want to install. Boilermaker templates can have as many language implementations as desired so this option is sometimes necessary to identify which version of the template to use._

## Create a project from the template

```bash
boil new boil-hello-world -l bash -Od /tmp -n boil-bash
```

There's a few things happening here worth pointing out:

- `boil new` says "take template X and render it into a new project"
- `-l bash` is just to ensure we have the right lang. _(Boilermaker will resolve a template w/o this option if there's only one lang installed.)_
- `-O` tells `boil` to `overwrite` the project if it already exists. This is just to make sure we don't get an error if we run this command multiple times.
- `-d /tmp` tells `boil` to create the project in the `/tmp` directory. You can change this to whatever directory you want.
- `-n` is short for `--rename` and it tells `boil` to rename the project to `boil-bash`. By default, the project would be named `boil-hello-world`, but this option allows us to change that.

That's it for creating projects! The rest is up to you the template developer for what the project will do when you run it.

## Run the project

To run our new BASH project, we can `cd` into the project directory and run it. These instructions will change depending on the template/language and Boilermaker does not make or enforce any assumptions or restrictions about how to run your projects. Most projects will likely have some kind of `main` function but this is not a requirement or a rule.

In our case, we can just `cd` into the project directory and call BASH, directly:

```bash
cd /tmp/boil-bash
bash src/hello-world.sh
```

If all went well, you should see something like:

```bash
Hello, boiled BASH!
Hello, World!
```

Congratulations! You've just installed a Boilermaker template, created a project from it, and ran that project. This is the basic workflow for using Boilermaker and you can now explore the documentation to learn about all the other features and capabilities it provides.

A good next step would be to check out the [Template Structure](/docs/template-structure) documentation to learn about how Boilermaker templates are structured and how to create your own.

You can also check out the [CLI Reference](/docs/cli-reference) to learn about all the different commands and options available in the `boil` CLI.
