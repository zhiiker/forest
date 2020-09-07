// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::client::Filecoin;
use cid::{Cid, json::vec::CidJsonVec};
use jsonrpc_v2::Error as JsonRpcError;
use jsonrpsee::raw::RawClient;
use jsonrpsee::transport::http::HttpTransportClient as HTC;
use message::signed_message::json::SignedMessageJson;
use message::SignedMessage;
use std::collections::HashMap;
use address::Address;
use std::str::FromStr;
use message::Message;
use blocks::tipset_json::TipsetJson;
use message_pool::json::MpSubChangeJson;
use actor::ActorState;
use crate::mpool_ops::json::MpStatJson;

/// Get all pending messages in message pool
pub async fn pending(client: &mut RawClient<HTC>, local: &String) ->  Result<Vec<SignedMessageJson>, JsonRpcError> {
    let mut filter: Vec<Address> = Vec::new();

    if local == "local" {
        let addrs: Vec<String> = Filecoin::wallet_list(client).await?;
        filter = addrs.into_iter().map(|addr| Address::from_str(&addr).unwrap()).collect();
    }

    let vec = vec![Cid::default()];
    let msgs_json: Vec<SignedMessageJson> = Filecoin::mpool_pending(client, CidJsonVec(vec)).await?;
    let msgs: Vec<SignedMessage> = msgs_json.into_iter().map(|msg_json| {
        let SignedMessageJson(smsg) = msg_json;
        smsg
    }).collect();

    let mut out_msgs = Vec::new();
    for msg in msgs {
        if !filter.is_empty() {
            if filter.contains(msg.from()) {
                out_msgs.push(SignedMessageJson(msg))
            }
        } else {
            out_msgs.push(SignedMessageJson(msg))
        }
    }
    Ok(out_msgs)
}

pub async fn sub(client: &mut RawClient<HTC>) -> Result<MpSubChangeJson, JsonRpcError> {
    Ok(Filecoin::mpool_sub(client).await?)
}

/// Struct that will contain all of the message pool stats
pub struct StatBucket {
    pub msgs: HashMap<u64, SignedMessage>
}

/// This is the struct that will be used to display stats of the message pool
pub struct MpStat {
    pub addr: String,
    pub past: u64,
    pub cur: u64,
    pub future: u64
}

#[cfg(feature = "json")]
pub mod json {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Wrapper for serializing and deserializing MpStat from JSON.
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct MpStatJson(#[serde(with = "self")] pub MpStat);

    /// Wrapper for serializing a MpStat reference to JSON.
    #[derive(Serialize)]
    #[serde(transparent)]
    pub struct MpStatJsonRef<'a>(#[serde(with = "self")] pub &'a MpStat);

    impl From<MpStatJson> for MpStat {
        fn from(wrapper: MpStatJson) -> Self {
            wrapper.0
        }
    }

    pub fn serialize<S>(m: &MpStat, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct MpStatSer<'a> {
            addr: &'a String,
            past: &'a u64,
            cur: &'a u64,
            future: &'a u64,
        }
        MpStatSer {
            addr: &m.addr,
            past: &m.past,
            cur: &m.cur,
            future: &m.future,
        }
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<MpStat, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct MpStatDe {
            addr: String,
            past: u64,
            cur: u64,
            future: u64,
        }
        let MpStatDe { addr, past, cur, future } = Deserialize::deserialize(deserializer)?;
        Ok(MpStat { addr, past, cur, future })
    }

    pub mod vec {
        use super::*;
        use forest_json_utils::GoVecVisitor;
        use serde::ser::SerializeSeq;

        pub fn serialize<S>(m: &[MpStat], serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(m.len()))?;
            for e in m {
                seq.serialize_element(&MpStatJsonRef(e))?;
            }
            seq.end()
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<MpStat>, D::Error>
            where
                D: Deserializer<'de>,
        {
            deserializer.deserialize_any(GoVecVisitor::<MpStat, MpStatJson>::new())
        }
    }
}

/// Get the stats of the message pool
pub async fn stat(client: &mut RawClient<HTC>, local: &String) -> Result<Vec<MpStatJson>, JsonRpcError>
{
    let TipsetJson(tipset) = Filecoin::chain_get_head(client).await.unwrap();


    let mut filter: Vec<Address> = Vec::new();

    if local == "local" {
        let addrs: Vec<String> = Filecoin::wallet_list(client).await.unwrap();
        filter = addrs.into_iter().map(|addr| Address::from_str(&addr).unwrap()).collect();
    }

    let vec = vec![Cid::default()];
    let msgs_json: Vec<SignedMessageJson> = Filecoin::mpool_pending(client, CidJsonVec(vec)).await.unwrap();
    let msgs: Vec<SignedMessage> = msgs_json.into_iter().map(|msg_json| {
        let SignedMessageJson(smsg) = msg_json;
        smsg
    }).collect();

    let mut buckets = HashMap::new();

    for item in msgs {
        if !filter.is_empty() {
            if !filter.contains(item.from()) {
                continue
            }
        }


        if buckets.contains_key(item.from()) {
            let bkt = StatBucket{ msgs: HashMap::new() };
            buckets.insert(item.from().clone(), bkt);
        }
        buckets.get_mut(item.from()).unwrap().msgs.insert(item.sequence(), item);
    }

    let mut out = Vec::new();

    for (k, bkt) in buckets {
        let act: ActorState = Filecoin::state_get_actor(client, (k, tipset.parents().clone())).await.unwrap().unwrap();

        let mut cur = act.sequence;
        loop {
            let s = bkt.msgs.get(&cur);
            if s.is_none() {
                break
            cur += 1;}
        }

        let mut past = 0;
        let mut future = 0;

        for (_, m) in bkt.msgs {
            if m.message().sequence < act.sequence {
                past += 1;
            }
            if m.message().sequence > cur {
                future += 1;
            }
        }

        out.push(MpStat {
            addr: k.to_string(),
            past,
            cur: cur - act.sequence,
            future
        })
    }

    out.sort_by(|a, b| {
        a.addr.cmp(&b.addr)
    });

    let mut out_json = Vec::new();
    for i in out {
        out_json.push(MpStatJson(i))
    }

    Ok(out_json)
}