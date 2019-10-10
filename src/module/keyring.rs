use super::{DefaultModule, Module, Octet};
use ring::{rand, signature, signature::EcdsaKeyPair, signature::KeyPair};
use rlua::{Context, Error, Result, UserData, UserDataMethods, Value, Variadic};

/// A public / private Keypair. At the moment, this Keypair is only configured
/// for the NIST256 curve. This may change in the future however.
///
/// After instantiating from the [`KeyringClass`] Module, the lua variable
/// exposes multiple instance methods:
/// * `keyring:generate()`: Generate a new private / public key pair
/// * `keyring:public(<optional Octet>)`: Public key getter / setter
/// * `keyring:private(<optional Octet>)`: Private key getter / setter
/// * `keyring:sign(<message Octet>)`: Sign a message. Returns Octet with signature bytes
/// * `keyring:verify(<message Octet>, <signature Octet>)`: Verify a signature and message
#[derive(Clone, Debug, Default)]
pub struct Keyring {
    public: Octet,
    private: Octet,
}

/// A [`Keyring`] factory
///
/// Exposes a default `KEYRING` module in Lua
/// * `KEYRING.new()`: Create a new default Keyring (blank)
/// * `KEYRING.generated()`: Create a new Keyring and generate a private / public keypair
#[derive(Default, Clone)]
pub struct KeyringClass;

impl Keyring {
    /// Create new blank Keyring
    pub fn new() -> Self {
        Keyring::default()
    }

    /// Create a new Keyring and generate a private / public keypair
    pub fn new_generated() -> Result<Self> {
        let mut keyring = Keyring::new();
        keyring.generate()?;
        Ok(keyring)
    }

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

    /// Sign a message with the private key
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

    /// Verify the signature signed by the public key
    pub fn verify(&self, message: &Octet, signature: &Octet) -> bool {
        signature::verify(
            &signature::ECDSA_P256_SHA256_FIXED,
            untrusted::Input::from(self.public.as_ref()),
            untrusted::Input::from(message.as_ref()),
            untrusted::Input::from(signature.as_ref()),
        )
        .is_ok()
    }

    /// Generate new private / public keypair
    pub fn generate(&mut self) -> Result<()> {
        self.generate_private()?;
        self.generate_public()?;
        Ok(())
    }

    /// Get the public key
    pub fn public(&self) -> &Octet {
        &self.public
    }

    /// Get the private key
    pub fn private(&self) -> &Octet {
        &self.private
    }

    /// Set the public key
    pub fn set_public(&mut self, public: Octet) -> Result<Octet> {
        // TODO validate input and see if it matches private
        self.public = public.clone();
        Ok(public)
    }

    /// Set the private key
    pub fn set_private(&mut self, private: Octet) -> Result<Octet> {
        self.private = private.clone();
        self.generate_public()?;
        Ok(private)
    }
}

impl UserData for Keyring {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("generate", |_, this, ()| Ok(this.generate()?));
        methods.add_method_mut("public", |_, this, vals: Variadic<Octet>| {
            if vals.len() > 0 {
                this.set_public(vals[0].clone())
            } else {
                Ok(this.public().clone())
            }
        });
        methods.add_method_mut("private", |_, this, vals: Variadic<Octet>| {
            if vals.len() > 0 {
                this.set_private(vals[0].clone())
            } else {
                Ok(this.private().clone())
            }
        });
        methods.add_method("sign", |_, this, message| Ok(this.sign(&message)));
        methods.add_method("verify", |_, this, (message, signature)| {
            Ok(this.verify(&message, &signature))
        });
    }
}

impl Module for KeyringClass {
    fn build_module<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let module = ctx.create_table()?;
        module.set("new", ctx.create_function(|_, ()| Ok(Keyring::new()))?)?;
        module.set(
            "generate",
            ctx.create_function(|_, ()| Ok(Keyring::new_generated()?))?,
        )?;
        Ok(Value::Table(module))
    }
}

impl DefaultModule for KeyringClass {
    const GLOBAL_VAR: &'static str = "KEYRING";
}

#[cfg(test)]
mod tests {
    use super::*;
    use rlua::{Lua, Result};

    #[test]
    fn lua_generate() -> Result<()> {
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            KeyringClass::import_module(lua_ctx)?;
            lua_ctx.load("KEYRING.generate()").eval()
        })
        .and_then(|keyring: Keyring| {
            assert_eq!(keyring.public().len(), 65);
            assert_eq!(keyring.private().len(), 138);
            Ok(())
        })
    }

    #[test]
    fn lua_sign() -> Result<()> {
        let lua = Lua::new();
        let keyring = Keyring::new_generated()?;
        let message = Octet::new(b"hello".to_vec());

        lua.context(|lua_ctx| {
            lua_ctx.globals().set("keyring", keyring)?;
            lua_ctx.globals().set("message", message)?;
            lua_ctx.load("keyring:sign(message)").eval()
        })
        .and_then(|signature: Octet| {
            assert_eq!(signature.len(), 64);
            Ok(())
        })
    }

    #[test]
    fn lua_verify() -> Result<()> {
        let lua = Lua::new();
        let keyring = Keyring::new_generated()?;
        let message = Octet::new(b"hello".to_vec());
        let signature = keyring.sign(&message)?;

        lua.context(|lua_ctx| {
            lua_ctx.globals().set("keyring", keyring)?;
            lua_ctx.globals().set("message", message)?;
            lua_ctx.globals().set("signature", signature)?;
            lua_ctx.load("keyring:verify(message, signature)").eval()
        })
        .and_then(|verified: bool| {
            assert!(verified);
            Ok(())
        })
    }
}
