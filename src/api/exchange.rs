use crate::api::{current_time_millis, CancelResponse, OrderResponse};
use crate::error::{HyperliquidError, Result};
use crate::signature::sign::{sig_v_from_bool, sign_l1_action};
use crate::types::exchange::{Builder, CancelAction, CancelRequest, Grouping, OrderRequest};
use crate::types::signature::Eip712Signature;
use crate::{
    api::request_util::post_json,
    client::HyperliquidClient,
    types::exchange::{
        AgentEnableDexAbstractionAction, ApproveAgentAction, ApproveBuilderFeeAction,
        BatchModifyAction, CDepositAction, CWithdrawAction, DefaultResponse, ModifyAction,
        OrderAction, ReserveRequestWeightAction, ScheduleCancelAction, SendAssetAction,
        TokenDelegateAction, TwapCancelAction, TwapCancelResponse, TwapOrderAction,
        TwapOrderResponse, UpdateIsolatedMarginAction, UpdateLeverageAction,
        UsdClassTransferAction, UsdSendAction, UserDexAbstractionAction, ValidatorL1StreamAction,
        VaultTransferAction, WithdrawAction,
    },
};
use alloy::signers::Signature;
use serde::de::DeserializeOwned;
use serde_json::json;

/// Exchange endpoint
/// The exchange endpoint is used to interact with and trade on the Hyperliquid chain.
///
/// Asset
/// Many of the requests take asset as an input. For perpetuals this is the index in
/// the universe field returned by themeta response. For spot assets, use 10000 +
/// index where index is the corresponding index in spotMeta.universe. For example,
/// when submitting an order for PURR/USDC, the asset that should be used is 10000
/// because its asset index in the spot metadata is 0.
///
/// Subaccounts and vaults
/// Subaccounts and vaults do not have private keys. To perform actions on behalf of
/// a subaccount or vault signing should be done by the master account and the vaultAddress
/// field should be set to the address of the subaccount or vault. .
///
/// Expires After
/// Some actions support an optional field expiresAfter which is a timestamp in milliseconds
/// after which the action will be rejected. User-signed actions such as Core USDC transfer
/// do not support the expiresAfter field. Note that actions consume 5x the usual address-based
/// rate limit when canceled due to a stale expiresAfter field.
///
pub struct ExchangeApi<'client> {
    client: &'client HyperliquidClient,
}

impl<'client> ExchangeApi<'client> {
    /// Creates a [`ExchangeApi`].
    pub fn new(client: &'client HyperliquidClient) -> Self {
        Self { client }
    }

    /// Executes a POST request and returns the result.
    async fn post<T>(&self, payload: serde_json::Value) -> Result<T>
    where
        T: DeserializeOwned,
    {
        post_json(
            self.client.http_client(),
            &format!("{}/exchange", self.client.base_url()),
            payload,
        )
        .await
    }

    /// Place an order on Hyperliquid.
    ///
    /// For limit orders, TIF (time-in-force) sets the behavior of the order upon first hitting the book.
    /// ALO (add liquidity only, i.e. "post only") will be canceled instead of immediately matching.
    /// IOC (immediate or cancel) will have the unfilled part canceled instead of resting.
    /// GTC (good til canceled) orders have no special behavior.
    /// Client Order ID (cloid) is an optional 128 bit hex string, e.g. 0x1234567890abcdef1234567890abcdef
    pub async fn place_order(
        &self,
        order: OrderRequest,
        grouping: Grouping,
        builder: Option<Builder>,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<OrderResponse> {
        self.bulk_orders(vec![order], grouping, builder, vault_address, expires_after)
            .await
    }

    /// Places multiple orders on Hyperliquid.
    ///
    /// For limit orders, TIF (time-in-force) sets the behavior of the order upon first hitting the book.
    /// ALO (add liquidity only, i.e. "post only") will be canceled instead of immediately matching.
    /// IOC (immediate or cancel) will have the unfilled part canceled instead of resting.
    /// GTC (good til canceled) orders have no special behavior.
    /// Client Order ID (cloid) is an optional 128 bit hex string, e.g. 0x1234567890abcdef1234567890abcdef
    pub async fn bulk_orders(
        &self,
        orders: Vec<OrderRequest>,
        grouping: Grouping,
        builder: Option<Builder>,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<OrderResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = OrderAction {
            type_: "order".to_owned(),
            orders,
            grouping,
            builder,
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn cancel_order(
        &self,
        cancel: CancelRequest,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<CancelResponse> {
        self.bulk_cancel_orders(vec![cancel], vault_address, expires_after)
            .await
    }

    pub async fn bulk_cancel_orders(
        &self,
        cancels: Vec<CancelRequest>,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<CancelResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let action = CancelAction {
            type_: "cancel".to_string(),
            cancels,
        };

        let nonce = current_time_millis();

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn cancel_order_by_cloid(
        &self,
        action: OrderAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<CancelResponse> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn schedule_cancel(
        &self,
        action: ScheduleCancelAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<()> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn modify_an_order(
        &self,
        action: ModifyAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<()> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn modify_multiple_orders(
        &self,
        action: BatchModifyAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<()> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn update_isolated_margin(
        &self,
        action: UpdateIsolatedMarginAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<DefaultResponse> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn update_leverage(
        &self,
        action: UpdateLeverageAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<DefaultResponse> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn core_usdc_transfer(
        &self,
        action: UsdSendAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn initiate_withdrawal_request(
        &self,
        action: WithdrawAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn transfer_to_or_from_spot_account_to_or_from_perp_account(
        &self,
        action: UsdClassTransferAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn send_asset(
        &self,
        action: SendAssetAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn deposit_into_staking(
        &self,
        action: CDepositAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn withdraw_from_staking(
        &self,
        action: CWithdrawAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn delegate_or_undelegate_from_validator(
        &self,
        action: TokenDelegateAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn deposit_or_withdraw_from_a_vault(
        &self,
        action: VaultTransferAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn approve_an_api_wallet(
        &self,
        action: ApproveAgentAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn approve_a_builder_fee(
        &self,
        action: ApproveBuilderFeeAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn place_twap_order(
        &self,
        action: TwapOrderAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<TwapOrderResponse> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn cancel_twap_order(
        &self,
        action: TwapCancelAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<TwapCancelResponse> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("vaultAddress".to_owned(), json!(vault_address));
        }

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn reserve_additional_actions(
        &self,
        action: ReserveRequestWeightAction,
        nonce: u64,
        signature: Signature,
        expires_after: Option<u64>,
    ) -> Result<DefaultResponse> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn invalidate_pending_nonce_noop(
        &self,
        nonce: u64,
        signature: Signature,
        expires_after: Option<u64>,
    ) -> Result<DefaultResponse> {
        let mut payload = json!({
            "action": json!({ "type": "noop" }),
            "nonce": nonce,
            "signature": signature
        });

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    pub async fn enable_hip3_dex_abstraction(
        &self,
        action: UserDexAbstractionAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn enable_hip3_dex_abstraction_agent(
        &self,
        action: AgentEnableDexAbstractionAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    pub async fn validator_vote_on_risk_free_rate_for_aligned_quote_asset(
        &self,
        action: ValidatorL1StreamAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }
}
