use structopt::StructOpt;

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct StateCommands {
    #[structopt(subcommand)]
    pub subcommand : Subcommands,
    #[structopt(short,    about =  "Specify tipset to call method on (pass comma separated array of cids)")]
    pub tipset_value: String,
}

#[derive(Debug, StructOpt)]
pub enum Subcommands {
    #[structopt( name = "power", about =  "Query network or miner power")]
    Power{
        miner_addr : Option<String>
    }               ,
    #[structopt(  name = "sectors",  about =  "Query the sector set of a miner")]
    Sectors{
        miner_addr : String
    }             ,
    #[structopt( name = "active", about =  "Query the active sector set of a miner")]
    ActiveSectors{
        miner_addr : String
    }      ,
    #[structopt( name = "list-actors", about =  "List all actors in the network")]
    ListActors         ,
    #[structopt( name = "list-miners" , about =  "list all miners in the network")]
    ListMiners  {
        #[structopt(short,)]
        sort : bool
    }       ,
    #[structopt(name = "supply" , about =  "Get the current circulating supply of filecoin")]
    CirculatingSupply {

    } ,
    #[structopt( name = "get" , about =   "Print actor information")]
    GetActor{
        miner_addr : String
    }           ,
    #[structopt( name = "look" , about =   "Find corresponding ID address")]
    Lookup{
        miner_addr : String,
        #[structopt(short,)]
        reverse : bool,
    }              ,
    #[structopt( name = "replay" , about =   "Replay a particular message within a tipset")]
    Replay{
        tipset : String,
        message_cid : String,
    }              ,
    #[structopt( name = "sector-size" , about =   "Look up miners sector size")]
    SectorSize{
        miner_addr : String
    }         ,
    #[structopt( name = "list-miners" , about =   "View a json representation of an actors state")]
    ReadState{
        actor_addr : String
    }          ,
    #[structopt( name = "list-messages" , about =   "list messages on chain matching given criteria")]
    ListMessages {
        to : Option<String>,
        from : Option<String>,
        to_height : Option<u64>,
        #[structopt(short,)]
        cids : bool,
    }      ,
    #[structopt( name = "compute-state" , about =   "Perform state computations")]
    ComputeState{
        vm_height : Option<u64>,
        #[structopt(short,)]
        apply_mpool_messages : bool,
        #[structopt(short,)]
        show_trace : bool,
        #[structopt(short,)]
        html : bool,
    }       ,
    #[structopt( name = "call" , about =   "Invoke a method on an actor locally")]
    Call{

    }                ,
    #[structopt( name = "get-deal" , about =   "View on-chain deal info")]
    GetDeal{
        deal_id : String
    }            ,
    #[structopt( name = "wait-msg" , about =   "Wait for a message to appear on chain")]
    WaitMsg{
        message_cid : String,
        #[structopt(short,)]
        time_out : bool,
    }            ,
    #[structopt( name = "search-msg" , about =   "Search to see whether a message has appeared on chain")]
    SearchMsg{
        message_cid : String
    }          ,
    #[structopt( name = "miner-info" , about =   "Retrieve miner information")]
    MinerInfo{
        miner_addr : String
    }          ,
    #[structopt(name = "market" , about =   "Inspect the storage market actor")]
    Market {
        #[structopt(subcommand)]
        market_commands:  MarketCommands
    } ,
}

#[derive(Debug, StructOpt)]
pub enum MarketCommands {
    #[structopt(name = "balance" , about =   "Get the market balance (locked and escrowed) for a given account")]
    Balance,
}

impl StateCommands {
    pub async fn run(&self) {

    }
}