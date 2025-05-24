# `node-deps`

`node-deps` is a small (could also be considered polished) CLI that prints dependencies of a Node.js project in a nice and readable way

# Usage

If you are in a Node project and want to see its dependencies, simply run:

```sh
node-deps
```

And that's all

If you are in another directory, it is also pretty intuitive

```sh
node-deps ./my-node-project
```

Or, in case you need it, you can also do it from a file that is not called `package.json`. It just works

# stdin

It also supports input from stdin, just add the `-` argument

This means, you can integrate it with stuff like `curl` and do

```sh
curl -s -L https://github.com/npm/cli/raw/refs/heads/latest/package.json | node-deps -
```
