use alloc::vec::Vec;
use alloy_eips::{eip2930::AccessList, eip7702::SignedAuthorization};
use alloy_primitives::{Address, BlockHash, Bytes, ChainId, TxHash, B256, U256};
use alloy_serde::WithOtherFields;

use crate::BlockTransactions;

/// Receipt JSON-RPC response.
pub trait ReceiptResponse {
    /// Address of the created contract, or `None` if the transaction was not a deployment.
    fn contract_address(&self) -> Option<Address>;

    /// Status of the transaction.
    ///
    /// ## Note
    ///
    /// Caution must be taken when using this method for deep-historical
    /// receipts, as it may not accurately reflect the status of the
    /// transaction. The transaction status is not knowable from the receipt
    /// for transactions before [EIP-658].
    fn status(&self) -> bool;

    /// Hash of the block this transaction was included within.
    fn block_hash(&self) -> Option<BlockHash>;

    /// Number of the block this transaction was included within.
    fn block_number(&self) -> Option<u64>;

    /// Transaction Hash.
    fn transaction_hash(&self) -> TxHash;

    /// Index within the block.
    fn transaction_index(&self) -> Option<u64>;

    /// Gas used by this transaction alone.
    fn gas_used(&self) -> u128;

    /// Effective gas price.
    fn effective_gas_price(&self) -> u128;

    /// Blob gas used by the eip-4844 transaction.
    fn blob_gas_used(&self) -> Option<u128>;

    /// Blob gas price paid by the eip-4844 transaction.
    fn blob_gas_price(&self) -> Option<u128>;

    /// Address of the sender.
    fn from(&self) -> Address;

    /// Address of the receiver.
    fn to(&self) -> Option<Address>;

    /// EIP-7702 Authorization list.
    fn authorization_list(&self) -> Option<&[SignedAuthorization]>;

    /// Returns the cumulative gas used at this receipt.
    fn cumulative_gas_used(&self) -> u128;

    /// The post-transaction state root (pre Byzantium)
    ///
    /// EIP98 makes this field optional.
    fn state_root(&self) -> Option<B256>;
}

/// Transaction JSON-RPC response.
pub trait TransactionResponse {
    /// Signature type of the transaction
    type Signature;
    /// Hash of the transaction
    #[doc(alias = "transaction_hash")]
    fn tx_hash(&self) -> TxHash;

    /// Nonce
    fn nonce(&self) -> u64;

    /// Block hash
    fn block_hash(&self) -> Option<BlockHash>;

    /// Block number
    fn block_number(&self) -> Option<u64>;

    /// Transaction Index
    fn transaction_index(&self) -> Option<u64>;

    /// Sender of the transaction
    fn from(&self) -> Address;

    /// Recipient of the transaction
    fn to(&self) -> Option<Address>;

    /// Transferred value
    fn value(&self) -> U256;

    /// Gas Price
    fn gas_price(&self) -> Option<u128>;

    /// Gas limit
    fn gas(&self) -> u64;

    /// Max BaseFeePerGas the user is willing to pay
    fn max_fee_per_gas(&self) -> Option<u128>;

    /// The miner's tip
    fn max_priority_fee_per_gas(&self) -> Option<u128>;

    /// Configured max fee per blob gas for eip-4844 transactions
    fn max_fee_per_blob_gas(&self) -> Option<u128>;

    /// Input data
    #[doc(alias = "calldata")]
    fn input(&self) -> &Bytes;

    /// Transaction signature
    fn signature(&self) -> Option<Self::Signature>;

    /// The chain id of the transaction
    fn chain_id(&self) -> Option<ChainId>;

    /// Contains the blob hashes for eip-4844 transactions
    fn blob_versioned_hashes(&self) -> Option<Vec<B256>>;

    /// EIP2930 access list
    fn access_list(&self) -> Option<AccessList>;

    /// Transaction type
    fn transaction_type(&self) -> Option<u8>;

    /// The signed authorization list
    fn authorization_list(&self) -> Option<Vec<SignedAuthorization>>;
}

/// Header JSON-RPC response.
pub trait HeaderResponse {
    /// Block hash
    fn hash(&self) -> BlockHash;

    /// Block number
    fn number(&self) -> u64;

    /// Block timestamp
    fn timestamp(&self) -> u64;

    /// Extra data
    fn extra_data(&self) -> &Bytes;

    /// Base fee per unit of gas (If EIP-1559 is supported)
    fn base_fee_per_gas(&self) -> Option<u64>;

    /// Blob fee for the next block (if EIP-4844 is supported)
    fn next_block_blob_fee(&self) -> Option<u128>;

    /// Coinbase/Miner of the block
    fn coinbase(&self) -> Address;

    /// Gas limit of the block
    fn gas_limit(&self) -> u64;

    /// Mix hash of the block
    ///
    /// Before the merge this proves, combined with the nonce, that a sufficient amount of
    /// computation has been carried out on this block: the Proof-of-Work (PoW).
    ///
    /// After the merge this is `prevRandao`: Randomness value for the generated payload.
    ///
    /// This is an Option because it is not always set by non-ethereum networks.
    ///
    /// See also <https://eips.ethereum.org/EIPS/eip-4399>
    /// And <https://github.com/ethereum/execution-apis/issues/328>
    fn mix_hash(&self) -> Option<B256>;

    /// Difficulty of the block
    ///
    /// Unused after the Paris (AKA the merge) upgrade, and replaced by `prevrandao`.
    fn difficulty(&self) -> U256;
}

/// Block JSON-RPC response.
pub trait BlockResponse {
    /// Header type
    type Header: HeaderResponse;
    /// Transaction type
    type Transaction: TransactionResponse;

    /// Block header
    fn header(&self) -> &Self::Header;

    /// Block transactions
    fn transactions(&self) -> &BlockTransactions<Self::Transaction>;

    /// Mutable reference to block transactions
    fn transactions_mut(&mut self) -> &mut BlockTransactions<Self::Transaction>;

    /// Returns the `other` field from `WithOtherFields` type.
    fn other_fields(&self) -> Option<&alloy_serde::OtherFields> {
        None
    }
}

impl<T: TransactionResponse> TransactionResponse for WithOtherFields<T> {
    type Signature = T::Signature;

    fn tx_hash(&self) -> TxHash {
        self.inner.tx_hash()
    }

    fn nonce(&self) -> u64 {
        self.inner.nonce()
    }

    fn block_hash(&self) -> Option<BlockHash> {
        self.inner.block_hash()
    }

    fn block_number(&self) -> Option<u64> {
        self.inner.block_number()
    }

    fn transaction_index(&self) -> Option<u64> {
        self.inner.transaction_index()
    }

    fn from(&self) -> Address {
        self.inner.from()
    }

    fn to(&self) -> Option<Address> {
        self.inner.to()
    }

    fn value(&self) -> U256 {
        self.inner.value()
    }

    fn gas_price(&self) -> Option<u128> {
        self.inner.gas_price()
    }

    fn gas(&self) -> u64 {
        self.inner.gas()
    }

    fn max_fee_per_gas(&self) -> Option<u128> {
        self.inner.max_fee_per_gas()
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.inner.max_priority_fee_per_gas()
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        self.inner.max_fee_per_blob_gas()
    }

    fn input(&self) -> &Bytes {
        self.inner.input()
    }

    fn signature(&self) -> Option<T::Signature> {
        self.inner.signature()
    }

    fn chain_id(&self) -> Option<ChainId> {
        self.inner.chain_id()
    }

    fn blob_versioned_hashes(&self) -> Option<Vec<B256>> {
        self.inner.blob_versioned_hashes()
    }

    fn access_list(&self) -> Option<AccessList> {
        self.inner.access_list()
    }

    fn transaction_type(&self) -> Option<u8> {
        self.inner.transaction_type()
    }

    fn authorization_list(&self) -> Option<Vec<SignedAuthorization>> {
        self.inner.authorization_list()
    }
}

impl<T: ReceiptResponse> ReceiptResponse for WithOtherFields<T> {
    fn contract_address(&self) -> Option<Address> {
        self.inner.contract_address()
    }

    fn status(&self) -> bool {
        self.inner.status()
    }

    fn block_hash(&self) -> Option<BlockHash> {
        self.inner.block_hash()
    }

    fn block_number(&self) -> Option<u64> {
        self.inner.block_number()
    }

    fn transaction_hash(&self) -> TxHash {
        self.inner.transaction_hash()
    }

    fn transaction_index(&self) -> Option<u64> {
        self.inner.transaction_index()
    }

    fn gas_used(&self) -> u128 {
        self.inner.gas_used()
    }

    fn effective_gas_price(&self) -> u128 {
        self.inner.effective_gas_price()
    }

    fn blob_gas_used(&self) -> Option<u128> {
        self.inner.blob_gas_used()
    }

    fn blob_gas_price(&self) -> Option<u128> {
        self.inner.blob_gas_price()
    }

    fn from(&self) -> Address {
        self.inner.from()
    }

    fn to(&self) -> Option<Address> {
        self.inner.to()
    }

    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        self.inner.authorization_list()
    }

    fn cumulative_gas_used(&self) -> u128 {
        self.inner.cumulative_gas_used()
    }

    fn state_root(&self) -> Option<B256> {
        self.inner.state_root()
    }
}

impl<T: BlockResponse> BlockResponse for WithOtherFields<T> {
    type Header = T::Header;
    type Transaction = T::Transaction;

    fn header(&self) -> &Self::Header {
        self.inner.header()
    }

    fn transactions(&self) -> &BlockTransactions<Self::Transaction> {
        self.inner.transactions()
    }

    fn transactions_mut(&mut self) -> &mut BlockTransactions<Self::Transaction> {
        self.inner.transactions_mut()
    }

    fn other_fields(&self) -> Option<&alloy_serde::OtherFields> {
        Some(&self.other)
    }
}

impl<T: HeaderResponse> HeaderResponse for WithOtherFields<T> {
    fn hash(&self) -> BlockHash {
        self.inner.hash()
    }

    fn number(&self) -> u64 {
        self.inner.number()
    }

    fn timestamp(&self) -> u64 {
        self.inner.timestamp()
    }

    fn extra_data(&self) -> &Bytes {
        self.inner.extra_data()
    }

    fn base_fee_per_gas(&self) -> Option<u64> {
        self.inner.base_fee_per_gas()
    }

    fn next_block_blob_fee(&self) -> Option<u128> {
        self.inner.next_block_blob_fee()
    }

    fn coinbase(&self) -> Address {
        self.inner.coinbase()
    }

    fn gas_limit(&self) -> u64 {
        self.inner.gas_limit()
    }

    fn mix_hash(&self) -> Option<B256> {
        self.inner.mix_hash()
    }

    fn difficulty(&self) -> U256 {
        self.inner.difficulty()
    }
}
