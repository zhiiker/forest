(function() {var implementors = {};
implementors["actor_interface"] = [{"text":"impl StructuralPartialEq for MinerInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for FilterEstimate","synthetic":false,"types":[]}];
implementors["beacon"] = [{"text":"impl StructuralPartialEq for BeaconEntry","synthetic":false,"types":[]}];
implementors["chain"] = [{"text":"impl StructuralPartialEq for LookbackEntry","synthetic":false,"types":[]}];
implementors["chain_sync"] = [{"text":"impl StructuralPartialEq for SyncStage","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SyncState","synthetic":false,"types":[]}];
implementors["conformance_tests"] = [{"text":"impl StructuralPartialEq for RandomnessKind","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for RandomnessRule","synthetic":false,"types":[]}];
implementors["fil_types"] = [{"text":"impl StructuralPartialEq for DeadlineInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for UnpaddedPieceSize","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for PaddedPieceSize","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for PieceInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Randomness","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SectorInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for PoStProof","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for WinningPoStVerifyInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for WindowPoStVerifyInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for OnChainWindowPoStVerifyInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for RegisteredSealProof","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for RegisteredPoStProof","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SealVerifyInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SealVerifyParams","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SectorSize","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SectorID","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for StateTreeVersion","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for NetworkVersion","synthetic":false,"types":[]}];
implementors["forest_actor"] = [{"text":"impl StructuralPartialEq for Entry","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for DealProposal","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ClientDealProposal","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for DealState","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for PowerPair","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for MinerInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for WorkerKeyChange","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SectorPreCommitInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SectorPreCommitOnChainInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SectorOnChainInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ChainSectorInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Fault","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Transaction","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for LaneState","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Merge","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SignedVoucher","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ModVerifyParams","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for VestingFunction","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Reward","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for AwardBlockRewardParams","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ThisEpochRewardReturn","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for VerifierParams","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for BytesParams","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for FilterEstimate","synthetic":false,"types":[]}];
implementors["forest_address"] = [{"text":"impl StructuralPartialEq for Error","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Network","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Payload","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Protocol","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Address","synthetic":false,"types":[]}];
implementors["forest_bitfield"] = [{"text":"impl StructuralPartialEq for BitFieldJson","synthetic":false,"types":[]}];
implementors["forest_blocks"] = [{"text":"impl StructuralPartialEq for Block","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ElectionProof","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Error","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for BlockHeader","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Ticket","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for EPostTicket","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for EPostProof","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for TipsetKeys","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Tipset","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for FullTipset","synthetic":false,"types":[]}];
implementors["forest_car"] = [{"text":"impl StructuralPartialEq for CarHeader","synthetic":false,"types":[]}];
implementors["forest_cid"] = [{"text":"impl StructuralPartialEq for Code","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Prefix","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Cid","synthetic":false,"types":[]}];
implementors["forest_crypto"] = [{"text":"impl StructuralPartialEq for Error","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for DomainSeparationTag","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SignatureType","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Signature","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for VRFProof","synthetic":false,"types":[]}];
implementors["forest_encoding"] = [{"text":"impl StructuralPartialEq for BytesDe","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Error","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for CodecProtocol","synthetic":false,"types":[]}];
implementors["forest_hash_utils"] = [{"text":"impl StructuralPartialEq for BytesKey","synthetic":false,"types":[]}];
implementors["forest_ipld"] = [{"text":"impl StructuralPartialEq for Error","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Path","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for PathSegment","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for VisitReason","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for LastBlockInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Selector","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for RecursionLimit","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Condition","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Ipld","synthetic":false,"types":[]}];
implementors["forest_libp2p"] = [{"text":"impl StructuralPartialEq for ChainExchangeRequest","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ChainExchangeResponseStatus","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ChainExchangeResponse","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for CompactedMessages","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for TipsetBundle","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ChainExchangeProtocolName","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for HelloRequest","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for HelloResponse","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for HelloProtocolName","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for RPCResponse","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for RPCRequest","synthetic":false,"types":[]}];
implementors["forest_message"] = [{"text":"impl StructuralPartialEq for MessageReceipt","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for SignedMessage","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for UnsignedMessage","synthetic":false,"types":[]}];
implementors["forest_vm"] = [{"text":"impl StructuralPartialEq for ActorState","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ActorError","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for ExitCode","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Serialized","synthetic":false,"types":[]}];
implementors["ipld_amt"] = [{"text":"impl StructuralPartialEq for BitMap","synthetic":false,"types":[]}];
implementors["ipld_blockstore"] = [{"text":"impl StructuralPartialEq for BSStats","synthetic":false,"types":[]}];
implementors["key_management"] = [{"text":"impl StructuralPartialEq for Error","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for KeyInfo","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for MemKeyStore","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for PersistentKeyStore","synthetic":false,"types":[]},{"text":"impl StructuralPartialEq for Key","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; StructuralPartialEq for Wallet&lt;T&gt;","synthetic":false,"types":[]}];
implementors["message_pool"] = [{"text":"impl StructuralPartialEq for Error","synthetic":false,"types":[]}];
implementors["state_manager"] = [{"text":"impl StructuralPartialEq for Error","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()