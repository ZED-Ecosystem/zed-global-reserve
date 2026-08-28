use std::collections::HashMap;

pub struct SovereignReserveVault {
    pub asset_reserves: HashMap<String, u128>,
    pub total_pol_yield_accumulated: u128,
    pub is_autonomous_routing_active: bool,
}

impl SovereignReserveVault {
    pub fn new() -> Self {
        Self {
            asset_reserves: HashMap::new(),
            total_pol_yield_accumulated: 0,
            is_autonomous_routing_active: true,
        }
    }

    pub fn deposit_reserve_asset(&mut self, asset_symbol: &str, amount: u128) {
        let reserve = self.asset_reserves.entry(asset_symbol.to_string()).or_insert(0);
        *reserve += amount;
        println!("[RESERVE DEPOSIT] Deposit: {} units of {}", amount, asset_symbol);
    }

    pub fn receive_pol_fee_yield(&mut self, amount_zed: u128) {
        self.total_pol_yield_accumulated += amount_zed;
        println!("[POL YIELD ROUTED] Received {} ℤ from GSwap 0.03% POL fee allocation", amount_zed);
        println!("Total POL Reserve Yield Accumulated: {} ℤ", self.total_pol_yield_accumulated);
    }
}

fn main() {
    println!("=== ℤ ZED Sovereign Reserve & POL Vault Engine ===");
    let mut vault = SovereignReserveVault::new();

    vault.deposit_reserve_asset("GOLD_TOKEN", 500_000);
    vault.deposit_reserve_asset("USDC", 10_000_000);

    vault.receive_pol_fee_yield(30);
    println!("Autonomous Vault Routing Active: {}", vault.is_autonomous_routing_active);
}
