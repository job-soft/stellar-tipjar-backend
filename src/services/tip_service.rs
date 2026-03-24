use anyhow::Result;

use crate::controllers::tip_controller;
use crate::db::connection::AppState;
use crate::models::tip::{RecordTipRequest, Tip};

#[allow(dead_code)]
pub struct TipService;

#[allow(dead_code)]
impl TipService {
    pub fn new() -> Self {
        Self
    }

    /// Record a new tip after optionally verifying the transaction on-chain.
    pub async fn record_tip(&self, state: &AppState, req: RecordTipRequest) -> Result<Tip> {
        tip_controller::record_tip(state, req).await
    }

    /// Retrieve all tips for a given creator username.
    pub async fn get_tips_for_creator(&self, state: &AppState, username: &str) -> Result<Vec<Tip>> {
        tip_controller::get_tips_for_creator(state, username).await
    }
}
