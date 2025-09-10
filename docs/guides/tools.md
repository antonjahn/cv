# Using tools

cv has support for easily invoking various tools.

## Running tools

The `cvx` command invokes a tool without installing it.

For example, to run `make`:

```console
cvx make
```

!!! note

    This is exactly equivalent to:

    ```console
    cv tool run make
    ```

    `cvx` is provided as an alias for convenience.
