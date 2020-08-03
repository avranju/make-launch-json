# make-launch-json

Processes input that looks like this from stdin:

```bash
#!/bin/sh

export RUST_LOG="trace"
export BOO="baa"
export COW="moo"

# start app
~/code/theapp/target/debug/theapp
```

And produces output like the following which can be used in VS Code's `launch.json` to populate the env vars for a debug configuration.

```json
{
  "env": {
    "RUST_LOG": "trace",
    "BOO": "baa",
    "COW": "moo"
  }
}

```