# About

This repo provide one hook to use with [pre-commit](https://pre-commit.com/) that optimize png files: it will reduce their size without losing quality.

⚠️ It requires the `rust` toolchain installed, and uses [`oxipng`](https://github.com/shssoichiro/oxipng) under the hood.
It is predictable, so running the hook twice on the same PNG file will produce the same result.

To install `rust`, head to https://www.rust-lang.org/tools/install and do it.

## Example usage

Then: add a similar snippet to your `.pre-commit-config.yaml` file

```yaml
- repo: https://github.com/redwarp/optimize-png-hooks
  rev: ${LATEST_SHA_OR_VERSION}
  hooks:
    - id: optimize-png
```

Optionally, you can activate zopfli for extra crunch. I wouldn't do it if I were you, as the cost is IMO too high (zopfli is really efficient, but is order of magnitudes more expensive in terms of computation and will take forever to run on each png file)

So, you should probably not do it, but just in case you want it:

```yaml
- repo: https://github.com/redwarp/optimize-png-hooks
  rev: ${LATEST_SHA_OR_VERSION}
  hooks:
    - id: optimize-png
      args: [--zopfli]
```
