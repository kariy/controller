use cainome::cairo_serde::NonZero;
use starknet::signers::SigningKey;
use starknet_crypto::FieldElement;

use crate::abigen::cartridge_account::{SignerSignature, StarknetSignature, StarknetSigner};

use super::{AccountSigner, SignError};

use async_trait::async_trait;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl AccountSigner for SigningKey {
    async fn sign(&self, tx_hash: &FieldElement) -> Result<SignerSignature, SignError> {
        let signature = self.sign(tx_hash).map_err(SignError::Signer)?;
        let pubkey = NonZero::new(self.verifying_key().scalar()).unwrap();
        Ok(SignerSignature::Starknet((
            StarknetSigner { pubkey },
            StarknetSignature {
                r: signature.r,
                s: signature.s,
            },
        )))
    }
}
