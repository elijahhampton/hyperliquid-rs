use crate::error::Result;
use crate::{
    api::request::post_json,
    client::HyperliquidClient,
    types::exchange::{
        AgentEnableDexAbstractionAction, ApproveAgentAction, ApproveBuilderFeeAction,
        BatchModifyAction, CDepositAction, CWithdrawAction, CancelResponseData, DefaultResponse,
        ModifyAction, OrderAction, OrderResponse, ReserveRequestWeightAction, ScheduleCancelAction,
        SendAssetAction, TokenDelegateAction, TwapCancelAction, TwapCancelResponse,
        TwapOrderAction, TwapOrderResponse, TwapOrderResponseData, UpdateIsolatedMarginAction,
        UpdateLeverageAction, UsdClassTransferAction, UsdSendAction, UserDexAbstractionAction,
        ValidatorL1StreamAction, VaultTransferAction, WithdrawAction,
    },
};
use alloy::signers::Signature;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Exchange endpoint
/// The exchange endpoint is used to interact with and trade on the Hyperliquid chain.

/// Asset
/// Many of the requests take asset as an input. For perpetuals this is the index in
/// the universe field returned by themeta response. For spot assets, use 10000 +
/// index where index is the corresponding index in spotMeta.universe. For example,
/// when submitting an order for PURR/USDC, the asset that should be used is 10000
/// because its asset index in the spot metadata is 0.

/// Subaccounts and vaults
/// Subaccounts and vaults do not have private keys. To perform actions on behalf of
/// a subaccount or vault signing should be done by the master account and the vaultAddress
/// field should be set to the address of the subaccount or vault. .

/// Expires After
/// Some actions support an optional field expiresAfter which is a timestamp in milliseconds
/// after which the action will be rejected. User-signed actions such as Core USDC transfer
/// do not support the expiresAfter field. Note that actions consume 5x the usual address-based
/// rate limit when canceled due to a stale expiresAfter field.

pub struct ExchangeApi<'a> {
    client: &'a HyperliquidClient,
}

impl<'a> ExchangeApi<'a> {
    /// Creates a [`ExchangeApi`].
    pub fn new(client: &'a HyperliquidClient) -> Self {
        Self { client }
    }

    /// Executes a POST request and returns the result.
    async fn post<T>(&self, payload: serde_json::Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        post_json(
            self.client.http_client(),
            &format!("{}/info", self.client.base_url()),
            payload,
        )
        .await
    }

    pub async fn place_order(
        &self,
        action: OrderAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<OrderResponse> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
        }

        self.post(payload).await
    }

    pub async fn cancel_order(
        &self,
        action: OrderAction,
        nonce: u64,
        signature: Signature,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<CancelResponseData> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
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
    ) -> Result<CancelResponseData> {
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        if let Some(vault_address) = vault_address {
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
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
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
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
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
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
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
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
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
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
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
        }

        self.post(payload).await
    }

    pub async fn core_usdc_transfer(
        &self,
        action: UsdSendAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
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
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
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
            payload["vaultAddress"] = json!(vault_address);
        }

        if let Some(expires_after) = expires_after {
            payload["expiresAfter"] = json!(expires_after);
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
            payload["expiresAfter"] = json!(expires_after);
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
            payload["expiresAfter"] = json!(expires_after);
        }

        self.post(payload).await
    }

    pub async fn enable_hip3_dex_abstraction(
        &self,
        action: UserDexAbstractionAction,
        nonce: u64,
        signature: Signature,
    ) -> Result<DefaultResponse> {
        let mut payload = json!({
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
        let mut payload = json!({
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
        let mut payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }
}
