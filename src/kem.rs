
trait Scheme {

    fn generate_keypair(&mut self) -> (Box<dyn PublicKey>, Box<dyn PrivateKey>);

    fn encapsulate(public_key: dyn PublicKey) -> (Vec<u8>, Vec<u8>);

    fn decapsulate(private_key: dyn PrivateKey, ciphertext: Vec<u8>) -> Vec<u8>;

    fn deserialize_binary_public_key(&mut self, data: &[u8]) -> Box<dyn PublicKey>;

    fn deserialize_binary_private_key(&mut self, data: &[u8]) -> Box<dyn PrivateKey>;

    fn ciphertext_size(&self) -> usize;

    fn shared_secret_size(&self) -> usize;

    fn public_key_size(&self) -> usize;

    fn private_key_size(&self) -> usize;
}

trait PublicKey {
    fn to_vec(&self) -> Vec<u8>;
}

trait PrivateKey {
    fn to_vec(&self) -> Vec<u8>;
}
