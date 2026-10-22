use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
  InscriptionCreated {
    block_height: u32,
    charms: u16,
    inscription_id: InscriptionId,
    location: Option<SatPoint>,
    parent_inscription_ids: Vec<InscriptionId>,
    sequence_number: u32,
  },
  InscriptionTransferred {
    block_height: u32,
    inscription_id: InscriptionId,
    new_location: SatPoint,
    old_location: SatPoint,
    sequence_number: u32,
  },
  RuneBurned {
    amount: u128,
    block_height: u32,
    rune_id: RuneId,
    txid: Txid,
  },
  RuneEtched {
    block_height: u32,
    rune_id: RuneId,
    txid: Txid,
  },
  RuneMinted {
    amount: u128,
    block_height: u32,
    rune_id: RuneId,
    txid: Txid,
  },
  RuneTransferred {
    amount: u128,
    block_height: u32,
    outpoint: OutPoint,
    rune_id: RuneId,
    txid: Txid,
  },
  /// Sent right after `InscriptionCreated`, with what the envelope declared.
  InscriptionRevealed {
    block_height: u32,
    details: InscriptionDetails,
    fee: u64,
    inscription_id: InscriptionId,
    inscription_number: i32,
    sequence_number: u32,
  },
  /// Sent right after `RuneEtched`, with the new rune's entry.
  RuneEntryCreated {
    block_height: u32,
    entry: RuneEntry,
    rune_id: RuneId,
    txid: Txid,
  },
  /// Sent once a block's events have all been sent.
  BlockIndexed {
    block_hash: BlockHash,
    block_height: u32,
  },
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InscriptionDetails {
  pub content_encoding: Option<String>,
  pub content_length: Option<usize>,
  pub content_type: Option<String>,
  pub delegate: Option<InscriptionId>,
  pub gallery: bool,
  pub hidden: bool,
  pub metaprotocol: Option<String>,
  pub pointer: Option<u64>,
}

impl InscriptionDetails {
  pub(crate) fn from_inscription(inscription: &Inscription) -> Self {
    Self {
      content_encoding: inscription
        .content_encoding()
        .and_then(|encoding| encoding.to_str().ok().map(str::to_string)),
      content_length: inscription.content_length(),
      content_type: inscription.content_type().map(str::to_string),
      delegate: inscription.delegate(),
      gallery: !inscription.properties().gallery.is_empty(),
      hidden: inscription.hidden(),
      metaprotocol: inscription.metaprotocol().map(str::to_string),
      pointer: inscription.pointer(),
    }
  }
}
