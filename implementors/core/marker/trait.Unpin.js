(function() {var implementors = {};
implementors["libsignal_service"] = [{"text":"impl&lt;Service&gt; Unpin for AccountManager&lt;Service&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Service: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for TrustStore","synthetic":true,"types":[]},{"text":"impl Unpin for ServiceAddress","synthetic":true,"types":[]},{"text":"impl Unpin for AttachmentCipherError","synthetic":true,"types":[]},{"text":"impl Unpin for ServiceCipher","synthetic":true,"types":[]},{"text":"impl Unpin for ServiceConfiguration","synthetic":true,"types":[]},{"text":"impl Unpin for Credentials","synthetic":true,"types":[]},{"text":"impl Unpin for SignalServers","synthetic":true,"types":[]},{"text":"impl Unpin for Reaction","synthetic":true,"types":[]},{"text":"impl Unpin for AttachmentPointer","synthetic":true,"types":[]},{"text":"impl Unpin for CallMessage","synthetic":true,"types":[]},{"text":"impl Unpin for DataMessage","synthetic":true,"types":[]},{"text":"impl Unpin for ReceiptMessage","synthetic":true,"types":[]},{"text":"impl Unpin for SyncMessage","synthetic":true,"types":[]},{"text":"impl Unpin for TypingMessage","synthetic":true,"types":[]},{"text":"impl Unpin for Metadata","synthetic":true,"types":[]},{"text":"impl Unpin for Content","synthetic":true,"types":[]},{"text":"impl Unpin for Flags","synthetic":true,"types":[]},{"text":"impl Unpin for Flags","synthetic":true,"types":[]},{"text":"impl Unpin for Type","synthetic":true,"types":[]},{"text":"impl Unpin for ContentBody","synthetic":true,"types":[]},{"text":"impl Unpin for Sent","synthetic":true,"types":[]},{"text":"impl Unpin for Contacts","synthetic":true,"types":[]},{"text":"impl Unpin for Groups","synthetic":true,"types":[]},{"text":"impl Unpin for Blocked","synthetic":true,"types":[]},{"text":"impl Unpin for Request","synthetic":true,"types":[]},{"text":"impl Unpin for Read","synthetic":true,"types":[]},{"text":"impl Unpin for Configuration","synthetic":true,"types":[]},{"text":"impl Unpin for StickerPackOperation","synthetic":true,"types":[]},{"text":"impl Unpin for ViewOnceOpen","synthetic":true,"types":[]},{"text":"impl Unpin for FetchLatest","synthetic":true,"types":[]},{"text":"impl Unpin for Keys","synthetic":true,"types":[]},{"text":"impl Unpin for MessageRequestResponse","synthetic":true,"types":[]},{"text":"impl Unpin for UnidentifiedDeliveryStatus","synthetic":true,"types":[]},{"text":"impl Unpin for Type","synthetic":true,"types":[]},{"text":"impl Unpin for Type","synthetic":true,"types":[]},{"text":"impl Unpin for Type","synthetic":true,"types":[]},{"text":"impl Unpin for Type","synthetic":true,"types":[]},{"text":"impl Unpin for Envelope","synthetic":true,"types":[]},{"text":"impl Unpin for EnvelopeEntity","synthetic":true,"types":[]},{"text":"impl Unpin for WebSocketMessage","synthetic":true,"types":[]},{"text":"impl Unpin for WebSocketRequestMessage","synthetic":true,"types":[]},{"text":"impl Unpin for WebSocketResponseMessage","synthetic":true,"types":[]},{"text":"impl Unpin for PanicingWebSocketService","synthetic":true,"types":[]},{"text":"impl Unpin for WebSocketStreamItem","synthetic":true,"types":[]},{"text":"impl Unpin for Type","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Unpin for Attachment&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for Group","synthetic":true,"types":[]},{"text":"impl Unpin for Message","synthetic":true,"types":[]},{"text":"impl Unpin for PreKeyEntity","synthetic":true,"types":[]},{"text":"impl Unpin for SignedPreKeyEntity","synthetic":true,"types":[]},{"text":"impl Unpin for SignedPreKey","synthetic":true,"types":[]},{"text":"impl Unpin for PreKeyState","synthetic":true,"types":[]},{"text":"impl Unpin for ProvisioningCipher","synthetic":true,"types":[]},{"text":"impl Unpin for ProvisioningError","synthetic":true,"types":[]},{"text":"impl Unpin for ProvisioningStep","synthetic":true,"types":[]},{"text":"impl Unpin for DeviceId","synthetic":true,"types":[]},{"text":"impl Unpin for ConfirmDeviceMessage","synthetic":true,"types":[]},{"text":"impl Unpin for ConfirmCodeMessage","synthetic":true,"types":[]},{"text":"impl Unpin for DeviceCapabilities","synthetic":true,"types":[]},{"text":"impl Unpin for ProfileKey","synthetic":true,"types":[]},{"text":"impl Unpin for PreKeyStatus","synthetic":true,"types":[]},{"text":"impl Unpin for ConfirmCodeResponse","synthetic":true,"types":[]},{"text":"impl Unpin for PreKeyResponse","synthetic":true,"types":[]},{"text":"impl Unpin for WhoAmIResponse","synthetic":true,"types":[]},{"text":"impl Unpin for PreKeyResponseItem","synthetic":true,"types":[]},{"text":"impl Unpin for MismatchedDevices","synthetic":true,"types":[]},{"text":"impl Unpin for StaleDevices","synthetic":true,"types":[]},{"text":"impl Unpin for SmsVerificationCodeResponse","synthetic":true,"types":[]},{"text":"impl Unpin for VoiceVerificationCodeResponse","synthetic":true,"types":[]},{"text":"impl Unpin for ServiceError","synthetic":true,"types":[]},{"text":"impl&lt;Service&gt; Unpin for MessageReceiver&lt;Service&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Service: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for MessageReceiverError","synthetic":true,"types":[]},{"text":"impl Unpin for UnidentifiedAccessPair","synthetic":true,"types":[]},{"text":"impl Unpin for UnidentifiedAccess","synthetic":true,"types":[]},{"text":"impl Unpin for UnidentifiedSenderMessageContent","synthetic":true,"types":[]},{"text":"impl Unpin for SenderCertificate","synthetic":true,"types":[]},{"text":"impl Unpin for ServerCertificate","synthetic":true,"types":[]},{"text":"impl Unpin for CertificateValidator","synthetic":true,"types":[]},{"text":"impl Unpin for SealedSessionError","synthetic":true,"types":[]},{"text":"impl Unpin for MacError","synthetic":true,"types":[]},{"text":"impl Unpin for OutgoingPushMessage","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Unpin for OutgoingPushMessages&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Unpin for SendMessageResponse","synthetic":true,"types":[]},{"text":"impl&lt;Service&gt; Unpin for MessageSender&lt;Service&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Service: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for MessageSenderError","synthetic":true,"types":[]},{"text":"impl&lt;'pin, WS:&nbsp;WebSocketService&gt; Unpin for MessagePipe&lt;WS&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__MessagePipe&lt;'pin, WS&gt;: Unpin,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'pin, WS:&nbsp;WebSocketService&gt; Unpin for ProvisioningPipe&lt;WS&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__ProvisioningPipe&lt;'pin, WS&gt;: Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libsignal_service_actix"] = [{"text":"impl Unpin for AwcPushService","synthetic":true,"types":[]},{"text":"impl Unpin for AwcWebSocket","synthetic":true,"types":[]},{"text":"impl Unpin for AwcWebSocketError","synthetic":true,"types":[]},{"text":"impl Unpin for SecondaryDeviceProvisioning","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()