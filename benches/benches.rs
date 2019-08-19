#[macro_use]
extern crate criterion;
extern crate rlua;
extern crate rlua_examples;

use criterion::{black_box, Criterion};
use rlua::{Lua, Result};
use rlua_examples::{prelude::*, Keyring, Octet};

fn empty_script(c: &mut Criterion) {
    c.bench_function("empty_script", move |b| {
        b.iter(|| {
            let res: Result<()> = Lua::new().context(|lua_ctx| {
                lua_ctx.load("").exec().unwrap();
                Ok(())
            });
            res.unwrap();
        })
    });
}

fn preloaded_empty_script(c: &mut Criterion) {
    let lua = Lua::new();
    c.bench_function("preloaded_empty_script", move |b| {
        b.iter(|| {
            let res: Result<()> = lua.context(|lua_ctx| {
                lua_ctx.load(black_box("")).exec().unwrap();
                Ok(())
            });
            res.unwrap();
        })
    });
}

fn keyring_generate(c: &mut Criterion) {
    c.bench_function("keyring_generate", move |b| {
        b.iter(|| {
            let res: Result<Keyring> = Lua::new().context(|lua_ctx| {
                Keyring::load_module(lua_ctx)?;
                lua_ctx.load(black_box("KEYRING.generate()")).eval()
            });
            res.unwrap();
        })
    });
}

fn preloaded_keyring_generate(c: &mut Criterion) {
    let lua = Lua::new();
    lua.context(Keyring::load_module).unwrap();
    c.bench_function("preloaded_keyring_generate", move |b| {
        b.iter(|| {
            let res: Result<Keyring> =
                lua.context(|lua_ctx| lua_ctx.load(black_box("KEYRING.generate()")).eval());
            res.unwrap();
        })
    });
}

fn keyring_sign(c: &mut Criterion) {
    let keyring = Keyring::new_generated().unwrap();
    let message = Octet::new(b"hello".to_vec());
    c.bench_function("keyring_sign", move |b| {
        b.iter(|| {
            let res: Result<Octet> = Lua::new().context(|lua_ctx| {
                lua_ctx.globals().set("keyring", keyring.clone())?;
                lua_ctx.globals().set("message", message.clone())?;
                lua_ctx.load(black_box("keyring:sign(message)")).eval()
            });
            res.unwrap();
        })
    });
}

fn preloaded_keyring_sign(c: &mut Criterion) {
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        lua_ctx
            .globals()
            .set("keyring", Keyring::new_generated().unwrap())
            .unwrap();
        lua_ctx
            .globals()
            .set("message", Octet::new(b"hello".to_vec()))
            .unwrap();
    });
    c.bench_function("preloaded_keyring_sign", move |b| {
        b.iter(|| {
            let res: Result<Octet> =
                lua.context(|lua_ctx| lua_ctx.load(black_box("keyring:sign(message)")).eval());
            res.unwrap();
        })
    });
}

fn keyring_verify(c: &mut Criterion) {
    let keyring = Keyring::new_generated().unwrap();
    let message = Octet::new(b"hello".to_vec());
    let signature = keyring.sign(&message).unwrap();
    c.bench_function("keyring_verify", move |b| {
        b.iter(|| {
            let res: Result<bool> = Lua::new().context(|lua_ctx| {
                lua_ctx.globals().set("keyring", keyring.clone())?;
                lua_ctx.globals().set("message", message.clone())?;
                lua_ctx.globals().set("signature", signature.clone())?;
                lua_ctx
                    .load(black_box("keyring:verify(message, signature)"))
                    .eval()
            });
            assert!(res.unwrap());
        })
    });
}

fn preloaded_keyring_verify(c: &mut Criterion) {
    let lua = Lua::new();
    let keyring = Keyring::new_generated().unwrap();
    let message = Octet::new(b"hello".to_vec());
    lua.context(|lua_ctx| {
        lua_ctx.globals().set("keyring", keyring.clone()).unwrap();
        lua_ctx.globals().set("message", message.clone()).unwrap();
        lua_ctx
            .globals()
            .set("signature", keyring.sign(&message).unwrap())
            .unwrap();
    });
    c.bench_function("preloaded_keyring_verify", move |b| {
        b.iter(|| {
            let res: Result<bool> = lua.context(|lua_ctx| {
                lua_ctx
                    .load(black_box("keyring:verify(message, signature)"))
                    .eval()
            });
            assert!(res.unwrap());
        })
    });
}

criterion_group!(
    benches,
    empty_script,
    preload_lua_empty_script,
    keyring_generate,
    preloaded_keyring_generate,
    keyring_sign,
    preloaded_keyring_sign,
    keyring_verify,
    preloaded_keyring_verify,
);
criterion_main!(benches);
