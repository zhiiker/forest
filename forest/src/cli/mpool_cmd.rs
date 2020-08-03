// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::stringify_rpc_err;
use rpc_client::{new_client, pending, sub, stat};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum MpoolCommands {
    /// List all pending messages
    #[structopt(about = "<String> = local to print only wallet local messages only")]
    Pending {
        #[structopt(short, help = "input local to print pending messags for addresses in local wallet only")]
        local: String,
    },

    /// Subscribe to MessagePool changes
    #[structopt(about = "subscribe to MessagePool changes", help = "Subscribe to MessagePool Changes")]
    Sub,

    /// Prints print out the stats of the MessagePool
    #[structopt(about = "Print Stats of MessagePool", help = "Print stats of MessagePool")]
    Stat {
        #[structopt(short, help = "input local to print stats for addresses in local wallet only")]
        local: String,
    },
}

impl MpoolCommands {
    pub async fn run(&self) {
        match self {
            Self::Pending { local } => {
                let mut client = new_client();
                let pending = pending(&mut client, local).await.map_err(stringify_rpc_err).unwrap();

                println!("{}", serde_json::to_string_pretty(&pending).unwrap());
            }
            Self::Sub => {
                let mut client = new_client();
                let sub = sub(&mut client).await.map_err(stringify_rpc_err).unwrap();

                println!("{}", serde_json::to_string_pretty(&sub).unwrap());
            }
            Self::Stat { local } => {
                unimplemented!()
                // let mut client = new_client();
                // let stat = stat(&mut client, local).await.map_err(stringify_rpc_err).unwrap();
                // println!("{}", serde_json::to_string_pretty(&stat).unwrap());
            }
        }
    }
}