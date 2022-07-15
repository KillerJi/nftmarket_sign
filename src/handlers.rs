use crate::{app_data::AppData, error::XProtocolError};
use actix_web::{web, HttpResponse};
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use web3::{
    ethabi::ParamType,
    signing::{Key, SecretKeyRef},
    types::{H160, H256},
};
use web3_macros::SignV4;

#[derive(SignV4)]
#[primary_type]
#[domain_712("XP721", "1")]
pub struct SellerSign {
    #[web3_type("ParamType::Uint(256)")]
    pub tokenid: u64,
    #[web3_type("ParamType::Address")]
    pub nftaddress: H160,
    #[web3_type("ParamType::Uint(256)")]
    pub orderid: u64,
    #[web3_type("ParamType::Uint(256)")]
    pub price: web3::types::U256,
}

#[derive(SignV4)]
#[primary_type]
#[domain_712("XP721", "1")]
pub struct TakeDown {
    #[web3_type("ParamType::Address")]
    pub account: H160,
    #[web3_type("ParamType::Uint(256)")]
    pub orderid: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MaySignature {
    pub tokenid: u64,
    pub nftaddress: H160,
    pub orderid: u64,
    pub price: web3::types::U256,
    pub v: u64,
    pub r: H256,
    pub s: H256,
}

#[derive(Serialize, Deserialize)]
pub struct MaySignature2 {
    pub account: H160,
    pub orderid: u64,
    pub v: u64,
    pub r: H256,
    pub s: H256,
}

pub struct Handlers;

impl Handlers {
    pub fn app_config(cfg: &mut web::ServiceConfig) {
        cfg.route("/", web::get().to(Self::index))
            .route(
                "/sign/{chain_id}/{tokenid}/{nftaddress}/{orderid}/{price}",
                web::get().to(Self::sign),
            )
            .route(
                "/sign2/{chain_id}/{account}/{orderid}",
                web::get().to(Self::sign2),
            );
    }

    pub async fn index() -> Result<HttpResponse, XProtocolError> {
        Ok(HttpResponse::Ok().body("Hello World"))
    }

    pub async fn sign2(
        path: web::Path<(String, String, u64)>,
        _data: web::Data<AppData>,
    ) -> Result<HttpResponse, XProtocolError> {
        let (chain_id, account, orderid) = path.into_inner();
        // let contract = "0x5fbdb2315678afecb367f032d93f642f64180aa3";
        let contract = "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707";
        // let contract = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";
        let contract = H160::from_str(contract).map_err(|_| XProtocolError::InternalServerError)?;
        let account = account
            .parse()
            .map_err(|_| XProtocolError::ExpectationFailed)?;
        let sign: [u8; 32] = TakeDown { account, orderid }
            .sign_hash(&chain_id, contract)
            .map_err(|_| XProtocolError::InternalServerError)?;
        println!("sign {:?}", sign);
        let secret = SecretKey::from_slice(_data.private_key.as_bytes()).unwrap();
        let secret_ref = SecretKeyRef::new(&secret);
        let signature = secret_ref
            .sign(&sign, None)
            .map_err(|_| XProtocolError::InternalServerError)?;
        Ok(HttpResponse::Ok().json(MaySignature2 {
            account,
            orderid,
            r: signature.r,
            s: signature.s,
            v: signature.v,
        }))
    }

    pub async fn sign(
        path: web::Path<(String, u64, String, u64, String)>,
        _data: web::Data<AppData>,
    ) -> Result<HttpResponse, XProtocolError> {
        let (chain_id, tokenid, nftaddress, orderid, price) = path.into_inner();
        // let contract = "0x5fbdb2315678afecb367f032d93f642f64180aa3";
        let contract = "0x5CcE31385A5Db892d793098bBbbE1e50de7c3b8c";
        // let contract = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";
        let contract = H160::from_str(contract).map_err(|_| XProtocolError::InternalServerError)?;
        let account = nftaddress
            .parse()
            .map_err(|_| XProtocolError::ExpectationFailed)?;
        let price = web3::types::U256::from_str_radix(&price, 10)
            .map_err(|_| XProtocolError::ExpectationFailed)?;
        let sign: [u8; 32] = SellerSign {
            tokenid,
            nftaddress: account,
            orderid,
            price,
        }
        .sign_hash(&chain_id, contract)
        .map_err(|_| XProtocolError::InternalServerError)?;
        println!("sign {:?}", sign);
        let secret = SecretKey::from_slice(_data.private_key.as_bytes()).unwrap();
        let secret_ref = SecretKeyRef::new(&secret);
        let signature = secret_ref
            .sign(&sign, None)
            .map_err(|_| XProtocolError::InternalServerError)?;
        Ok(HttpResponse::Ok().json(MaySignature {
            tokenid,
            nftaddress: account,
            orderid,
            price,
            r: signature.r,
            s: signature.s,
            v: signature.v,
        }))
    }
}
