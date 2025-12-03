use hyperliquid_rs::{
    api::current_time_millis,
    client::HyperliquidClientBuilder,
    example_helpers::load_signer,
    init_tracing::init_tracing,
    signature::sign::{sig_v_from_bool, sign_l1_action},
    types::{
        exchange::{Grouping, LimitOrderType, OrderAction, OrderRequest, OrderType, Tif},
        signature::Eip712Signature,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let pkey_signer = load_signer();
    let mut builder = HyperliquidClientBuilder::new();

    let hyperliquid = builder.testnet().with_wallet(pkey_signer.clone()).build()?;

    let info_api = hyperliquid.info();
    let exchange_api = hyperliquid.exchange();

    let perp_meta = info_api.perpetuals_metadata(None).await?;
    let doge_asset_id = perp_meta
        .universe
        .iter()
        .enumerate()
        .find(|asset| asset.1.name == "DOGE");

    tracing::info!("Using asset: {:?}", doge_asset_id.unwrap());

    let all_mids = info_api.all_mids(None).await?;
    let doge_price = all_mids
        .get(&format!("@{}", doge_asset_id.unwrap().0))
        .unwrap();

    let limit_order_type = LimitOrderType { tif: Tif::Gtc };

    if let Some((asset_idx, _)) = doge_asset_id {
        let order_req = OrderRequest {
            a: asset_idx as u32,
            b: true,
            p: doge_price.to_string(),
            s: "100.0".to_string(),
            r: false,
            t: OrderType::Limit(limit_order_type),
            c: None,
        };

        let order_action = OrderAction {
            type_: "order".to_owned(),
            orders: vec![order_req],
            grouping: Grouping::Na,
            builder: None,
        };

        let nonce = current_time_millis();
        let sig = sign_l1_action(&pkey_signer, &order_action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        tracing::info!("Attempting to place order.");

        let order_response = exchange_api
            .place_order(order_action, nonce, signature, None, None)
            .await?;
        tracing::info!("{:?}", order_response);
    }

    Ok(())
}
