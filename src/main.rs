use std::collections::HashMap;

pub struct ReserveVault {
    pub assets: HashMap<String, u128>,
    pub pol_escrow_balance: u128,
    pub is_autonomous_yield_enabled: bool,
}

impl ReserveVault {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            pol_escrow_balance: 0,
            is_autonomous_yield_enabled: true,
        }
    }

    pub fn deposit_asset(&mut self, ticker: &str, amount: u128) {
        let entry = self.assets.entry(ticker.to_string()).or_insert(0);
        *entry += amount;
        println!("[RESERVE DEPOSIT] Added {} units of {}", amount, ticker);
    }

    pub fn route_pol_yield(&mut self, yield_amount: u128) {
        self.pol_escrow_balance += yield_amount;
        println!("[POL YIELD ROUTED] Escrow balance now: {}", self.pol_escrow_balance);
    }
}

fn main() {
    println!("=== ZED Sovereign Reserve & POL Vault Engine ===");
    let mut vault = ReserveVault::new();

    vault.deposit_asset("GOLD_TOKEN", 500_000);
    vault.deposit_asset("RAW_ASSET_BASKET", 1_200_000);
    vault.route_pol_yield(15_000);

    println!("Reserve Status: Active. Autonomous Yield: {}", vault.is_autonomous_yield_enabled);
}
