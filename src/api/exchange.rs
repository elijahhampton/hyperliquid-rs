use crate::api::response::{
    AgentEnableDexAbstractionResponse, ApproveAgentResponse, ApproveBuilderFeeResponse,
    BatchModifyResponse, CDepositResponse, CWithdrawResponse, CancelResponse, DefaultResponse,
    ModifyResponse, NoopResponse, OrderResponse, ScheduleCancelResponse, SendAssetResponse,
    TokenDelegateResponse, TwapCancelResponse, TwapOrderResponse, UpdateIsolatedMarginResponse,
    UpdateLeverageResponse, UsdClassTransferResponse, UsdSendResponse, UserDexAbstractionResponse,
    ValidatorL1StreamResponse, VaultTransferResponse, WithdrawResponse,
};
use crate::error::{HyperliquidError, Result};
use crate::signature::sign::{sig_v_from_bool, sign_l1_action, sign_typed_data};
use crate::types::exchange::{
    Builder, CancelAction, CancelRequest, Grouping, ModifyRequest, NoopAction, OrderRequest,
    TwapRequest,
};
use crate::types::signature::Eip712Signature;
use crate::utils::current_time_millis;
use crate::{
    api::request_util::post_json,
    client::HyperliquidClient,
    types::exchange::{
        AgentEnableDexAbstractionAction, ApproveAgentAction, ApproveBuilderFeeAction,
        BatchModifyAction, CDepositAction, CWithdrawAction, ModifyAction, OrderAction,
        ReserveRequestWeightAction, ScheduleCancelAction, SendAssetAction, TokenDelegateAction,
        TwapCancelAction, TwapOrderAction, UpdateIsolatedMarginAction, UpdateLeverageAction,
        UsdClassTransferAction, UsdSendAction, UserDexAbstractionAction, ValidatorL1StreamAction,
        VaultTransferAction, WithdrawAction,
    },
};
use alloy::primitives::Address;
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

    /// Place one or more orders.
    ///
    /// # Time-in-Force Options
    ///
    /// * `Alo` - Add liquidity only (post only), canceled if it would immediately match
    /// * `Ioc` - Immediate or cancel, unfilled portion is canceled
    /// * `Gtc` - Good til canceled
    ///
    /// # Arguments
    ///
    /// * `orders` - List of order requests
    /// * `grouping` - Order grouping type (`Na`, `NormalTpsl`, or `PositionTpsl`)
    /// * `builder` - Optional builder fee configuration
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
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

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            false,
        )?;

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

    /// Cancel one or more orders.
    ///
    /// # Arguments
    ///
    /// * `cancels` - List of cancellation requests, each specifying asset index and order ID
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn cancel_order(
        &self,
        cancel: CancelRequest,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<CancelResponse> {
        self.bulk_cancel_orders(vec![cancel], vault_address, expires_after)
            .await
    }

    /// Cancel one or more orders.
    ///
    /// # Arguments
    ///
    /// * `cancels` - cancellation requests, each specifying asset index and order ID
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
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

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            false,
        )?;

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

    /// Schedule a cancel-all operation at a future time (dead man's switch).
    ///
    /// Omitting `time` removes any existing scheduled cancel. The scheduled time must be
    /// at least 5 seconds in the future. Maximum 10 triggers per day, resetting at 00:00 UTC.
    ///
    /// # Arguments
    ///
    /// * `time` - Optional timestamp in milliseconds to trigger cancel-all
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn schedule_cancel(
        &self,
        time: Option<u64>,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<ScheduleCancelResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let action = ScheduleCancelAction {
            type_: "scheduleCancel".to_string(),
            time,
        };

        let nonce = current_time_millis();

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

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

    /// Modify an existing order.
    ///
    /// # Arguments
    ///
    /// * `oid` - Order ID or client order ID (cloid) to modify
    /// * `order` - New order parameters
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn modify_an_order(
        &self,
        request: ModifyRequest,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<ModifyResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = ModifyAction {
            type_: "modify".to_string(),
            oid: request.oid,
            order: request.order,
        };

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

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

    /// Modify multiple orders in a single request.
    ///
    /// # Arguments
    ///
    /// * `modifies` - List of order modifications, each containing an order ID and new order parameters
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn modify_multiple_orders(
        &self,
        requests: Vec<ModifyRequest>,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<BatchModifyResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = BatchModifyAction {
            type_: "batchModify".to_string(),
            modifies: requests,
        };

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

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

    /// Add or remove margin from an isolated position.
    ///
    /// # Arguments
    ///
    /// * `asset` - Asset index
    /// * `is_buy` - Position side (no effect until hedge mode is introduced)
    /// * `ntli` - Amount to add/remove with 6 decimals (e.g., 1000000 for 1 USD)
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn update_isolated_margin(
        &self,
        asset: u32,
        is_buy: bool,
        ntli: u32,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<UpdateIsolatedMarginResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = UpdateIsolatedMarginAction {
            type_: "updateIsolatedMargin".to_string(),
            asset,
            is_buy,
            ntli,
        };

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

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

    /// Update cross or isolated leverage on an asset.
    ///
    /// # Arguments
    ///
    /// * `asset` - Asset index
    /// * `is_cross` - `true` for cross leverage, `false` for isolated
    /// * `leverage` - New leverage value (subject to per-asset constraints)
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn update_leverage(
        &self,
        asset: u32,
        is_cross: bool,
        leverage: u32,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<UpdateLeverageResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = UpdateLeverageAction {
            type_: "updateLeverage".to_string(),
            asset,
            is_cross,
            leverage,
        };

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

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

    /// Send USDC to another address on Hyperliquid L1.
    ///
    /// This transfer does not touch the EVM bridge.
    ///
    /// # Arguments
    ///
    /// * `destination` - Recipient address
    /// * `amount` - Amount in USD as a decimal string
    pub async fn core_usdc_transfer(
        &self,
        destination: Address,
        amount: String,
        time: u64,
    ) -> Result<UsdSendResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();
        let action = UsdSendAction {
            type_: "usdSend".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            destination: destination.to_string(),
            amount,
            time,
        };

        let sig = sign_typed_data(&action, &signer)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Initiate a withdrawal to the Arbitrum bridge.
    ///
    /// L1 validators will sign and send the withdrawal request to the bridge contract.
    /// Withdrawals have a $1 fee and take approximately 5 minutes to finalize.
    ///
    /// # Arguments
    ///
    /// * `destination` - Destination address on Arbitrum
    /// * `amount` - Amount in USD as a decimal string
    pub async fn initiate_withdrawal_request(
        &self,
        amount: String,
        time: u64,
        destination: Address,
    ) -> Result<WithdrawResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = WithdrawAction {
            type_: "withdraw3".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            amount,
            time,
            destination: destination.to_string(),
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Transfer USDC between spot and perp wallets.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount in USD as a decimal string (e.g., "1" for 1 USD)
    /// * `to_perp` - `true` to transfer spot → perp, `false` for perp → spot
    pub async fn transfer_to_or_from_spot_account_to_or_from_perp_account(
        &self,
        amount: String,
        to_perp: bool,
    ) -> Result<UsdClassTransferResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = UsdClassTransferAction {
            type_: "usdClassTransfer".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            amount,
            to_perp,
            nonce,
        };

        let sig = sign_typed_data(&action, &signer)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Transfer tokens between perp DEXs, spot balances, users, and/or sub-accounts.
    ///
    /// Use `""` to specify the default USDC perp DEX and `"spot"` to specify spot.
    /// Only the collateral token can be transferred to or from a perp DEX.
    ///
    /// # Arguments
    ///
    /// * `destination` - Destination address
    /// * `source_dex` - Source perp DEX name (or `""` for default, `"spot"` for spot)
    /// * `destination_dex` - Destination perp DEX name (or `""` for default, `"spot"` for spot)
    /// * `token` - Token identifier (e.g., "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2")
    /// * `amount` - Amount as a decimal string
    /// * `from_sub_account` - Sub-account address, or empty string if not from a sub-account
    pub async fn send_asset(
        &self,
        destination: Address,
        source_dex: Address,
        destination_dex: Address,
        token: String,
        amount: String,
        from_sub_account: Option<String>,
    ) -> Result<SendAssetResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let from_sub_account_param = from_sub_account.unwrap_or_default();

        let action = SendAssetAction {
            type_: "sendAsset".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            destination: destination.to_string(),
            source_dex: source_dex.to_string(),
            destination_dex: destination_dex.to_string(),
            token,
            amount,
            from_sub_account: from_sub_account_param,
            nonce,
        };

        let sig = sign_typed_data(&action, &signer)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Deposit native tokens from the spot account into staking.
    ///
    /// Deposited tokens can then be delegated to validators.
    ///
    /// # Arguments
    ///
    /// * `wei` - Amount in wei to deposit
    pub async fn deposit_into_staking(&self, wei: u64) -> Result<CDepositResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = CDepositAction {
            type_: "cDeposit".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            wei,
            nonce,
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Withdraw native tokens from staking into the spot account.
    ///
    /// Withdrawals go through a 7 day unstaking queue.
    ///
    /// # Arguments
    ///
    /// * `wei` - Amount in wei to withdraw
    pub async fn withdraw_from_staking(&self, wei: u64) -> Result<CWithdrawResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = CWithdrawAction {
            type_: "cWithdraw".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            wei,
            nonce,
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Delegate or undelegate native tokens to or from a validator.
    ///
    /// Delegations have a lockup duration of 1 day.
    ///
    /// # Arguments
    ///
    /// * `validator` - Validator address
    /// * `is_undelegate` - `true` to undelegate, `false` to delegate
    /// * `wei` - Amount in wei
    pub async fn delegate_or_undelegate_from_validator(
        &self,
        validator: String,
        is_undelegate: bool,
        wei: u64,
    ) -> Result<TokenDelegateResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = TokenDelegateAction {
            type_: "tokenDelegate".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            is_undelegate,
            wei,
            nonce,
            validator,
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Deposit or withdraw funds from a vault.
    ///
    /// # Arguments
    ///
    /// * `vault_address` - Vault address
    /// * `is_deposit` - `true` to deposit, `false` to withdraw
    /// * `usd` - Amount in USD
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn deposit_or_withdraw_from_a_vault(
        &self,
        vault_address: String,
        is_deposit: bool,
        usd_amount: u64,
        expires_after: Option<u64>,
    ) -> Result<VaultTransferResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let action = VaultTransferAction {
            type_: "vaultTransfer".to_string(),
            vault_address: vault_address.clone(),
            is_deposit,
            usd: usd_amount,
        };

        let nonce = current_time_millis();

        let sig = sign_l1_action(
            &signer,
            &action,
            Some(vault_address),
            nonce,
            expires_after,
            false,
        )?;

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

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    /// Approve an API wallet (agent wallet) to trade on behalf of this account.
    ///
    /// An account can have 1 unnamed approved wallet and up to 3 named ones.
    /// An additional 2 named agents are allowed per subaccount.
    ///
    /// # Arguments
    ///
    /// * `agent_address` - Address of the API wallet to approve
    /// * `agent_name` - Optional name for the API wallet
    pub async fn approve_an_api_wallet(
        &self,
        agent_address: String,
        agent_name: Option<String>,
    ) -> Result<ApproveAgentResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = ApproveAgentAction {
            type_: "approveBuilderFee".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            nonce,
            agent_address,
            agent_name,
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Approve a maximum fee rate for a builder.
    ///
    /// # Arguments
    ///
    /// * `builder` - Builder address to approve
    /// * `max_fee_rate` - Maximum allowed fee rate as a percent string (e.g., "0.001%")
    pub async fn approve_a_builder_fee(
        &self,
        // The maximum allowed builder fee rate as a percent string; e.g. "0.001%",
        max_fee_rate: String,
        builder: String,
    ) -> Result<ApproveBuilderFeeResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = ApproveBuilderFeeAction {
            type_: "approveBuilderFee".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            max_fee_rate,
            builder,
            nonce,
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Place a TWAP (Time-Weighted Average Price) order.
    ///
    /// Executes a large order in smaller chunks over a specified time period
    /// to minimize market impact.
    ///
    /// # Arguments
    ///
    /// * `asset` - Asset index
    /// * `is_buy` - `true` for buy, `false` for sell
    /// * `size` - Order size as a decimal string
    /// * `reduce_only` - If `true`, only reduces existing position
    /// * `minutes` - Duration to execute the order over
    /// * `randomize` - If `true`, randomizes execution timing
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn place_twap_order(
        &self,
        request: TwapRequest,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<TwapOrderResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = TwapOrderAction {
            type_: "twapOrder".to_string(),
            twap: request,
        };

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

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

    /// Cancel an active TWAP order.
    ///
    /// # Arguments
    ///
    /// * `asset` - Asset index
    /// * `twap_id` - TWAP order ID to cancel
    /// * `vault_address` - Optional vault or subaccount address if trading on behalf of one
    /// * `expires_after` - Optional expiration timestamp in milliseconds
    pub async fn cancel_twap_order(
        &self,
        asset: usize,
        twap_id: u32,
        vault_address: Option<String>,
        expires_after: Option<u64>,
    ) -> Result<TwapCancelResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = TwapCancelAction {
            type_: "twapCancel".to_string(),
            a: asset,
            t: twap_id,
        };

        let sig = sign_l1_action(
            &signer,
            &action,
            vault_address.clone(),
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

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

    /// Instead of trading to increase the address based rate limits, this action
    /// allows reserving additional actions for 0.0005 USDC per request.
    /// The cost is paid from the Perps balance.
    pub async fn reserve_additional_actions(
        &self,
        weight: u32,
        expires_after: Option<u64>,
    ) -> Result<DefaultResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = ReserveRequestWeightAction {
            type_: "reserveRequestWeight".to_string(),
            weight,
        };

        let sig = sign_l1_action(
            &signer,
            &action,
            None,
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

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

        if let Some(expires_after) = expires_after {
            payload
                .as_object_mut()
                .ok_or_else(|| HyperliquidError::Internal("payload wrongly formatted".to_owned()))?
                .insert("expiresAfter".to_owned(), json!(expires_after));
        }

        self.post(payload).await
    }

    /// This action does not do anything (no operation), but causes the nonce to be marked as used. This can be a more effective way
    /// to cancel in-flight orders than the cancel action.
    pub async fn invalidate_pending_nonce_noop(
        &self,
        expires_after: Option<u64>,
    ) -> Result<NoopResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = NoopAction {
            type_: "noop".to_string(),
        };

        let sig = sign_l1_action(
            &signer,
            &action,
            None,
            nonce,
            expires_after,
            self.client.is_mainnet(),
        )?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

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

    /// If set, actions on HIP-3 perps will automatically transfer collateral from
    /// validator-operated USDC perps balance for HIP-3 DEXs where USDC is the
    /// collateral token, and spot otherwise. When HIP-3 DEX abstraction is active,
    /// collateral is returned to the same source (validator-operated USDC perps or
    /// spot balance) when released from positions or open orders.
    pub async fn enable_hip3_dex_abstraction(&self) -> Result<UserDexAbstractionResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = UserDexAbstractionAction {
            type_: "userDexAbstraction".to_string(),
            hyperliquid_chain: self.client.network_type().hyperliquid_chain(),
            signature_chain_id: self.client.network_type().signature_chain_id(),
            user: signer.address().to_string(),
            enabled: true,
            nonce,
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Same effect as `enable_hip3_dex_abstraction`, but only works if setting
    /// the value from null to true.
    pub async fn enable_hip3_dex_abstraction_agent(
        &self,
    ) -> Result<AgentEnableDexAbstractionResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = AgentEnableDexAbstractionAction {
            type_: "agentEnableDexAbstraction".to_string(),
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }

    /// Submit a validator vote for the risk-free rate on an aligned quote asset.
    ///
    /// # Arguments
    ///
    /// * `risk_free_rate` - Rate as a decimal string (e.g., "0.04" for 4%)
    pub async fn validator_vote_on_risk_free_rate_for_aligned_quote_asset(
        &self,
        risk_free_rate: String,
    ) -> Result<ValidatorL1StreamResponse> {
        let signer = self
            .client
            .signer()
            .ok_or_else(|| HyperliquidError::SignerRequired)?;

        let nonce = current_time_millis();

        let action = ValidatorL1StreamAction {
            type_: "validatorL1Stream".to_string(),
            risk_free_rate,
        };

        let sig = sign_l1_action(&signer, &action, None, nonce, None, false)?;

        let signature = Eip712Signature {
            r: format!("0x{:x}", sig.r()),
            s: format!("0x{:x}", sig.s()),
            v: sig_v_from_bool(sig.v()),
        };

        let payload = json!({
            "action": action,
            "nonce": nonce,
            "signature": signature
        });

        self.post(payload).await
    }
}
