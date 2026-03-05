# Variables

## Standard Variables

As mentioned before, Boilermaker uses [minijinja](https://docs.rs/minijinja/latest/minijinja/) as its templating engine. This means anything you can do in minijinja, you can do in Boilermaker.

As a quick overview, minijinja (much like Jinja2, Selmer, Django Templates, etc) use the double-brace syntax by default _(and as of writing Boilermaker doesn't allow changing this)_.

Any of the following are valid variable interpolations:

```
{{ a }}

{{ x.y.z }}

{{ my.vector_of_maps[0].some_key }}
```

### Nested variables

Both `boilermaker.toml` and any [`extra vars file`](/docs/configuration#doc-section-4) allow nested vars. As of writing the command line `--var` option **does not** but that may change in the future.

### Merging Ruls

Due to how Boilermaker works with minijinja, merges of any variables are **shallow**.

Given the following:

```toml
[project]
...

[variables]
a = 1
b = true
c = { d = { e = "f" }}

[variables.profiles.override-c]
c = { x = { y = "z" } }
```

Calling `boil new TEMPLATE --profile override-c` means that you would have the following context:

```
a = 1
b = true
c = { x = { y = "z" } }
```

> _Note that `c` now contains `x.y`, not `d.e`. This is a shallow merge._

## Variable Profiles

So, let's say we have the following `boilermaker.toml`:

```toml
[project]
...

[variables]
a = 1
b = 2
c = -1

[variables.profiles.one]
c = 3

[variables.profiles.two]
a = 4
b = 5
c = { d = { e = "f" } }
```

We would get the following contexts when using these profiles:

#### Profile "one"

```
boil new TEMPLATE -p one
```

_produces:_

```toml
a = 1
b = 2
c = 3  # <- Note that `c` is now a different value.
```


#### Profile "two"

```
boil new TEMPLATE -p two
```

_produces:_

```toml
a = 4  # <- `a` is overridden
b = 5  # <- `b` is overridden
c = { d = { e = "f" } }  # <- `c` is completely changed in type and all
```

