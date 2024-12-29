use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::{Public, public};
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use crate::entities::error::AuthError;
use crate::utils::settings::AuthSettings;

pub trait IntoClaims {
    type Err;
    fn try_into_claims(self) -> Result<Claims, Self::Err>;
}


pub struct TokenString(pub String);

impl IntoClaims for TokenString {
    
    type Err = AuthError;
    fn try_into_claims(self) -> Result<Claims, Self::Err> {
        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token = UntrustedToken::<Public, V4>::try_from(&self.0).map_err(|_| AuthError::InvalidToken)?;

        let trusted_token = public::verify(&AuthSettings::get_public_key(), &untrusted_token, &validation_rules, None, Some(b"implicit assertion")).map_err(|_| AuthError::WrongCredentials)?;

        if let Some(claims) = trusted_token.payload_claims() {
            return Ok(claims.to_owned())
        }
        
        Err(AuthError::WrongCredentials)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::settings::AuthSettings;
    use crate::utils::validate_token::{IntoClaims, TokenString};

    /// Expire in July 2044
    /// Token contain identifier, username : "Juliana McLaughlin" and role moderator
    const TOKEN_USERNAME: &'static str = "Juliana McLaughlin";
    const ACCESS_TOKEN_IDENTIFIER: &'static str = "a7cbd03a-68f1-4ec9-a17f-f54e71cd2dee";
    const ACCESS_TOKEN: &'static str = "v4.public.eyJ1c2VybmFtZSI6Ikp1bGlhbmEgTWNMYXVnaGxpbiIsInJvbGUiOiJNb2RlcmF0b3IiLCJuYmYiOiIyMDI0LTA3LTA1VDE3OjI4OjMzLjc5MDAzOVoiLCJpYXQiOiIyMDI0LTA3LTA1VDE3OjI4OjMzLjc5MDAzOVoiLCJzdWIiOiJhY2Nlc3MiLCJqdGkiOiJhN2NiZDAzYS02OGYxLTRlYzktYTE3Zi1mNTRlNzFjZDJkZWUiLCJleHAiOiIyMDQ0LTA2LTMwVDE3OjI4OjMzLjc5MDAwNyswMDowMCJ9Ryq_DPM3yPsvPwa8sf17cgq9YOAlJTFijoafYSKExiQuzp4NUqyBsTwkF39SI40mh1jNgmNICnlJrkFj15OSAw";
    const ROLES_TOKEN: &'static str = "Moderator";
    /// Token contain identifier, username : "Juliana McLaughlin"
    const REFRESH_TOKEN_IDENTIFIER: &'static str = "94377ef3-e313-4e84-a35b-be29a9ef3fdf";
    const REFRESH_TOKEN: &'static str = "v4.public.eyJ1c2VybmFtZSI6Ikp1bGlhbmEgTWNMYXVnaGxpbiIsImV4cCI6IjIwNDQtMDYtMzBUMTc6Mjg6MzMuNzkxMDkzKzAwOjAwIiwiaWF0IjoiMjAyNC0wNy0wNVQxNzoyODozMy43OTEwOTRaIiwic3ViIjoicmVmcmVzaCIsImp0aSI6Ijk0Mzc3ZWYzLWUzMTMtNGU4NC1hMzViLWJlMjlhOWVmM2ZkZiIsIm5iZiI6IjIwMjQtMDctMDVUMTc6Mjg6MzMuNzkxMDk0WiJ9NYr6HCMzkaNC7dwzbtkmskf-36w8oqtwxGplcgGFkIqlRmy08KP7KOF7Go77TJk_oe60EuKxQLT6xzdr5beMCg";

    #[test]
    fn test_access_token_string_into_claims() {
        AuthSettings::init_fake();

        let access_token = TokenString(ACCESS_TOKEN.to_string());
        let claim = access_token.try_into_claims().expect("Unable parse access token. Maybe need regenerate it (After 2044 required)");

        assert_eq!(claim.get_claim("username").unwrap().to_string().trim_matches('"'), TOKEN_USERNAME.to_string());
        assert_eq!(claim.get_claim("sub").unwrap().to_string().trim_matches('"'), "access");
        assert_eq!(claim.get_claim("jti").unwrap().to_string().trim_matches('"'), ACCESS_TOKEN_IDENTIFIER.to_string());
        assert_eq!(claim.get_claim("role").unwrap().to_string().trim_matches('"'), ROLES_TOKEN);
    }

    #[test]
    fn test_refresh_token_string_into_claims() {
        AuthSettings::init_fake();

        let refresh_token = TokenString(REFRESH_TOKEN.to_string());
        let claim = refresh_token.try_into_claims().expect("Unable parse refresh token. Maybe need regenerate it (After 2044 required)");

        assert_eq!(claim.get_claim("username").unwrap().to_string().trim_matches('"'), TOKEN_USERNAME.to_string());
        assert_eq!(claim.get_claim("sub").unwrap().to_string().trim_matches('"'), "refresh");
        assert_eq!(claim.get_claim("jti").unwrap().to_string().trim_matches('"'), REFRESH_TOKEN_IDENTIFIER.to_string());
    }
}