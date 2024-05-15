#[derive(Debug)]
pub struct CartridgeAccount<A: starknet::accounts::ConnectedAccount + Sync> {
    pub address: starknet::core::types::FieldElement,
    pub account: A,
    pub block_id: starknet::core::types::BlockId,
}
impl<A: starknet::accounts::ConnectedAccount + Sync> CartridgeAccount<A> {
    pub fn new(address: starknet::core::types::FieldElement, account: A) -> Self {
        Self {
            address,
            account,
            block_id: starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending),
        }
    }
    pub fn set_contract_address(mut self, address: starknet::core::types::FieldElement) {
        self.address = address;
    }
    pub fn provider(&self) -> &A::Provider {
        self.account.provider()
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive(Debug)]
pub struct CartridgeAccountReader<P: starknet::providers::Provider + Sync> {
    pub address: starknet::core::types::FieldElement,
    pub provider: P,
    pub block_id: starknet::core::types::BlockId,
}
impl<P: starknet::providers::Provider + Sync> CartridgeAccountReader<P> {
    pub fn new(address: starknet::core::types::FieldElement, provider: P) -> Self {
        Self {
            address,
            provider,
            block_id: starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending),
        }
    }
    pub fn set_contract_address(mut self, address: starknet::core::types::FieldElement) {
        self.address = address;
    }
    pub fn provider(&self) -> &P {
        &self.provider
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Call {
    pub to: cainome::cairo_serde::ContractAddress,
    pub selector: starknet::core::types::FieldElement,
    pub calldata: Vec<starknet::core::types::FieldElement>,
}
impl cainome::cairo_serde::CairoSerde for Call {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.to);
        __size += starknet::core::types::FieldElement::cairo_serialized_size(&__rust.selector);
        __size +=
            Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&__rust.calldata);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.to,
        ));
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            &__rust.calldata,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let to = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
        let selector = starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&selector);
        let calldata =
            Vec::<starknet::core::types::FieldElement>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&calldata);
        Ok(Call {
            to,
            selector,
            calldata,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WebauthnPubKey {
    pub x: cainome::cairo_serde::U256,
    pub y: cainome::cairo_serde::U256,
}
impl cainome::cairo_serde::CairoSerde for WebauthnPubKey {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.x);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.y);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.x));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.y));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let x = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&x);
        let y = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&y);
        Ok(WebauthnPubKey { x, y })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SessionSignature {
    pub signature_type: starknet::core::types::FieldElement,
    pub r: starknet::core::types::FieldElement,
    pub s: starknet::core::types::FieldElement,
    pub session_key: starknet::core::types::FieldElement,
    pub session_expires: u64,
    pub root: starknet::core::types::FieldElement,
    pub proofs: SignatureProofs,
    pub session_token: Vec<starknet::core::types::FieldElement>,
}
impl cainome::cairo_serde::CairoSerde for SessionSignature {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            starknet::core::types::FieldElement::cairo_serialized_size(&__rust.signature_type);
        __size += starknet::core::types::FieldElement::cairo_serialized_size(&__rust.r);
        __size += starknet::core::types::FieldElement::cairo_serialized_size(&__rust.s);
        __size += starknet::core::types::FieldElement::cairo_serialized_size(&__rust.session_key);
        __size += u64::cairo_serialized_size(&__rust.session_expires);
        __size += starknet::core::types::FieldElement::cairo_serialized_size(&__rust.root);
        __size += SignatureProofs::cairo_serialized_size(&__rust.proofs);
        __size += Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(
            &__rust.session_token,
        );
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.signature_type,
        ));
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.r,
        ));
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.s,
        ));
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.session_key,
        ));
        __out.extend(u64::cairo_serialize(&__rust.session_expires));
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.root,
        ));
        __out.extend(SignatureProofs::cairo_serialize(&__rust.proofs));
        __out.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            &__rust.session_token,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let signature_type =
            starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&signature_type);
        let r = starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&r);
        let s = starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&s);
        let session_key =
            starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&session_key);
        let session_expires = u64::cairo_deserialize(__felts, __offset)?;
        __offset += u64::cairo_serialized_size(&session_expires);
        let root = starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&root);
        let proofs = SignatureProofs::cairo_deserialize(__felts, __offset)?;
        __offset += SignatureProofs::cairo_serialized_size(&proofs);
        let session_token =
            Vec::<starknet::core::types::FieldElement>::cairo_deserialize(__felts, __offset)?;
        __offset +=
            Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&session_token);
        Ok(SessionSignature {
            signature_type,
            r,
            s,
            session_key,
            session_expires,
            root,
            proofs,
            session_token,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct TokenRevoked {
    pub token: Vec<starknet::core::types::FieldElement>,
}
impl cainome::cairo_serde::CairoSerde for TokenRevoked {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&__rust.token);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            &__rust.token,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let token =
            Vec::<starknet::core::types::FieldElement>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&token);
        Ok(TokenRevoked { token })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SignatureProofs {
    pub single_proof_len: u32,
    pub proofs_flat: Vec<starknet::core::types::FieldElement>,
}
impl cainome::cairo_serde::CairoSerde for SignatureProofs {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += u32::cairo_serialized_size(&__rust.single_proof_len);
        __size +=
            Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&__rust.proofs_flat);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(u32::cairo_serialize(&__rust.single_proof_len));
        __out.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            &__rust.proofs_flat,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let single_proof_len = u32::cairo_deserialize(__felts, __offset)?;
        __offset += u32::cairo_serialized_size(&single_proof_len);
        let proofs_flat =
            Vec::<starknet::core::types::FieldElement>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&proofs_flat);
        Ok(SignatureProofs {
            single_proof_len,
            proofs_flat,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WebauthnSignature {
    pub signature_type: starknet::core::types::FieldElement,
    pub r: cainome::cairo_serde::U256,
    pub s: cainome::cairo_serde::U256,
    pub type_offset: u32,
    pub challenge_offset: u32,
    pub origin_offset: u32,
    pub client_data_json: Vec<u8>,
    pub origin: Vec<u8>,
    pub authenticator_data: Vec<u8>,
}
impl cainome::cairo_serde::CairoSerde for WebauthnSignature {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            starknet::core::types::FieldElement::cairo_serialized_size(&__rust.signature_type);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.r);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.s);
        __size += u32::cairo_serialized_size(&__rust.type_offset);
        __size += u32::cairo_serialized_size(&__rust.challenge_offset);
        __size += u32::cairo_serialized_size(&__rust.origin_offset);
        __size += Vec::<u8>::cairo_serialized_size(&__rust.client_data_json);
        __size += Vec::<u8>::cairo_serialized_size(&__rust.origin);
        __size += Vec::<u8>::cairo_serialized_size(&__rust.authenticator_data);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.signature_type,
        ));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.r));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.s));
        __out.extend(u32::cairo_serialize(&__rust.type_offset));
        __out.extend(u32::cairo_serialize(&__rust.challenge_offset));
        __out.extend(u32::cairo_serialize(&__rust.origin_offset));
        __out.extend(Vec::<u8>::cairo_serialize(&__rust.client_data_json));
        __out.extend(Vec::<u8>::cairo_serialize(&__rust.origin));
        __out.extend(Vec::<u8>::cairo_serialize(&__rust.authenticator_data));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let signature_type =
            starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&signature_type);
        let r = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&r);
        let s = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&s);
        let type_offset = u32::cairo_deserialize(__felts, __offset)?;
        __offset += u32::cairo_serialized_size(&type_offset);
        let challenge_offset = u32::cairo_deserialize(__felts, __offset)?;
        __offset += u32::cairo_serialized_size(&challenge_offset);
        let origin_offset = u32::cairo_deserialize(__felts, __offset)?;
        __offset += u32::cairo_serialized_size(&origin_offset);
        let client_data_json = Vec::<u8>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<u8>::cairo_serialized_size(&client_data_json);
        let origin = Vec::<u8>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<u8>::cairo_serialized_size(&origin);
        let authenticator_data = Vec::<u8>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<u8>::cairo_serialized_size(&authenticator_data);
        Ok(WebauthnSignature {
            signature_type,
            r,
            s,
            type_offset,
            challenge_offset,
            origin_offset,
            client_data_json,
            origin,
            authenticator_data,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct OwnerAdded {
    pub new_owner_guid: starknet::core::types::FieldElement,
}
impl cainome::cairo_serde::CairoSerde for OwnerAdded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            starknet::core::types::FieldElement::cairo_serialized_size(&__rust.new_owner_guid);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.new_owner_guid,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let new_owner_guid =
            starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&new_owner_guid);
        Ok(OwnerAdded { new_owner_guid })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct OwnerRemoved {
    pub removed_owner_guid: starknet::core::types::FieldElement,
}
impl cainome::cairo_serde::CairoSerde for OwnerRemoved {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            starknet::core::types::FieldElement::cairo_serialized_size(&__rust.removed_owner_guid);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(starknet::core::types::FieldElement::cairo_serialize(
            &__rust.removed_owner_guid,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let removed_owner_guid =
            starknet::core::types::FieldElement::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::FieldElement::cairo_serialized_size(&removed_owner_guid);
        Ok(OwnerRemoved { removed_owner_guid })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum SessionComponentEvent {
    TokenRevoked(TokenRevoked),
}
impl cainome::cairo_serde::CairoSerde for SessionComponentEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            SessionComponentEvent::TokenRevoked(val) => {
                TokenRevoked::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        match __rust {
            SessionComponentEvent::TokenRevoked(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(TokenRevoked::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __index: u128 = __felts[__offset].try_into().unwrap();
        match __index as usize {
            0usize => Ok(SessionComponentEvent::TokenRevoked(
                TokenRevoked::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "SessionComponentEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for SessionComponentEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("TokenRevoked")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "TokenRevoked"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let token = match Vec::<starknet::core::types::FieldElement>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "token", "TokenRevoked", e
                    ))
                }
            };
            data_offset +=
                Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&token);
            return Ok(SessionComponentEvent::TokenRevoked(TokenRevoked { token }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum WebauthnComponentEvent {}
impl cainome::cairo_serde::CairoSerde for WebauthnComponentEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        match __rust {
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __index: u128 = __felts[__offset].try_into().unwrap();
        match __index as usize {
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "WebauthnComponentEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for WebauthnComponentEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Event {
    OwnerAdded(OwnerAdded),
    OwnerRemoved(OwnerRemoved),
    SessionEvent(SessionComponentEvent),
    WebauthnEvent(WebauthnComponentEvent),
}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Event::OwnerAdded(val) => OwnerAdded::cairo_serialized_size(val) + 1,
            Event::OwnerRemoved(val) => OwnerRemoved::cairo_serialized_size(val) + 1,
            Event::SessionEvent(val) => SessionComponentEvent::cairo_serialized_size(val) + 1,
            Event::WebauthnEvent(val) => WebauthnComponentEvent::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        match __rust {
            Event::OwnerAdded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(OwnerAdded::cairo_serialize(val));
                temp
            }
            Event::OwnerRemoved(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(OwnerRemoved::cairo_serialize(val));
                temp
            }
            Event::SessionEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(SessionComponentEvent::cairo_serialize(val));
                temp
            }
            Event::WebauthnEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(WebauthnComponentEvent::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __index: u128 = __felts[__offset].try_into().unwrap();
        match __index as usize {
            0usize => Ok(Event::OwnerAdded(OwnerAdded::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            1usize => Ok(Event::OwnerRemoved(OwnerRemoved::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            2usize => Ok(Event::SessionEvent(
                SessionComponentEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            3usize => Ok(Event::WebauthnEvent(
                WebauthnComponentEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "Event"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for Event {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnerAdded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnerAdded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let new_owner_guid = match starknet::core::types::FieldElement::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "new_owner_guid", "OwnerAdded", e
                    ))
                }
            };
            data_offset +=
                starknet::core::types::FieldElement::cairo_serialized_size(&new_owner_guid);
            return Ok(Event::OwnerAdded(OwnerAdded { new_owner_guid }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnerRemoved")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnerRemoved"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let removed_owner_guid = match starknet::core::types::FieldElement::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "removed_owner_guid", "OwnerRemoved", e
                    ))
                }
            };
            data_offset +=
                starknet::core::types::FieldElement::cairo_serialized_size(&removed_owner_guid);
            return Ok(Event::OwnerRemoved(OwnerRemoved { removed_owner_guid }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("SessionEvent")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "SessionEvent"))
        {
            let selector = event.keys[1];
            if selector
                == starknet::core::utils::get_selector_from_name("TokenRevoked")
                    .unwrap_or_else(|_| panic!("Invalid selector for {}", "TokenRevoked"))
            {
                let mut key_offset = 1 + 1;
                let mut data_offset = 0;
                let token = match Vec::<starknet::core::types::FieldElement>::cairo_deserialize(
                    &event.data,
                    data_offset,
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "token", "TokenRevoked", e
                        ))
                    }
                };
                data_offset +=
                    Vec::<starknet::core::types::FieldElement>::cairo_serialized_size(&token);
                return Ok(Event::SessionEvent(SessionComponentEvent::TokenRevoked(
                    TokenRevoked { token },
                )));
            };
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("WebauthnEvent")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "WebauthnEvent"))
        {}
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl<A: starknet::accounts::ConnectedAccount + Sync> CartridgeAccount<A> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate_deploy__(
        &self,
        class_hash: &starknet::core::types::FieldElement,
        contract_address_salt: &starknet::core::types::FieldElement,
        _public_key: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            class_hash,
        ));
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            contract_address_salt,
        ));
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            _public_key,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("__validate_deploy__"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate_declare__(
        &self,
        class_hash: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            class_hash,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("__validate_declare__"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn validate_session(
        &self,
        public_key: &starknet::core::types::FieldElement,
        signature: &SessionSignature,
        calls: &Vec<Call>,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            public_key,
        ));
        __calldata.extend(SessionSignature::cairo_serialize(signature));
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("validate_session"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn validate_session_serialized(
        &self,
        public_key: &starknet::core::types::FieldElement,
        signature: &Vec<starknet::core::types::FieldElement>,
        calls: &Vec<Call>,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            public_key,
        ));
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            signature,
        ));
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("validate_session_serialized"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn compute_proof(
        &self,
        calls: &Vec<Call>,
        position: &u64,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, Vec<starknet::core::types::FieldElement>>
    {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        __calldata.extend(u64::cairo_serialize(position));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("compute_proof"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn compute_root(
        &self,
        call: &Call,
        proof: &Vec<starknet::core::types::FieldElement>,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Call::cairo_serialize(call));
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            proof,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("compute_root"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn compute_session_hash(
        &self,
        unsigned_signature: &SessionSignature,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(SessionSignature::cairo_serialize(unsigned_signature));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("compute_session_hash"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __execute__(
        &self,
        calls: &Vec<Call>,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, Vec<Vec<starknet::core::types::FieldElement>>>
    {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("__execute__"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate__(
        &self,
        calls: &Vec<Call>,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("__validate__"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_valid_signature(
        &self,
        hash: &starknet::core::types::FieldElement,
        signature: &Vec<starknet::core::types::FieldElement>,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(hash));
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            signature,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_valid_signature"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_public_key(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_public_key"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_webauthn_pub_key(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, Option<WebauthnPubKey>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_webauthn_pub_key"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn verify_webauthn_signer(
        &self,
        signature: &WebauthnSignature,
        tx_hash: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(WebauthnSignature::cairo_serialize(signature));
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            tx_hash,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("verify_webauthn_signer"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn verify_webauthn_signer_serialized(
        &self,
        signature: &Vec<starknet::core::types::FieldElement>,
        tx_hash: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            signature,
        ));
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            tx_hash,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("verify_webauthn_signer_serialized"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn revoke_session_getcall(
        &self,
        token: &Vec<starknet::core::types::FieldElement>,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            token,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("revoke_session"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn revoke_session(
        &self,
        token: &Vec<starknet::core::types::FieldElement>,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            token,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("revoke_session"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_public_key_getcall(
        &self,
        new_public_key: &starknet::core::types::FieldElement,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            new_public_key,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_public_key"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn set_public_key(
        &self,
        new_public_key: &starknet::core::types::FieldElement,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            new_public_key,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_public_key"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_webauthn_pub_key_getcall(
        &self,
        public_key: &WebauthnPubKey,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(WebauthnPubKey::cairo_serialize(public_key));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_webauthn_pub_key"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn set_webauthn_pub_key(
        &self,
        public_key: &WebauthnPubKey,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(WebauthnPubKey::cairo_serialize(public_key));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_webauthn_pub_key"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
}
impl<P: starknet::providers::Provider + Sync> CartridgeAccountReader<P> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate_deploy__(
        &self,
        class_hash: &starknet::core::types::FieldElement,
        contract_address_salt: &starknet::core::types::FieldElement,
        _public_key: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            class_hash,
        ));
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            contract_address_salt,
        ));
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            _public_key,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("__validate_deploy__"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate_declare__(
        &self,
        class_hash: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            class_hash,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("__validate_declare__"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn validate_session(
        &self,
        public_key: &starknet::core::types::FieldElement,
        signature: &SessionSignature,
        calls: &Vec<Call>,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            public_key,
        ));
        __calldata.extend(SessionSignature::cairo_serialize(signature));
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("validate_session"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn validate_session_serialized(
        &self,
        public_key: &starknet::core::types::FieldElement,
        signature: &Vec<starknet::core::types::FieldElement>,
        calls: &Vec<Call>,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            public_key,
        ));
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            signature,
        ));
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("validate_session_serialized"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn compute_proof(
        &self,
        calls: &Vec<Call>,
        position: &u64,
    ) -> cainome::cairo_serde::call::FCall<P, Vec<starknet::core::types::FieldElement>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        __calldata.extend(u64::cairo_serialize(position));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("compute_proof"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn compute_root(
        &self,
        call: &Call,
        proof: &Vec<starknet::core::types::FieldElement>,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Call::cairo_serialize(call));
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            proof,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("compute_root"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn compute_session_hash(
        &self,
        unsigned_signature: &SessionSignature,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(SessionSignature::cairo_serialize(unsigned_signature));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("compute_session_hash"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __execute__(
        &self,
        calls: &Vec<Call>,
    ) -> cainome::cairo_serde::call::FCall<P, Vec<Vec<starknet::core::types::FieldElement>>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("__execute__"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate__(
        &self,
        calls: &Vec<Call>,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("__validate__"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_valid_signature(
        &self,
        hash: &starknet::core::types::FieldElement,
        signature: &Vec<starknet::core::types::FieldElement>,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(hash));
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            signature,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_valid_signature"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_public_key(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_public_key"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_webauthn_pub_key(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, Option<WebauthnPubKey>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_webauthn_pub_key"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn verify_webauthn_signer(
        &self,
        signature: &WebauthnSignature,
        tx_hash: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(WebauthnSignature::cairo_serialize(signature));
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            tx_hash,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("verify_webauthn_signer"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn verify_webauthn_signer_serialized(
        &self,
        signature: &Vec<starknet::core::types::FieldElement>,
        tx_hash: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<starknet::core::types::FieldElement>::cairo_serialize(
            signature,
        ));
        __calldata.extend(starknet::core::types::FieldElement::cairo_serialize(
            tx_hash,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("verify_webauthn_signer_serialized"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
}
