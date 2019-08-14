use crate::octet::Octet;
use ring::{rand, signature, signature::EcdsaKeyPair, signature::KeyPair};
use rlua::{prelude::*, Context, Error, Result, Table, UserData, UserDataMethods, Value};

#[derive(Clone, Debug, Default)]
pub struct Keyring {
    public: Octet,
    private: Octet,
}

impl Keyring {
    fn generate_private(&mut self) -> Result<()> {
        let rng = rand::SystemRandom::new();
        let doc = EcdsaKeyPair::generate_pkcs8(&signature::ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
            .map_err(|e| Error::RuntimeError(e.to_string()))?;
        self.private = doc.as_ref().to_vec().into();
        Ok(())
    }

    fn keypair(&self) -> Result<EcdsaKeyPair> {
        EcdsaKeyPair::from_pkcs8(
            &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            untrusted::Input::from(self.private.as_ref()),
        )
        .map_err(|e| Error::RuntimeError(e.to_string()))
    }

    fn generate_public(&mut self) -> Result<()> {
        self.public = self.keypair()?.public_key().as_ref().to_vec().into();
        Ok(())
    }

    pub fn sign(&self, message: &Octet) -> Result<Octet> {
        let rng = rand::SystemRandom::new();
        Ok(self
            .keypair()?
            .sign(&rng, untrusted::Input::from(message.as_ref()))
            .unwrap()
            .as_ref()
            .to_vec()
            .into())
    }

    pub fn verify(&self, message: &Octet, signature: &Octet) -> bool {
        signature::verify(
            &signature::ECDSA_P256_SHA256_FIXED,
            untrusted::Input::from(self.public.as_ref()),
            untrusted::Input::from(message.as_ref()),
            untrusted::Input::from(signature.as_ref()),
        )
        .is_ok()
    }

    pub fn generate(&mut self) -> Result<()> {
        self.generate_private()?;
        self.generate_public()?;
        Ok(())
    }

    pub fn public(&self) -> &Octet {
        &self.public
    }

    pub fn private(&self) -> &Octet {
        &self.private
    }

    pub fn set_public(&mut self, public: Octet) -> Result<()> {
        // TODO validate input and see if it matches private
        self.public = public;
        Ok(())
    }

    pub fn set_private(&mut self, private: Octet) -> Result<()> {
        self.private = private;
        self.generate_private()?;
        Ok(())
    }
}

impl UserData for Keyring {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("generate", |_, this, ()| Ok(this.generate()?));
        methods.add_method("public", |_, this, ()| Ok(this.public().clone()));
        methods.add_method("private", |_, this, ()| Ok(this.private().clone()));
        methods.add_method("sign", |_, this, message| Ok(this.sign(&message)));
        methods.add_method("verify", |_, this, (message, signature)| {
            Ok(this.verify(&message, &signature))
        });
    }
}

pub fn build_keyring_module(ctx: Context) -> Result<Table> {
    let module = ctx.create_table()?;
    module.set("new", ctx.create_function(|_, ()| Ok(Keyring::default()))?)?;
    module.set(
        "generate",
        ctx.create_function(|_, ()| {
            let mut keyring = Keyring::default();
            keyring.generate()?;
            Ok(keyring)
        })?,
    )?;
    Ok(module)
}
