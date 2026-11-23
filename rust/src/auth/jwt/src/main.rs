use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, decode_header, encode,
    jwk::{Jwk, JwkSet},
};
use serde::{Deserialize, Serialize};

const ISSUER: &str = "piyo";
const KEY_ID: &str = "piyo-key";

const PRIVATE_KEY: &str = include_str!("../private.pem");
const PUBLIC_KEY: &str = include_str!("../public.pem");

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iss: String,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    SystemTime(std::time::SystemTimeError),
    JsonWebToken(jsonwebtoken::errors::Error),
    NotFoundKid,
    NoMatchJwk,
}

fn issue_jwt(subject: &str) -> Result<String, Error> {
    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(Error::SystemTime)?
        .as_secs() as usize
        + 3600;
    let claims = Claims {
        sub: subject.to_owned(),
        exp: expires_at,
        iss: ISSUER.to_owned(),
    };
    let key = EncodingKey::from_rsa_pem(PRIVATE_KEY.as_bytes()).map_err(Error::JsonWebToken)?;
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(KEY_ID.to_owned());

    encode(&header, &claims, &key).map_err(Error::JsonWebToken)
}

fn create_jwks() -> Result<JwkSet, Error> {
    let decoding_key =
        DecodingKey::from_rsa_pem(PUBLIC_KEY.as_bytes()).map_err(Error::JsonWebToken)?;
    let mut jwk = Jwk::from_decoding_key(&decoding_key, Some(Algorithm::RS256))
        .map_err(Error::JsonWebToken)?;
    jwk.common.key_id = Some(KEY_ID.to_owned());

    Ok(JwkSet { keys: vec![jwk] })
}

fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let header = decode_header(token).map_err(Error::JsonWebToken)?;
    let kid = header.kid.ok_or(Error::NotFoundKid)?;
    let jwks = create_jwks()?;
    let jwk = jwks.find(&kid).ok_or(Error::NoMatchJwk)?;
    let key = DecodingKey::from_jwk(jwk).map_err(Error::JsonWebToken)?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[ISSUER]);

    Ok(decode::<Claims>(token, &key, &validation)
        .map_err(Error::JsonWebToken)?
        .claims)
}

fn main() {
    let token = issue_jwt("puyo").unwrap();
    println!("JWT: {token}");

    let claims = decode_jwt(&token).unwrap();
    println!("Decoded claims: {claims:?}");
}
