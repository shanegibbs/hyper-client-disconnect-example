### Modification of the Echo example

Start server with:

```
$ cargo run
```

Test with:

```
$ curl -v --data Hello localhost:1337/echo
```

#### Observed
If the `curl` command is `Ctrl-C`'d before completion (within 2 seconds), the sleep future still completes and a response is generated.

#### Expected
The sleep future doesn't complete. The `Echo` service is `Drop`ed perhaps?
