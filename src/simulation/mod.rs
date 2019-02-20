use crate::datastructures::block::Block;
use crate::datastructures::transaction::Transaction;
use crate::datastructures::pbft::ViewChange;

#[derive(Clone, Debug)]
pub enum Event {
    // External events.
    Block(Block),
    Transaction(Transaction),

    // PBFT.
    ViewChange(ViewChange),
    BlockProposal(Block),
    BlockPrepare,
    BlockCommit,

    // Internal events.
    BlockProcessed(Block, bool),
    BlockProduced(Block),
    TransactionProcessed(Transaction),
    Timeout,
}

pub struct SimulationConfig {
    pub blocks: u32,
}