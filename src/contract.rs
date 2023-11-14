use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::error::{ContractError, CryptoError};
use crate::msg::{
    CountResponse, DecryptedResponse, ExecuteMsg, GetStoredMessageResp, InstantiateMsg,
    KeysResponse, QueryMsg,
};
use crate::state::{
    config, config_read, Decrypted, MyKeys, MyMessage, State, DECRYPTED, MY_KEYS, STORED_MESSAGE,
};
use ecies::utils::encapsulate;
use ecies::utils::generate_keypair;
use ecies::{PublicKey, SecretKey};
use rand_chacha::{rand_core::SeedableRng, ChaChaRng};

// //
use aes_siv::aead::generic_array::GenericArray;
use aes_siv::siv::Aes128Siv;
use ethabi::{decode, ParamType};
use log::*;
// use secret_toolkit_crypto::ContractPrng;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };

    config(deps.storage).save(&state)?;

    deps.api
        .debug(&format!("Contract was initialized by {}", info.sender));

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateKeys {} => try_create_keys(deps, env),
        ExecuteMsg::TryDecrypt {
            ciphertext,
            public_key,
        } => try_decrypt(deps, env, ciphertext, public_key),
        ExecuteMsg::ReceiveMessageEvm {
            source_chain,
            source_address,
            payload,
        } => receive_message_evm(deps, source_chain, source_address, payload),
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_create_keys(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let mut ring = ChaChaRng::from_seed(env.block.random.unwrap().to_array()?);
    let (private_key, public_key) = generate_keypair(&mut ring);

    let private_key_vec = private_key.serialize().to_vec();
    let public_key_vec = public_key.serialize().to_vec();

    let my_keys = MyKeys {
        private_key: private_key_vec,
        public_key: public_key_vec,
    };

    MY_KEYS.save(deps.storage, &my_keys)?;

    Ok(Response::default())
}

pub fn try_decrypt(
    deps: DepsMut,
    _env: Env,
    ciphertext: Vec<u8>,
    public_key: Vec<u8>,
) -> Result<Response, ContractError> {
    let my_keys = MY_KEYS.load(deps.storage)?;

    let public_key_slice = public_key.as_slice();

    // // Convert the slice into an array
    let mut public_key_array = [0u8; 65];
    public_key_array.copy_from_slice(public_key_slice);

    // Now you can parse it
    let public_key_formatted = PublicKey::parse(&public_key_array).unwrap();

    let private_key_slice = my_keys.private_key.as_slice();
    let mut private_key_array = [0u8; 32];
    private_key_array.copy_from_slice(private_key_slice);
    let private_key_formatted = SecretKey::parse(&private_key_array).unwrap();

    let encapsulate_key = encapsulate(&private_key_formatted, &public_key_formatted);

    let key = encapsulate_key.unwrap();

    let ad_data: &[&[u8]] = &[];
    let ad = Some(ad_data);

    match aes_siv_decrypt(&ciphertext, ad, &key) {
        Ok(decrypted_data) => {
            let decrypted = Decrypted {
                decrypted: String::from_utf8(decrypted_data.clone()).unwrap(),
            };
            DECRYPTED.save(deps.storage, &decrypted)?;
            println!(
                "Decrypted data: {:?}",
                String::from_utf8(decrypted_data).unwrap()
            );
        }
        Err(e) => {
            warn!("Error decrypting data: {:?}", e);
        }
    }

    Ok(Response::default())
}

pub fn aes_siv_encrypt(
    plaintext: &[u8],
    ad: Option<&[&[u8]]>,
    key: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let ad = ad.unwrap_or(&[&[]]);

    let mut cipher = Aes128Siv::new(GenericArray::clone_from_slice(key));
    cipher.encrypt(ad, plaintext).map_err(|e| {
        warn!("aes_siv_encrypt error: {:?}", e);
        CryptoError::EncryptionError
    })
}

pub fn aes_siv_decrypt(
    ciphertext: &[u8],
    ad: Option<&[&[u8]]>,
    key: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let ad = ad.unwrap_or(&[&[]]);

    let mut cipher = Aes128Siv::new(GenericArray::clone_from_slice(key));
    cipher.decrypt(ad, ciphertext).map_err(|e| {
        warn!("aes_siv_decrypt error: {:?}", e);
        CryptoError::DecryptionError
    })
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    config(deps.storage).update(|mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    deps.api.debug("count incremented successfully");
    Ok(Response::default())
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    config(deps.storage).update(|mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;

    deps.api.debug("count reset successfully");
    Ok(Response::default())
}

pub fn receive_message_evm(
    deps: DepsMut,
    _source_chain: String,
    _source_address: String,
    payload: Binary,
) -> Result<Response, ContractError> {
    // decode the payload
    // executeMsgPayload: [sender, message]
    let decoded = decode(&vec![ParamType::Bytes], payload.as_slice()).unwrap();

    // store message
    STORED_MESSAGE.save(
        deps.storage,
        &MyMessage {
            message: decoded[0].to_string(),
        },
    )?;

    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetKeys {} => to_binary(&query_keys(deps)?),
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetDecrypted {} => to_binary(&query_decrypted(deps)?),
        QueryMsg::GetStoredMessage {} => to_binary(&get_stored_message(deps)?),
    }
}

fn query_decrypted(deps: Deps) -> StdResult<DecryptedResponse> {
    let decrypted = DECRYPTED.load(deps.storage)?;
    Ok(DecryptedResponse {
        decrypted: decrypted.decrypted,
    })
}

pub fn get_stored_message(deps: Deps) -> StdResult<GetStoredMessageResp> {
    let message = STORED_MESSAGE.may_load(deps.storage).unwrap().unwrap();
    let resp = GetStoredMessageResp {
        message: message.message,
    };
    Ok(resp)
}

fn query_keys(deps: Deps) -> StdResult<KeysResponse> {
    let my_keys = MY_KEYS.load(deps.storage)?;
    Ok(KeysResponse {
        public_key: my_keys.public_key,
        private_key: my_keys.private_key,
    })
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(CountResponse { count: state.count })
}
