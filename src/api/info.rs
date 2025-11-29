use crate::{
    api::{SUPPORTED_INTERVALS, current_time_millis, request::post_json},
    client::HyperliquidClient,
    error::Result,
    types::{
        info::{
            AlignedQuoteTokenInfo, AllMids, BuilderFeeApproval, CandleSnapshot, ClearinghouseState,
            FrontendOpenOrders, L2BookSnapshot, OpenOrders, OrderId, OrderStatus, OrderWithStatus,
            TwapSliceFills, UserFees, UserFills, UserHistoricalOrders, UserPortfolio,
            UserRateLimits, UserReferralInformation, UserRole, UserStakingDelegations,
            UserStakingRewards, UserStakingSummary, UserSubAccounts, UserVaultDeposits,
            perpetual::{
                ActiveAssetData, FundingHistory, FundingRate, LedgerUpdates,
                PerpDeployAuctionStatus, PerpDexLimits, PerpetualDexs, PerpetualsMetadata,
                PerpsAtOpenInterestCap, VenueFundings,
            },
            spot::{
                SpotAssetContext, SpotClearinghouseState, SpotDeployState,
                SpotMetaAndAssetContexts, SpotMetadata, SpotPairDeployAuctionStatus, TokenDetails,
            },
            user::{CandleSnapshotRequest, UserStakingHistory},
        },
        ws::PerpDexState,
    },
};
/// The info endpoint is used to fetch information about the exchange and specific users.
use serde_json::json;
use std::string::ToString;

/// The Info API: [`InfoApi`] covers all endpoint types requested with the /info path.
///
/// Pagination
/// Responses that take a time range will only return 500 elements or distinct blocks of data. To query
/// larger ranges, use the last returned timestamp as the next startTime for pagination.

/// Perpetuals vs Spot
/// The endpoints in this section as well as websocket subscriptions work for both Perpetuals and Spot.
/// For perpetuals coin is the name returned in the meta response. For Spot, coin should be PURR/USDC for PURR, and @{index} e.g. @1
/// for all other spot tokens where index is the index of the spot pair in the universe field of the spotMeta response.
/// For example, the spot index for HYPE on mainnet is @107 because the token index of HYPE is 150 and the spot pair @107 has tokens [150, 0].
/// Note that some assets may be remapped on user interfaces. For example, BTC/USDC on app.hyperliquid.xyz corresponds to UBTC/USDC on mainnet HyperCore.
/// The L1 name on the token details page can be used to detect remappings.

/// User address
/// To query the account data associated with a master or sub-account, you must pass in the actual address of that
/// account. A common pitfall is to use an agent wallet's address which leads to an empty result.
pub struct InfoApi<'a> {
    client: &'a HyperliquidClient,
}

impl<'a> InfoApi<'a> {
    /// Creates a [`InfoApi`].
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

    /// Retrieves mids for all coins.
    /// If the book is empty, the last trade price will be used as a fallback.
    pub async fn all_mids(&self, dex: Option<String>) -> Result<AllMids> {
        let dex_param = dex.unwrap_or("".to_string());
        let payload = json!({
            "type": "allMids",
            "dex": json!(dex_param)
        });

        self.post(payload).await
    }

    /// Retrieves a user's open orders.
    ///
    /// # Arguments
    /// * `user`    - Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    /// * `dex`     - Perp dex name. Defaults to the empty string which represents the first perp dex. Spot open orders
    /// are only included with the first perp dex.
    ///
    /// # Returns
    /// A [`OpenOrders`] containing the list of open orders for a user.
    pub async fn open_orders(&self, user: &str, dex: Option<&str>) -> Result<OpenOrders> {
        let payload = json!({
            "type": "openOrders",
            "user": user,
            "dex": dex.unwrap_or("")
        });

        self.post(payload).await
    }

    /// Retrieves a user's open orders with additional frontend info.
    ///
    /// # Arguments
    /// * `user`    - Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    /// * `dex`     - Perp dex name. Defaults to the empty string which represents the first perp dex. Spot open orders
    /// are only included with the first perp dex.
    ///
    /// # Returns
    /// A [`FrontendOpenOrders`] containing the list of open orders with additional information for a user.
    pub async fn open_orders_with_additional_info(
        &self,
        user: &str,
        dex: Option<&str>,
    ) -> Result<FrontendOpenOrders> {
        let mut payload = json!({
            "type": "frontendOpenOrders",
            "user": user
        });

        if let Some(dex) = dex.filter(|s| !s.is_empty()) {
            payload["dex"] = json!(dex);
        }

        self.post(payload).await
    }

    /// Retrieves a user's fills.
    ///
    /// # Arguments
    /// * `user`    - Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    /// * `aggregateByTime`     - When true, partial fills are combined when a crossing order gets filled by multiple
    /// different resting orders. Resting orders filled by multiple crossing orders are only aggregated if in the same block.
    ///
    /// # Returns
    /// [`UserFills`] containing the list of fills for a user.
    pub async fn fills(&self, user: &str, aggregate_by_time: Option<bool>) -> Result<UserFills> {
        let mut payload = json!({
            "type": "userFills",
            "user": user
        });

        if let Some(agg) = aggregate_by_time {
            payload["aggregateByTime"] = json!(agg);
        }

        self.post(payload).await
    }

    /// Retrieves a user's fills by time.
    ///
    /// # Arguments
    /// * `user` - Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    /// * `aggregateByTime` - When true, partial fills are combined when a crossing order gets filled by multiple
    /// different resting orders. Resting orders filled by multiple crossing orders are only aggregated if in the same block.
    /// * `startTime` - Start time in milliseconds, inclusive
    /// * `endTime` - End time in milliseconds, inclusive. Defaults to current time.
    /// * `aggregateByTime` - When true, partial fills are combined when a crossing order gets filled by multiple
    /// different resting orders. Resting orders filled by multiple crossing orders are only aggregated if in the same block.
    ///
    /// # Returns
    /// [`UserFills`] containing the list of fills for a user.
    pub async fn fills_by_time(
        &self,
        user: &str,
        start_time: u64,
        end_time: Option<u64>,
        aggregate_by_time: Option<bool>,
    ) -> Result<UserFills> {
        let mut payload = json!({
            "type": "userFillsByTime",
            "user": user,
            "startTime": start_time
        });

        if let Some(end_time) = end_time {
            payload["endTime"] = json!(end_time);
        }

        if let Some(agg) = aggregate_by_time {
            payload["aggregateByTime"] = json!(agg);
        }

        self.post(payload).await
    }

    /// Retrieves rate limits for the specified user.
    ///
    /// # Arguments
    /// * `user` - Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    ///
    /// # Returns
    /// The rate limits for for a user as [`UserRateLimits`].
    pub async fn rate_limits(&self, user: &str) -> Result<UserRateLimits> {
        let payload = json!({
            "type": "userRateLimit",
            "user": user,
        });

        self.post(payload).await
    }

    /// Retrieves an order status for the specified user.
    ///
    /// # Arguments
    /// * `user` - Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    /// * `oid` - Either u64 representing the order id or 16-byte hex string representing the client order id
    ///
    /// # Returns
    /// An order with the current status. Possible values for the order status string
    /// can be found here: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint
    pub async fn order_status(&self, user: &str, oid: OrderId) -> Result<OrderWithStatus> {
        let mut payload = json!({
            "type": "orderStatus",
            "user": user
        });

        match oid {
            OrderId::Numeric(id) => payload["oid"] = json!(id),
            OrderId::ClientId(client_id) => payload["oid"] = json!(client_id),
        };

        self.post(payload).await
    }

    /// Retrieves a snapshot of the orderbook. Returns at most 20 levels per side.
    ///
    /// # Arguments
    /// * `coin` - The desired coin to view the snapshot.
    /// * `nSigFigs` - Optional field to aggregate levels to nSigFigs significant figures. Valid values
    /// are 2, 3, 4, 5, and null, which means full precision.
    /// * `mantissa` - Optional field to aggregate levels. This field is only allowed if nSigFigs is 5.
    /// Accepts values of 1, 2 or 5.
    ///
    /// # Returns
    /// Returns an L2 snapshot of the orderbook for the specified coin as [`L2BookSnapshot`].
    pub async fn l2_book_snapshot(
        &self,
        coin: &str,
        n_sig_figs: Option<usize>,
        mantissa: Option<usize>,
    ) -> Result<L2BookSnapshot> {
        let mut payload = json!({
            "type": "l2Book",
            "coin": coin
        });

        match n_sig_figs {
            Some(2) | Some(3) | Some(4) | Some(5) => {
                payload["nSigFigs"] = json!(n_sig_figs);
            }
            Some(_) => {
                payload["nSigFigs"] = json!(null);
            }
            None => {
                payload["nSigFigs"] = json!(null);
            }
        }

        if n_sig_figs == Some(5) {
            if let Some(m) = mantissa {
                if [1, 2, 5].contains(&m) {
                    payload["mantissa"] = json!(m);
                }

                return Err(crate::error::HyperliquidError::InvalidRequestParameter {
                    method: "l2_book_snapshot".to_string(),
                    parameter: "mantissa".to_string(),
                    reason: format!("Invalid parameter {} for field {}", m, "mantissa"),
                });
            }
        }

        self.post(payload).await
    }

    /// Retrieves the most recent 5000 candles
    ///
    /// # Arguments
    /// * `req` - [`CandleSnapshotRequest`]. The supported time intervals are: "1m", "3m", "5m", "15m", "30m", "1h",
    /// "2h", "4h", "8h", "12h", "1d", "3d", "1w", "1M"
    ///
    /// # Returns
    /// The most recent 5000 candles
    pub async fn candle_snapshot(&self, req: CandleSnapshotRequest) -> Result<CandleSnapshot> {
        let payload = json!({
            "type": "candleSnapshot",
            "req": req
        });

        if !SUPPORTED_INTERVALS.contains(req.interval.as_str()) {
            return Err(crate::error::HyperliquidError::InvalidRequestParameter {
                method: "candle_snapshot".to_string(),
                parameter: "interval".to_string(),
                reason: format!("Expected one of: {:?}", SUPPORTED_INTERVALS),
            });
        }

        self.post(payload).await
    }

    /// Check the builder fee approval for the specific user.
    ///
    /// # Arguments
    /// * `user` - Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    /// * `builder` - Address in 42-character hexadecimal format; e.g. 0x0000000000000000000000000000000000000000.
    ///
    /// # Returns
    /// An integer representing the maximum fee approved in tenths of a basis point (i.e. 1 means 0.001%).
    pub async fn check_builder_fee_approval(
        &self,
        user: &str,
        builder: &str,
    ) -> Result<BuilderFeeApproval> {
        let payload = json!({
            "type": "maxBuilderFee",
            "user": user,
            "builder": builder
        });

        if user.len() != 42 {
            return Err(crate::error::HyperliquidError::InvalidRequestParameter {
                method: "check_builder_fee_approval".to_string(),
                parameter: "user".to_string(),
                reason: format!(
                    "Specified user parameter has incorrect length: {}",
                    user.len()
                ),
            });
        }

        if builder.len() != 42 {
            return Err(crate::error::HyperliquidError::InvalidRequestParameter {
                method: "check_builder_fee_approval".to_string(),
                parameter: "builder".to_string(),
                reason: format!(
                    "Specified builder parameter has incorrect length: {}",
                    builder.len()
                ),
            });
        }

        self.post(payload).await
    }

    pub async fn historical_orders(&self, user: &str) -> Result<UserHistoricalOrders> {
        let payload = json!({
            "type": "historicalOrders",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn twap_slice_fills(&self, user: &str) -> Result<TwapSliceFills> {
        let payload = json!({
            "type": "userTwapSliceFills",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn subaccounts(&self, user: &str) -> Result<UserSubAccounts> {
        let payload = json!({
            "type": "subAccounts",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn vault_details(
        &self,
        vault_address: &str,
        user: Option<String>,
    ) -> Result<UserSubAccounts> {
        let mut payload = json!({
            "type": "vaultDetails",
            "vaultAddress": vault_address,
        });

        if let Some(user) = user {
            payload["user"] = json!(user);
        }

        self.post(payload).await
    }

    pub async fn vault_deposits(&self, user: &str) -> Result<UserVaultDeposits> {
        let payload = json!({
            "type": "userVaultEquities",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn role(&self, user: &str) -> Result<UserRole> {
        let payload = json!({
            "type": "userRole",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn portfolio(&self, user: &str) -> Result<UserPortfolio> {
        let payload = json!({
            "type": "portfolio",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn referral_information(&self, user: &str) -> Result<UserReferralInformation> {
        let payload = json!({
            "type": "referral",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn fees(&self, user: &str) -> Result<UserFees> {
        let payload = json!({
            "type": "userFees",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn staking_delegations(&self, user: &str) -> Result<UserStakingDelegations> {
        let payload = json!({
            "type": "delegations",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn staking_summary(&self, user: &str) -> Result<UserStakingSummary> {
        let payload = json!({
            "type": "delegatorSummary",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn staking_history(&self, user: &str) -> Result<UserStakingHistory> {
        let payload = json!({
            "type": "delegatorHistory",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn staking_rewards(&self, user: &str) -> Result<UserStakingRewards> {
        let payload = json!({
            "type": "delegatorRewards",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn hip3_dex_abstraction_state(&self, user: &str) -> Result<Option<bool>> {
        let payload = json!({
            "type": "userDexAbstraction",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn aligned_quote_token_status(&self, user: &str) -> Result<AlignedQuoteTokenInfo> {
        let payload = json!({
            "type": "alignedQuoteTokenInfo",
            "user": user
        });

        self.post(payload).await
    }

    // Functions specific to Perpetuals

    pub async fn perpetual_dexs(&self) -> Result<PerpetualDexs> {
        let payload = json!({
            "type": "perpDexs",
        });

        self.post(payload).await
    }

    pub async fn perpetuals_metadata(&self, dex: Option<String>) -> Result<PerpetualsMetadata> {
        let payload = json!({
            "type": "meta",
            "dex": dex.unwrap_or("".to_string())
        });

        self.post(payload).await
    }

    pub async fn perpetuals_asset_contexts(&self) -> Result<PerpetualsMetadata> {
        let payload = json!({
            "type": "meta",
        });

        self.post(payload).await
    }

    pub async fn perpetuals_account_summary(
        &self,
        user: String,
        dex: Option<String>,
    ) -> Result<ClearinghouseState> {
        let payload = json!({
            "type": "clearinghouseState",
            "user": user,
            "dex": dex.unwrap_or("".to_string())
        });

        self.post(payload).await
    }

    pub async fn funding_history_updates(
        &self,
        user: String,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<LedgerUpdates> {
        let mut payload = json!({
            "type": "userFunding",
            "user": user,
            "startTime": start_time,
        });

        if let Some(end_time) = end_time {
            payload["endTime"] = json!(end_time);
        } else {
            payload["endTime"] = json!(current_time_millis());
        }

        self.post(payload).await
    }

    pub async fn non_funding_ledger_updates(
        &self,
        user: String,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<LedgerUpdates> {
        let mut payload = json!({
            "type": "userNonFundingLedgerUpdates",
            "user": user,
            "startTime": start_time,
        });

        if let Some(end_time) = end_time {
            payload["endTime"] = json!(end_time);
        } else {
            payload["endTime"] = json!(current_time_millis());
        }

        self.post(payload).await
    }

    pub async fn historical_funding_rates(
        &self,
        coin: String,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<FundingHistory> {
        let mut payload = json!({
            "type": "fundingHistory",
            "coin": coin,
            "startTime": start_time,
        });

        if let Some(end_time) = end_time {
            payload["endTime"] = json!(end_time);
        } else {
            payload["endTime"] = json!(current_time_millis());
        }

        self.post(payload).await
    }

    pub async fn predicted_funding_rates_for_different_venues(&self) -> Result<VenueFundings> {
        let payload = json!({
            "type": "predictedFundings",
        });

        self.post(payload).await
    }

    pub async fn query_perps_at_open_interest_caps(&self) -> Result<PerpsAtOpenInterestCap> {
        let payload = json!({
            "type": "perpsAtOpenInterestCap",
        });

        self.post(payload).await
    }

    pub async fn perp_deploy_auction_information(&self) -> Result<PerpDeployAuctionStatus> {
        let payload = json!({
            "type": "perpDeployAuctionStatus",
        });

        self.post(payload).await
    }

    pub async fn active_asset_data(&self, user: String, coin: String) -> Result<ActiveAssetData> {
        let payload = json!({
            "type": "activeAssetData",
            "user": json!(user),
            "coin": json!(coin)
        });

        if user.len() != 42 {
            return Err(crate::error::HyperliquidError::InvalidRequestParameter {
                method: "active_asset_data".to_string(),
                parameter: "user".to_string(),
                reason: format!(
                    "Specified user parameter has incorrect length: {}",
                    user.len()
                ),
            });
        }

        self.post(payload).await
    }

    pub async fn builder_deployed_perp_market_limits(&self, dex: String) -> Result<PerpDexLimits> {
        let payload = json!({
            "type": "perpDexLimits",
            "dex": json!(dex)
        });

        if dex.is_empty() {
            return Err(crate::error::HyperliquidError::InvalidRequestParameter {
                method: "builder_deployed_perp_market_limits".to_string(),
                parameter: "dex".to_string(),
                reason: format!("The empty string is not allowed."),
            });
        }

        self.post(payload).await
    }

    pub async fn market_status(&self, dex: String) -> Result<PerpDexState> {
        let payload = json!({
            "type": "perpDexStatus",
            "dex": json!(dex)
        });

        self.post(payload).await
    }

    // Functions specific to Spot

    pub async fn spot_metadata(&self) -> Result<SpotMetadata> {
        let payload = json!({
            "type": "spotMeta",
        });

        self.post(payload).await
    }

    pub async fn spot_asset_context(&self) -> Result<SpotMetaAndAssetContexts> {
        let payload = json!({
            "type": "spotMetaAndAssetCtxs"
        });

        self.post(payload).await
    }

    pub async fn token_balances(&self, user: String) -> Result<SpotClearinghouseState> {
        let payload = json!({
            "type": "spotClearinghouseState",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn spot_deploy_auction_information(&self, user: String) -> Result<SpotDeployState> {
        let payload = json!({
            "type": "spotDeployState",
            "user": user
        });

        self.post(payload).await
    }

    pub async fn spot_pair_deploy_auction_information(
        &self,
    ) -> Result<SpotPairDeployAuctionStatus> {
        let payload = json!({
            "type": "spotPairDeployAuctionStatus"
        });

        self.post(payload).await
    }

    pub async fn token_information(&self) -> Result<TokenDetails> {
        let payload = json!({
            "type": "tokenDetails"
        });

        self.post(payload).await
    }
}
