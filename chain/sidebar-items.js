initSidebarItems({"constant":[["BASE_FEE_MAX_CHANGE_DENOM",""],["BLOCK_GAS_TARGET",""],["HEAD_KEY",""],["INITIAL_BASE_FEE",""],["PACKING_EFFICIENCY_DENOM",""],["PACKING_EFFICIENCY_NUM",""]],"enum":[["Error","Chain error"],["EventsPayload",""],["HeadChange","Enum for pubsub channel that defines message type variant and data contained in message type."]],"fn":[["block_messages","Returns a Tuple of bls messages of type UnsignedMessage and secp messages of type SignedMessage"],["block_messages_from_cids","Returns a tuple of UnsignedMessage and SignedMessages from their Cid"],["compute_base_fee",""],["draw_randomness","Computes a pseudorandom 32 byte Vec"],["genesis","Returns the genesis block"],["get_chain_message","Attempts to deserialize to unsigend message or signed message and then returns it at as a message trait object"],["get_parent_reciept","returns message receipt given block_header"],["messages_for_tipset","given a tipset this function will return all messages"],["messages_from_cids","Returns messages from key-value store"],["persist_objects","Persists slice of serializable objects to blockstore."],["read_msg_cids","Returns a tuple of cids for both Unsigned and Signed messages"],["set_genesis","Sets the genesis key in the BlockStore. Be careful if using this outside of the ChainStore as it will not update what the ChainStore thinks is the genesis after the ChainStore has been created."],["sub_head_changes",""],["weight","Returns the weight of provided tipset"]],"mod":[["base_fee",""],["headchange_json",""]],"struct":[["ChainIndex","Keeps lookback tipsets in cache at a given interval `skip_length` and can be used to lookback at the chain to retrieve an old tipset."],["ChainStore","Stores chain data such as heaviest tipset and cached tipset info at each epoch. This structure is threadsafe, and all caches are wrapped in a mutex to allow a consistent `ChainStore` to be shared across tasks."],["IndexToHeadChange",""],["LookbackEntry","Lookback entry to cache in the `ChainIndex`. Stores all relevant info when doing lookbacks."],["MINIMUM_BASE_FEE","Cbor bytes of an empty array serialized."]]});