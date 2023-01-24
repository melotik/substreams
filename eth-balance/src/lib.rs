pub mod pb;

use substreams::Hex;
use substreams_ethereum::pb::eth as pbeth;
use substreams_helper::pb::evm_token::v1 as token;
use substreams_helper::token as token_helper;

#[substreams::handlers::map]
fn map_balances(block: pbeth::v2::Block) -> Result<token::Transfers, substreams::errors::Error> {
    let mut transfers = vec![];

    for transaction in &block.transaction_traces {
        let mut balance_changes = vec![];
        for call in &transaction.calls {
            for balance_change in &call.balance_changes {
                balance_changes.push(token::TokenBalance {
                    log_ordinal: balance_change.ordinal,
                    token: token_helper::get_eth_token(),
                    address: Hex(&balance_change.address).to_string(),
                    old_balance: token_helper::bigint_to_string(balance_change.old_value.clone()),
                    new_balance: token_helper::bigint_to_string(balance_change.new_value.clone()),
                    reason: Some(balance_change.reason),
                });
            }
        }
        transfers.push(token::Transfer {
            tx_hash: Hex::encode(&transaction.hash),
            block_number: block.number,
            timestamp: block
                .header
                .as_ref()
                .unwrap()
                .timestamp
                .as_ref()
                .unwrap()
                .seconds as u64,
            log_index: transaction.index,
            token: token_helper::get_eth_token(),
            to: Hex::encode(&transaction.to),
            from: Hex::encode(&transaction.from),
            amount: token_helper::bigint_to_string(transaction.value.clone()),
            amount_usd: None,
            balance_changes: balance_changes,
        });
    }

    Ok(token::Transfers { items: transfers })
}
