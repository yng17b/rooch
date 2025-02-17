module rooch_framework::ecdsa_k1 {

   /// Error if the signature is invalid.
   const EInvalidSignature: u64 = 0;

   /// Error if the public key is invalid.
   const EInvalidPubKey: u64 = 1;

   /// Hash function name that are valid for ecrecover and verify.
   const KECCAK256: u8 = 0;
   const SHA256: u8 = 1;

   /// @param signature: A 64-bytes signature in form (r, s) that is signed using
   /// Ecdsa. This is an non-recoverable signature without recovery id.
   /// @param public_key: A 33-bytes public key that is used to sign messages.
   /// @param msg: The message that the signature is signed against.
   /// @param hash: The hash function used to hash the message when signing.
   ///
   /// If the signature is valid to the pubkey and hashed message, return true. Else false.
   public native fun verify(
      signature: &vector<u8>,
      public_key: &vector<u8>,
      msg: &vector<u8>,
      hash: u8
   ): bool;

   #[test]
   fun test_verify_success() {
      let msg = x"00010203";
      let pubkey = x"033e99a541db69bd32040dfe5037fbf5210dafa8151a71e21c5204b05d95ce0a62";
      let sig = x"416a21d50b3c838328d4f03213f8ef0c3776389a972ba1ecd37b56243734eba208ea6aaa6fc076ad7accd71d355f693a6fe54fe69b3c168eace9803827bc9046";
      let result = verify(&sig, &pubkey, &msg, SHA256);
      assert!(result, 0);
   }

   #[test]
   #[expected_failure(abort_code = EInvalidSignature)]
   fun test_verify_fails_invalid_sig() {
      let msg = x"00010203";
      let pubkey = x"033e99a541db69bd32040dfe5037fbf5210dafa8151a71e21c5204b05d95ce0a62";
      let sig = x"";
      verify(&sig, &pubkey, &msg, SHA256);
   }

   #[test]
   #[expected_failure(abort_code = EInvalidPubKey)]
   fun test_verify_fails_invalid_pubkey() {
      let msg = x"00010203";
      let pubkey = x"";
      let sig = x"416a21d50b3c838328d4f03213f8ef0c3776389a972ba1ecd37b56243734eba208ea6aaa6fc076ad7accd71d355f693a6fe54fe69b3c168eace9803827bc9046";
      verify(&sig, &pubkey, &msg, SHA256);
   }
}
