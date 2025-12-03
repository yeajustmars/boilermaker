# Boilermaker Example Sqlite Source

This example demonstrates the basics of setting up a Sqlite Source for Boilermaker.

It shows the following:

- How to use a single `boilermaker_source.toml` file to create a reproducable Boilermaker Source.
- How to configure the `source` section in the `boilermaker_source.toml` file.
- How to add `templates` to the `boilermaker_source.toml` file.

# Usage 

## URL/Remote 

``` 
boil install https://github.com/yeajustmars/boilermaker/tree/main/examples/source_sqlite/boilermaker_source.toml 
```

## Local 

``` 
# git clone https://github.com/yeajustmars/boilermaker
# cd boilermaker

boil install examples/source_sqlite/boilermaker_source.toml
```

# Breakdown 

## `[source]` 

The `source` section declares the metadata for the Source. This includes things like the 
`name`, `description` and `type`. Of these, `name` and `type` are required for Boilermaker to 
process the Source. The `description` is optional but highly recommended.

## `[[templates]]`

This Source adds just a couple of Templates. However, in those, you can see that depending on 
what you're trying to do, a different subset of configuration is needed. For instance, in the 
`hello-world-clj` template, we only need the `repo` and `lang`, whereas for the `hello-world-rs` 
_(custom name)_, we have special instructions for `branch`, `subdir`, etc as this template is 
part of a larger project _(Boilermaker itself, in this case)_.