# Sources

Sources are a way to declare many related templates together into a single collection. By convention, you typically define them at the root of a template or Git repo as `boilermaker_source.toml` and they have the following structure:

## Structure

```toml
[source]
name = ""
description = ""
backend = "sql::sqlite"

[[templates]]
repo = ""
name = ""
lang = ""
branch = ""

[[templates]]
...
```

> _NOTE: `"sql::sqlite"` is the only option as of writing but will likely be expanded in the future._

## Usage

Once you have this file, a "Source" is basically just a file that lives at some URL ("coordinate" in Boilermaker parlance). For instance, maybe you saved it to the root directory of a Git repo.


To add the source to your local database, you can use the `boil sources add` command which simply takes the coordinate to raw source content. Boilermaker will then download the file and install each template as a "Source Template."

```
boil sources add COORDINATE
```

> _TIP: that while very similar, an "installed template" and "source template" are not identical. However, you can install templates from source templates._


## Example

This example is from an early version of the [`boil-hello-world`](https://github.com/yeajustmars/boil-hello-world) template, which is both the template and source. It demonstrates using a single template but adding each language implementation as a separate template in the source. Why, you ask: this allows adding all these templates to the database in a single go and makes them immediately available for the search engine.

> _TIP: There is no hard and fast rule about how or why you may want to group templates together. Sources just provide a generic, reusable way to accomplish it._

> _TIP: Bootstrapping the repo as both the template and source is not a requirement._

Without further explanation, given this source definition:

```toml
[source]
name = "source_boil-hello-world"
description = "Source for official Boilermake Hello World templates"
backend = "sql::sqlite"

[[templates]]
repo = "https://github.com/yeajustmars/boil-hello-world"
name = "hello-world"
lang = "bash"
branch = "main"

[[templates]]
repo = "https://github.com/yeajustmars/boil-hello-world"
name = "hello-world"
lang = "clojure"
branch = "main"

[[templates]]
repo = "https://github.com/yeajustmars/boil-hello-world"
name = "hello-world"
lang = "javascript"
branch = "main"

[[templates]]
repo = "https://github.com/yeajustmars/boil-hello-world"
name = "hello-world"
lang = "python"
branch = "main"

[[templates]]
repo = "https://github.com/yeajustmars/boil-hello-world"
name = "hello-world"
lang = "rust"
branch = "main"
```

We can see a few things:

- the `[source]` section is for the source metadata about the source itself.
- `[[templates]]` just creates a list of templates in TOML.
- each template states its `repo url, name, language and branch`. There is a lot of overlap here with the fields that exist in the `[project]` section of a `boilermaker.toml` template config.
