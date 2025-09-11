# Installing zig

## Getting started

To install the latest version of zig, run:

```console
cv zig install
```

The command downloads the latest official release from [ziglang.org](https://ziglang.org/download/).
cv installs a versioned distribution to `~/.local/share/cv/zig/<version>`.
To install `zig` executables to `~/.local/bin`, include the `--default` flag:

```console
cv zig install --default
```

## Viewing Zig installations

To view available and installed versions of zig, run:

```console
cv zig list
```
