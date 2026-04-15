# File Paths

Boilermaker allows a special syntax for interpolating variables into file paths. This is done using the `___x___` (triple underscore delimiter) or `---x---` (triple dash delimiter) syntax. These two are interchangeable, and only exist for readability for the puposes of dash- or underscore-delimited filenames. This allows you to have dynamic file paths, which can be very useful for generating output that has a specific naming convention.

## The Boilermaker file

For instance, let's say you have this in your `boilermaker.toml` file:

```toml
[variables]
app_name = "my-app"
```

## Directory Structure

Let's say you have this  directory structure in your template:

```
my-template/
├── python
    ├── ___app_name___/
    │   ├── main.py
    │   └── utils.py
├── boilermaker.toml
```

## Result after render

After rendering the template, your project would look like this:

```
my-project/
├── my-app/
│   ├── main.py
│   └── utils.py
```

> _Note that `app_name` is interpolated directly into the file path._

## Rules and Notes

Unlike template files, Boilermaker does _not_ support logic in file paths. This is a conscious decision, as it encourages users to think about their templates in terms of the output they want to generate, rather than the specific directory stuctures. However, since you have minijinja, you can always optionally include templates based on a predicate, therefore further controlling what files get generated in the output project.

