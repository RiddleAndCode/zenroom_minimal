# zenroom_minimal

A minimal Lua VM binding with default crypto / encoding modules
and a builtin human readable DSL for building scenario specific
execution environments.

## Motivation

`zenroom_minimal` is an offshoot from the DECODEproject's
[Zenroom](https://github.com/DECODEproject/zenroom/tree/master/src).
When evaluating Zenroom, we found that although the security and
cryptographic capabilities were very flexible, the performance of the VM
did not meet our standards for high-performance / high-throughput.
In addition we needed flexible support for scenario logic to run in various
secure environments, thus `zenroom_minimal` was born, using Rust as a
module building tool instead of Zenroom's C libraries.

## Usage

Although Zenroom's libraries can be used directly with
an [`rlua`] Lua environment. It is
recommended that you use one of `zenroom_minimal`'s Runtime Environments
for Code execution to harness the full power and security of the VM

### Default Runtime

The defualt runtime provides a Sandboxed Lua environment. This Lua
environment prevents the use of OS commands (like Time / File System / etc)
and provides instead an `import` function for whitelisted modules.

```rust
let res = DefaultRuntime::default()
    .load("return 'Hello, world!'")?
    .eval()?;
```

### Zenroom Runtime

The Zenroom Runtime leverages Zencode to execute Human Readable protected
code through loaded scenarios. Take a look at the `examples` for more
information on how to use
