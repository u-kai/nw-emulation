pub trait TlsClient {
    fn client_hello(&self) -> (CryptoAlgorisumParameter, ClientRandom);
    fn valid_server_certificate(&self, cert: ServerCertificate) -> bool;
    fn certificate(&self) -> ClientCertificate;
    fn client_key_exchange(&mut self) -> PreMasterSecret;
    fn client_verify(&self) -> DigitalSignature;
    fn change_cipher_spec(&mut self) -> MasterSecret;
    fn finished(&self) -> ValidateData;
    fn valid_data(&self, data: ValidateData) -> bool;
}

pub struct ValidateData;
pub struct MasterSecret;
pub struct DigitalSignature;
pub struct PreMasterSecret;

pub struct ClientCertificate {
    cert: String,
}
pub struct CryptoAlgorisumParameter {
    protocol_v: String,
    crypto_suite: CryptoSuite,
}
pub struct ClientRandom;

pub struct CryptoSuite {
    server_cert_algo: String,
    key_exchange_algo: String,
    messeage_crypto_algo: String,
    use_mode: String,
    hash_algo: String,
}
pub trait TlsServer {
    fn server_hello(&self) -> (CryptoAlgorisumParameter, ServerRandom);
    fn certificate(&self) -> ServerCertificate;
    fn certificate_request(&self) -> ();
    fn server_done(&self) -> ();
    fn valid_client_signater(&self, signature: DigitalSignature) -> bool;
    fn change_cipher_spec(&mut self) -> ();
    fn finished(&self) -> ValidateData;
}
pub struct ServerRandom(u32);

pub struct ServerCertificate {
    cert: String,
}
