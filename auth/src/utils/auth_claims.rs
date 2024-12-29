use std::fmt::Display;
use pasetors::claims::Claims;
use serde::{Deserialize, Serialize};
use crate::entities::{Roles, TokenType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub claim_type: TokenType,
    pub username: String,
    pub role: Option<Roles>,
    pub token_identifier: String,
}

impl Display for AuthClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}\n", self.username)
    }
}

impl AuthClaims {
    fn new_access_token(trusted_token: &Claims) -> Result<Self, ()> {
        if trusted_token.get_claim("jti").is_none() || trusted_token.get_claim("username").is_none() || trusted_token.get_claim("role").is_none() {
            return Err(());
        }

        let token_identifier = trusted_token.get_claim("jti").unwrap().to_string().trim_matches('"').to_string();
        let username = trusted_token.get_claim("username").unwrap().to_string().trim_matches('"').to_string();
        let role: Option<Roles> = trusted_token.get_claim("role").map(|value| value.to_string().trim_matches('"').to_string().parse().unwrap());

        Ok(Self {
            claim_type: TokenType::Access,
            token_identifier,
            username,
            role,
        })
    }

    fn new_refresh_token(trusted_token: &Claims) -> Result<Self, ()> {
        if trusted_token.get_claim("jti").is_none() || trusted_token.get_claim("username").is_none() {
            return Err(());
        }

        let token_identifier = trusted_token.get_claim("jti").unwrap().to_string().trim_matches('"').to_string();
        let username = trusted_token.get_claim("username").unwrap().to_string().trim_matches('"').to_string();

        Ok(Self {
            claim_type: TokenType::Refresh,
            token_identifier,
            username,
            role: None,
        })
    }
}

impl TryFrom<&Claims> for AuthClaims {
    type Error = ();
    fn try_from(trusted_token: &Claims) -> Result<Self, ()> {
        let claim_type_option: Option<TokenType> = trusted_token.get_claim("sub").map(|value| value.to_string().trim_matches('"').parse().unwrap());

        if claim_type_option.is_none() {
            return Err(());
        }

        let claim_type = claim_type_option.unwrap();

        match claim_type {
            TokenType::Refresh => Self::new_refresh_token(trusted_token),
            TokenType::Access => Self::new_access_token(trusted_token),
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use fake::faker::internet::en::Username;
    use pasetors::claims::Claims;
    use crate::entities::{Roles, TokenType};

    use crate::utils::auth_claims::AuthClaims;

    #[test]
    pub fn test_new_access_token() {
        let mut claims = Claims::new().unwrap();
        let token_id = uuid::Uuid::new_v4().to_string();
        claims.token_identifier(&token_id).expect("Unable to insert token id");
        claims.subject(&TokenType::Access.to_string()).unwrap();
        claims.add_additional("username", Username().fake::<String>()).unwrap();
        claims.add_additional("role", Faker.fake::<Roles>().to_string()).unwrap();

        let auth_claims = AuthClaims::try_from(&claims).expect("Unable convert claims to AuthClaims");

        assert_eq!(auth_claims.claim_type, TokenType::Access);
        assert_eq!(auth_claims.token_identifier, token_id);
        assert!(!auth_claims.username.is_empty());
        assert!(!auth_claims.role.is_none());
    }

    #[test]
    pub fn test_new_refresh_token() {
        let mut claims = Claims::new().unwrap();
        let token_id = "my_refresh_token_id".to_string();
        claims.token_identifier(&token_id).expect("Unable to insert token id");
        claims.subject(&TokenType::Refresh.to_string()).unwrap();
        claims.add_additional("username", Username().fake::<String>()).unwrap();
        claims.add_additional("role", Faker.fake::<Roles>().to_string()).unwrap();

        let auth_claims = AuthClaims::try_from(&claims).expect("Unable convert claims to AuthClaims");

        assert_eq!(auth_claims.claim_type, TokenType::Refresh);
        assert_eq!(auth_claims.token_identifier, token_id);
        assert!(!auth_claims.username.is_empty());
        assert!(auth_claims.role.is_none());
    }
}