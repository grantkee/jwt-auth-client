use jwks_client::error::Error as JwksError;
pub use jwks_client::keyset::KeyStore;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const auth_url: &'static str = "https://raw.githubusercontent.com/jfbilodeau/jwks-client/0.1.8/test/test-jwks.json";



/// Request context about a user's account
pub struct AccountContext {
    account: Uuid,

}

/// Attempt to validate token
pub async fn validate_token(token: &str) -> Result<AccountContext, AuthError> {

}
// use serde_derive::Deserialize;

// use jwks_client::keyset::KeyStore;

// #[tokio::main]
// async fn main() {
//     #[derive(Deserialize)]
//     pub struct MyClaims {
//         pub iss: String,
//         pub name: String,
//         pub email: String,
//     }

//     let url = "https://raw.githubusercontent.com/jfbilodeau/jwks-client/0.1.8/test/test-jwks.json";

//     let key_store = KeyStore::new_from(url.to_owned()).await.unwrap();

//     let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ.eyJuYW1lIjoiQWRhIExvdmVsYWNlIiwiaXNzIjoiaHR0cHM6Ly9jaHJvbm9nZWFycy5jb20vdGVzdCIsImF1ZCI6InRlc3QiLCJhdXRoX3RpbWUiOjEwMCwidXNlcl9pZCI6InVpZDEyMyIsInN1YiI6InNidTEyMyIsImlhdCI6MjAwLCJleHAiOjUwMCwibmJmIjozMDAsImVtYWlsIjoiYWxvdmVsYWNlQGNocm9ub2dlYXJzLmNvbSJ9.eTQnwXrri_uY55fS4IygseBzzbosDM1hP153EZXzNlLH5s29kdlGt2mL_KIjYmQa8hmptt9RwKJHBtw6l4KFHvIcuif86Ix-iI2fCpqNnKyGZfgERV51NXk1THkgWj0GQB6X5cvOoFIdHa9XvgPl_rVmzXSUYDgkhd2t01FOjQeeT6OL2d9KdlQHJqAsvvKVc3wnaYYoSqv2z0IluvK93Tk1dUBU2yWXH34nX3GAVGvIoFoNRiiFfZwFlnz78G0b2fQV7B5g5F8XlNRdD1xmVZXU8X2-xh9LqRpnEakdhecciFHg0u6AyC4c00rlo_HBb69wlXajQ3R4y26Kpxn7HA";

//     let jwt = key_store.decode(token).unwrap();

//     let claims = jwt.payload().into::<MyClaims>().unwrap();

//     println!("Issuer: {}", claims.iss);
//     println!("Name: {}", claims.name);
//     println!("Email: {}", claims.email);
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        todo!()
    }

}
