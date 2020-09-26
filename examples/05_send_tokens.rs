//! Send Tokens to an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 05_send_tokens
//! ```
use anyhow::Result;
use iota::{
    client::Transfer,
    signing::ternary::seed::Seed,
    ternary::{T1B1Buf, TryteBuf},
    transaction::bundled::{Address, BundledTransactionField},
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    // Prepare a vector of transfers
    let mut transfers = Vec::new();

    // Push the transfer to vector.
    transfers.push(Transfer {
        // Address is 81 trytes.
        address: Address::from_inner_unchecked(
            TryteBuf::try_from_str(
                "ADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADD9",
            )
            .unwrap()
            .as_trits()
            .encode(),
        ),
        value: 1,
        message: None,
        tag: None,
    });

    // Create a client instance
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.comnet.thetangle.org")?
        .build()?;
    // Call send_transfers api
    // Below is just a dummy seed which just serves as an example.
    // If you want to replace your own. It probably should be a seed with balance on comnet/devnet.
    let res = iota.send(Some(
        &Seed::from_trits(
            TryteBuf::try_from_str(
                "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
            )
            .unwrap()
            .as_trits()
            .encode::<T1B1Buf>(),
        )
        .unwrap(),
    ))
    // Input the transfers
    .transfers(transfers)
    // We are sending to comnet, so mwm should be 10. It's 14 by default if you don't call this.
    .min_weight_magnitude(10)
    // Sending to the node and receive the response
    .send()
    .await?;

    // The response of send_transfers is vector of Transaction type. We choose the first one and see what is its bundle hash
    println!(
        "Transfer sent: https://comnet.thetangle.org/bundle/{}",
        res[0].bundle().to_inner().as_i8_slice().trytes().unwrap()
    );

    Ok(())
}
