#[macro_use]
extern crate criterion;
extern crate rlua;
extern crate rlua_examples;

use criterion::Criterion;
use rlua::{Lua, Result};
use rlua_examples::keyring::{build_keyring_module, Keyring};
use rlua_examples::octet::Octet;

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

fn keyring_generate(c: &mut Criterion) {
    c.bench_function("keyring_generate", move |b| {
        b.iter(|| {
            let res: Result<Keyring> = Lua::new().context(|lua_ctx| {
                lua_ctx
                    .globals()
                    .set("KEYRING", build_keyring_module(lua_ctx)?)?;
                lua_ctx.load("KEYRING.generate()").eval()
            });
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
                lua_ctx.load("keyring:sign(message)").eval()
            });
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
                lua_ctx.load("keyring:verify(message, signature)").eval()
            });
            assert!(res.unwrap());
        })
    });
}

criterion_group!(
    benches,
    empty_script,
    keyring_generate,
    keyring_sign,
    keyring_verify
);
criterion_main!(benches);
