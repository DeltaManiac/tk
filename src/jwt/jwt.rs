use jsonwebtoken::decode_header;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    iat: u64,
}

/// Holds the information for a jwt Token
#[derive(Debug)]
pub struct JWTToken {
    jwt_raw: Option<String>,
    jwt_decoded: Option<String>,
}

impl JWTToken {
    pub fn new(jwt_raw: Option<String>) -> Self {
        let token = decode_header(jwt_raw.as_ref().unwrap().as_ref());
        dbg!(token);
        let mut v = Validation::new(Algorithm::HS256);
        v.insecure_disable_signature_validation();
        let token_message = decode::<Claims>(
            &jwt_raw.as_ref().unwrap(),
            &DecodingKey::from_secret("".as_ref()),
            &v,
        );
        dbg!(token_message);
        Self {
            jwt_raw,
            jwt_decoded: None,
        }
    }
}

impl Default for JWTToken {
    fn default() -> Self {
        Self {
            jwt_raw: Default::default(),
            jwt_decoded: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_jwt() {
        let token_str = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0";
        let token_str ="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        // let token_str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJjbGFpbXMiOlsiYXNkYXNmIl19.pJq-hWep2O8PjMlP6Da42FiEpAD-LOY7LWsZBj9B3zw";
        let t = JWTToken::new(Some(token_str.to_string().clone()));
    }
}
