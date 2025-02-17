# Building Documentation

Ptolemy uses [MkDocs](https://www.mkdocs.org/) to build documentation. Below are instructions to generate and run Ptolemy's documentation.

Before starting, you must have `uv` installed. If you're using a Mac, you can install `uv` with `brew`:
```sh
brew install uv
```

If you are using another operating system or don't want to use `brew`, you can find instructions on how to install `uv` in [their documentation](https://docs.astral.sh/uv/getting-started/installation/).

To generate Ptolemy's docs, clone the `ptolemy` GitHub repo:

```sh
git clone https://github.com/PtolemyLovesYou/ptolemy.git
```

Then, start the mkdocs server:
```sh
make docs
```

You should be able to find the docs at `http://localhost:8080` in your web browser.
