use crate::types::info::{AllMids, L2BookSnapshot};
use crate::HyperliquidClient;
use std::sync::Arc;
use crossterm::event::KeyCode;

pub enum View {
    MarketList,
    OrderBook,
}

pub struct App {
    client: Arc<HyperliquidClient>,
    pub all_mids: Option<AllMids>,
    pub orderbook: Option<L2BookSnapshot>,
    pub selected_coin: Option<String>,
    pub current_view: View,
    pub should_quit: bool,
    pub error_message: Option<String>,
    pub market_list_index: usize,
    pub needs_orderbook_refresh: bool,
    pub tick_count: u32

}

impl App {
    pub fn new(client: HyperliquidClient) -> Self {
        Self {
            client: Arc::new(client),
            all_mids: None,
            orderbook: None,
            selected_coin: None,
            current_view: View::MarketList,
            should_quit: false,
            error_message: None,
            market_list_index: 0,
            needs_orderbook_refresh: false,
            tick_count: 0
        }
    }

    pub async fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('r') => {
                self.refresh_data().await;
            }
            KeyCode::Tab => {
                self.current_view = match self.current_view {
                    View::MarketList => View::OrderBook,
                    View::OrderBook => View::MarketList,
                };
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Some(ref mids) = self.all_mids {
                    if self.market_list_index < mids.len().saturating_sub(1) {
                        self.market_list_index += 1;
                        self.update_selected_coin();
                         self.needs_orderbook_refresh = true;
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.market_list_index > 0 {
                    self.market_list_index = self.market_list_index.saturating_sub(1);
                    self.update_selected_coin();
                }
            }
            KeyCode::Enter => {
                self.current_view = View::OrderBook;
                self.refresh_orderbook().await;
            }
            _ => {}
        }
    }

   pub async fn handle_tick(&mut self) {
        self.tick_count += 1;

        match self.current_view {
            View::MarketList => {
                // Refresh market data every ~2 seconds (8 ticks * 250ms)
                if self.tick_count % 8 == 0 {
                    self.refresh_data().await;
                }
            }
            View::OrderBook => {
                // Refresh orderbook immediately if flagged, otherwise every ~1 second
                if self.needs_orderbook_refresh || (self.tick_count % 4 == 0 && self.selected_coin.is_some()) {
                    self.refresh_orderbook().await;
                    self.needs_orderbook_refresh = false;
                }
            }
        }
    }

    fn update_selected_coin(&mut self) {
        if let Some(ref mids) = self.all_mids {
            let coins: Vec<_> = mids.keys().collect();
            if let Some(coin) = coins.get(self.market_list_index) {
                self.selected_coin = Some((*coin).clone());
            }
        }
    }

    async fn refresh_data(&mut self) {
        match self.client.info().all_mids(None).await {
            Ok(mids) => {
                self.all_mids = Some(mids);
                self.error_message = None;

                if self.selected_coin.is_none() {
                    self.update_selected_coin();
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Error: {}", e));
            }
        }
    }

    async fn refresh_orderbook(&mut self) {
        if let Some(ref coin) = self.selected_coin {
            match self.client.info().l2_book_snapshot(coin, None, None).await {
                Ok(book) => {
                    self.orderbook = Some(book);
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Orderbook error: {}", e));
                }
            }
        }
    }
}
