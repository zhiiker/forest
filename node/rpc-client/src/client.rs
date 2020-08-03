// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

#![allow(clippy::all)]
#![allow(unused_variables, dead_code)]

use blocks::{header::json::BlockHeaderJson, tipset_json::TipsetJson};
use cid::json::CidJson;
use jsonrpsee::raw::RawClient;
use jsonrpsee::transport::http::HttpTransportClient;
use message::{unsigned_message::json::UnsignedMessageJson, signed_message::json::SignedMessageJson};
use cid::json::vec::CidJsonVec;
use message_pool::json::MpSubChangeJson;

jsonrpsee::rpc_api! {
    pub Filecoin {
        /// Chain
        #[rpc(method = "Filecoin.ChainGetBlock", positional_params)]
        fn chain_get_block(cid: CidJson) -> BlockHeaderJson;

        #[rpc(method = "Filecoin.ChainGetGenesis")]
        fn chain_get_genesis() -> TipsetJson;

        #[rpc(method = "Filecoin.ChainHead")]
        fn chain_get_head() -> TipsetJson;

        #[rpc(method = "Filecoin.ChainGetMessage", positional_params)]
        fn chain_get_messages(cid: CidJson) -> UnsignedMessageJson;

        #[rpc(method = "Filecoin.ChainGetObj", positional_params)]
        fn chain_read_obj(cid: CidJson) -> Vec<u8>;

        #[rpc(method = "Filecoin.WalletList")]
        fn wallet_list() -> Vec<String>;

        /// Mpool
        #[rpc(method = "Filecoin.MpoolPending", positional_params)]
        fn mpool_pending(cids: CidJsonVec) -> Vec<SignedMessageJson>;

        #[rpc(method = "Filecoin.MpoolSub")]
        fn mpool_sub() -> MpSubChangeJson;
    }
}

// TODO need to handle dynamic port
const URL: &str = "http://127.0.0.1:1234/rpc/v0";

// TODO pass config for URL
pub fn new_client() -> RawClient<HttpTransportClient> {
    RawClient::new(HttpTransportClient::new(URL))
}
