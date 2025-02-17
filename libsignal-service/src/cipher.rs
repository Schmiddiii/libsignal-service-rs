use std::convert::TryFrom;

use block_modes::block_padding::{Iso7816, Padding};
use libsignal_protocol::{
    message_decrypt_prekey, message_decrypt_signal, message_encrypt,
    sealed_sender_decrypt, sealed_sender_encrypt, CiphertextMessageType,
    IdentityKeyStore, PreKeySignalMessage, PreKeyStore, ProtocolAddress,
    PublicKey, SealedSenderDecryptionResult, SenderCertificate, SessionStore,
    SignalMessage, SignalProtocolError, SignedPreKeyStore,
};
use prost::Message;
use rand::{CryptoRng, Rng};
use uuid::Uuid;

use crate::{
    content::{Content, Metadata},
    envelope::Envelope,
    push_service::ServiceError,
    sender::OutgoingPushMessage,
    ServiceAddress,
};

/// Decrypts incoming messages and encrypts outgoing messages.
///
/// Equivalent of SignalServiceCipher in Java.
#[derive(Clone)]
pub struct ServiceCipher<S, I, SP, P, R> {
    session_store: S,
    identity_key_store: I,
    signed_pre_key_store: SP,
    pre_key_store: P,
    csprng: R,
    trust_root: PublicKey,
}

impl<S, I, SP, P, R> ServiceCipher<S, I, SP, P, R>
where
    S: SessionStore + Clone,
    I: IdentityKeyStore + Clone,
    SP: SignedPreKeyStore + Clone,
    P: PreKeyStore + Clone,
    R: Rng + CryptoRng + Clone,
{
    pub fn new(
        session_store: S,
        identity_key_store: I,
        signed_pre_key_store: SP,
        pre_key_store: P,
        csprng: R,
        trust_root: PublicKey,
    ) -> Self {
        Self {
            session_store,
            identity_key_store,
            signed_pre_key_store,
            pre_key_store,
            csprng,
            trust_root,
        }
    }

    /// Opens ("decrypts") an envelope.
    ///
    /// Envelopes may be empty, in which case this method returns `Ok(None)`
    pub async fn open_envelope(
        &mut self,
        envelope: Envelope,
    ) -> Result<Option<Content>, ServiceError> {
        if envelope.legacy_message.is_some() {
            let plaintext = self.decrypt(&envelope).await?;
            let message =
                crate::proto::DataMessage::decode(plaintext.data.as_slice())?;
            Ok(Some(Content::from_body(message, plaintext.metadata)))
        } else if envelope.content.is_some() {
            let plaintext = self.decrypt(&envelope).await?;
            let message =
                crate::proto::Content::decode(plaintext.data.as_slice())?;
            Ok(Content::from_proto(message, plaintext.metadata))
        } else {
            Ok(None)
        }
    }

    /// Equivalent of decrypt(Envelope, ciphertext)
    ///
    /// Triage of legacy messages happens inside this method, as opposed to the
    /// Java implementation, because it makes the borrow checker and the
    /// author happier.
    async fn decrypt(
        &mut self,
        envelope: &Envelope,
    ) -> Result<Plaintext, ServiceError> {
        let ciphertext = if let Some(msg) = envelope.legacy_message.as_ref() {
            msg
        } else if let Some(msg) = envelope.content.as_ref() {
            msg
        } else {
            return Err(ServiceError::InvalidFrameError {
                reason:
                    "Envelope should have either a legacy message or content."
                        .into(),
            });
        };

        use crate::proto::envelope::Type;
        let plaintext = match (
            envelope.r#type(),
            envelope.source_uuid.as_ref(),
            envelope.source_device,
        ) {
            (Type::PrekeyBundle, _, _) => {
                let sender = get_preferred_protocol_address(
                    &self.session_store,
                    &envelope.source_address(),
                    envelope.source_device(),
                )
                .await?;
                let metadata = Metadata {
                    sender: envelope.source_address(),
                    sender_device: envelope.source_device(),
                    timestamp: envelope.timestamp(),
                    needs_receipt: false,
                };

                let mut data = message_decrypt_prekey(
                    &PreKeySignalMessage::try_from(&ciphertext[..]).unwrap(),
                    &sender,
                    &mut self.session_store,
                    &mut self.identity_key_store,
                    &mut self.pre_key_store,
                    &mut self.signed_pre_key_store,
                    &mut self.csprng,
                    None,
                )
                .await?
                .as_slice()
                .to_vec();

                let session_record = self
                    .session_store
                    .load_session(&sender, None)
                    .await?
                    .ok_or(SignalProtocolError::SessionNotFound(sender))?;

                strip_padding(session_record.session_version()?, &mut data)?;
                Plaintext { metadata, data }
            },
            (Type::Ciphertext, _, _) => {
                let sender = get_preferred_protocol_address(
                    &self.session_store,
                    &envelope.source_address(),
                    envelope.source_device(),
                )
                .await?;
                let metadata = Metadata {
                    sender: envelope.source_address(),
                    sender_device: envelope.source_device(),
                    timestamp: envelope.timestamp(),
                    needs_receipt: false,
                };

                let mut data = message_decrypt_signal(
                    &SignalMessage::try_from(&ciphertext[..])?,
                    &sender,
                    &mut self.session_store,
                    &mut self.identity_key_store,
                    &mut self.csprng,
                    None,
                )
                .await?
                .as_slice()
                .to_vec();

                let session_record = self
                    .session_store
                    .load_session(&sender, None)
                    .await?
                    .ok_or(SignalProtocolError::SessionNotFound(sender))?;

                strip_padding(session_record.session_version()?, &mut data)?;
                Plaintext { metadata, data }
            },
            (Type::UnidentifiedSender, Some(uuid), Some(source_device)) => {
                let SealedSenderDecryptionResult {
                    sender_uuid,
                    sender_e164,
                    device_id,
                    message,
                } = sealed_sender_decrypt(
                    ciphertext,
                    &self.trust_root,
                    envelope.timestamp(),
                    envelope.source_e164.clone(),
                    uuid.clone(),
                    source_device,
                    &mut self.identity_key_store,
                    &mut self.session_store,
                    &mut self.pre_key_store,
                    &mut self.signed_pre_key_store,
                    None,
                )
                .await?;

                let sender = ServiceAddress {
                    phonenumber: sender_e164
                        .and_then(|n| phonenumber::parse(None, n).ok()),
                    uuid: Some(Uuid::parse_str(&sender_uuid).map_err(
                        |_| {
                            SignalProtocolError::InvalidSealedSenderMessage(
                                "invalid sender UUID".to_string(),
                            )
                        },
                    )?),
                    relay: None,
                };
                let metadata = Metadata {
                    sender,
                    sender_device: device_id,
                    timestamp: envelope.timestamp(),
                    needs_receipt: false,
                };
                Plaintext {
                    metadata,
                    data: message,
                }
            },
            _ => {
                // else
                return Err(ServiceError::InvalidFrameError {
                    reason: format!(
                        "Envelope has unknown type {:?}.",
                        envelope.r#type()
                    ),
                });
            },
        };
        Ok(plaintext)
    }

    pub(crate) async fn encrypt(
        &mut self,
        address: &ProtocolAddress,
        unindentified_access: Option<&SenderCertificate>,
        content: &[u8],
    ) -> Result<OutgoingPushMessage, ServiceError> {
        if let Some(unindentified_access) = unindentified_access {
            let session_record = self
                .session_store
                .load_session(address, None)
                .await?
                .ok_or_else(|| {
                    SignalProtocolError::SessionNotFound(address.clone())
                })?;

            let destination_registration_id =
                session_record.remote_registration_id()?;

            let message = sealed_sender_encrypt(
                address,
                unindentified_access,
                content,
                &mut self.session_store,
                &mut self.identity_key_store,
                None,
                &mut self.csprng,
            )
            .await?;

            use crate::proto::envelope::Type;
            Ok(OutgoingPushMessage {
                r#type: Type::UnidentifiedSender as u32,
                destination_device_id: address.device_id(),
                destination_registration_id,
                content: base64::encode(message),
            })
        } else {
            let session_record = self
                .session_store
                .load_session(address, None)
                .await?
                .ok_or_else(|| {
                    SignalProtocolError::SessionNotFound(address.clone())
                })?;

            let padded_content =
                add_padding(session_record.session_version()?, content)?;

            let message = message_encrypt(
                &padded_content,
                address,
                &mut self.session_store,
                &mut self.identity_key_store,
                None,
            )
            .await?;

            let destination_registration_id =
                session_record.remote_registration_id()?;

            let body = base64::encode(message.serialize());

            use crate::proto::envelope::Type;
            let message_type = match message.message_type() {
                CiphertextMessageType::PreKey => Type::PrekeyBundle,
                CiphertextMessageType::Whisper => Type::Ciphertext,
                t => panic!("Bad type: {:?}", t),
            } as u32;
            Ok(OutgoingPushMessage {
                r#type: message_type,
                destination_device_id: address.device_id(),
                destination_registration_id,
                content: body,
            })
        }
    }
}

struct Plaintext {
    metadata: Metadata,
    data: Vec<u8>,
}

#[allow(clippy::comparison_chain)]
fn add_padding(version: u32, contents: &[u8]) -> Result<Vec<u8>, ServiceError> {
    if version < 2 {
        Err(ServiceError::InvalidFrameError {
            reason: format!("Unknown version {}", version),
        })
    } else if version == 2 {
        Ok(contents.to_vec())
    } else {
        let message_length = contents.len();
        let message_length_with_terminator = contents.len() + 1;
        let mut message_part_count = message_length_with_terminator / 160;
        if message_length_with_terminator % 160 != 0 {
            message_part_count += 1;
        }

        let message_length_with_padding = message_part_count * 160;

        let mut buffer = vec![0u8; message_length_with_padding];
        buffer[..message_length].copy_from_slice(contents);
        Iso7816::pad_block(&mut buffer, message_length).map_err(|e| {
            ServiceError::InvalidFrameError {
                reason: format!("Invalid message padding: {:?}", e),
            }
        })?;
        Ok(buffer)
    }
}

#[allow(clippy::comparison_chain)]
fn strip_padding(
    version: u32,
    contents: &mut Vec<u8>,
) -> Result<(), ServiceError> {
    if version < 2 {
        Err(ServiceError::InvalidFrameError {
            reason: format!("Unknown version {}", version),
        })
    } else if version == 2 {
        Ok(())
    } else {
        let new_length = Iso7816::unpad(contents)
            .map_err(|e| ServiceError::InvalidFrameError {
                reason: format!("Invalid message padding: {:?}", e),
            })?
            .len();
        contents.resize(new_length, 0);
        Ok(())
    }
}

/// Equivalent of `SignalServiceCipher::getPreferredProtocolAddress`
pub async fn get_preferred_protocol_address<S: SessionStore>(
    session_store: &S,
    address: &ServiceAddress,
    device_id: u32,
) -> Result<ProtocolAddress, libsignal_protocol::error::SignalProtocolError> {
    if let Some(ref uuid) = address.uuid {
        let address = ProtocolAddress::new(uuid.to_string(), device_id);
        if session_store.load_session(&address, None).await?.is_some() {
            return Ok(address);
        }
    }
    if let Some(e164) = address.e164() {
        let address = ProtocolAddress::new(e164, device_id);
        if session_store.load_session(&address, None).await?.is_some() {
            return Ok(address);
        }
        if cfg!(feature = "prefer-e164") {
            log::warn!("prefer-e164 triggered.  This is a legacy feature and shouldn't be used for new applications.");
            return Ok(address);
        }
    }
    if cfg!(feature = "prefer-e164") {
        panic!(
            "{:?}:{} does not have a e164 associated, falling back to UUID.",
            address, device_id
        );
    }

    Ok(ProtocolAddress::new(address.identifier(), device_id))
}
