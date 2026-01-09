# Boilermaker Variable Profiles

Simple template for demonstrating [Boilermaker's](https://boilermaker.dev)
[`Variable Profiles`](https://boilermaker.dev/docs/variable-profiles) feature.

| | |
|-|-|
| Requires | Node.js runtime and/or Python interpreter |
| Docs | [https://boilermaker.dev/docs/variable-profiles](https://boilermaker.dev/docs/variable-profiles) |
| Src | [https://github.com/yeajustmars/boilermaker/tree/main/examples/variable-profiles](https://github.com/yeajustmars/boilermaker/tree/main/examples/variable-profiles) |
| Author | [@yeajustmars](https://github.com/yeajustmars) |

## Index

1. [Overview](#overview)
2. [Usage](#usage)
    1. [Quick install](#quick-install)
    2. [Node](#node)
    3. [Python](#python)

# Overview

This template has two languages/engines: (1) [Python](#python); and (2) [Node](#node).
_(I figure there's a decent chance these are both installed on a dev's system.)_
**The idea is to demonstrate a slight difference between incompatible languages with
a single Boiler template and a single context structure.**
For instance, in Python, boolean values are capitalized (`True`/`False`),
whereas in Node, they are lowercase (`true`/`false`). This minor difference is enough that to implement 
this in Boiler, we would either need to move the logic into the source code or we would need separate 
branches or subdirectories. Boilermaker has a 3rd option: implement
a [`Variable Profile`](https://boilermaker.dev/docs/variable-profiles) for each language and avoid 
having duplicated logic in the source files.

# Usage

> _HEADS UP!_ If you don't have Boilermaker installed, follow the
> [getting started](https://boilermaker.dev/docs/getting-started) instructions and
> then come back here.

For a full demo, install and instantiate both versions of the template.
_If you just want one, take your pick: [Node](#node), [Python](#python)._

If you do want both, you can run the following script.

## Quick install

```bash 
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/yeajustmars/boilermaker/refs/heads/main/examples/variable-profiles/quick-install.sh)"
```

_([Source here](https://github.com/yeajustmars/boilermaker/tree/main/examples/variable-profiles/quick-install.sh))_

## Node

```shell
boil install https://github.com/yeajustmars/boilermaker/ \
  -d examples/variable-profiles \
  -l node \
  -n var-profiles-node

boil new var-profiles-node -Od /tmp --use-profile node

cd /tmp/var-profiles-node

node src/main.js
```

> _Note the `-n` and `--use-profile` options above._

## Python

```shell
boil install https://github.com/yeajustmars/boilermaker/ \
  -d examples/variable-profiles \
  -l python \
  -n var-profiles-python

boil new var-profiles-python -Od /tmp --use-profile python

cd /tmp/var-profiles-python 

python3 src/main.py
```

> _Note the `-n` and `--use-profile` options above._
