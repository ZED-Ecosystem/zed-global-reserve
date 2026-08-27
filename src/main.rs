use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AssetReserve {
    pub ticker: String,
    pub total_escrowed: u128,
    pub pol_yield_accrued: u128,
}

pub struct SovereignReserveVault {
    pub reserves: HashMap<String, AssetReserve>,
    pub is_autonomous_yield_enabled: bool,
    pub total_pol_fee_accumulated: u128,
}

impl SovereignReserveVault {
    pub fn new() -> Self {
        Self {
            reserves: HashMap::new(),
            is_autonomous_yield_enabled: true,
            total_pol_fee_accumulated: 0,
        }
    }

    /// Deposit backing assets (e.g. Gold, Stablecoins, Raw Commodities) into escrow
    pub fn deposit_backing_asset(&mut self, ticker: &str, amount: u128) {
        let reserve = self.reserves.entry(ticker.to_string()).or_insert(AssetReserve {
            ticker: ticker.to_string(),
            total_escrowed: 0,
            pol_yield_accrued: 0,
        });
        reserve.total_escrowed += amount;
        println!("[RESERVE DEPOSIT] Deposit: {} units of {}", amount, ticker);
    }

    /// Receive 0.03% POL yield directly from GSwap AMM volume
    pub fn receive_gswap_pol_fee(&mut self, fee_amount: u128) {
        self.total_pol_fee_accumulated += fee_amount;
        println!("[POL YIELD ROUTED] Received {} ZED from GSwap 0.03% POL fee allocation", fee_amount);
    }
}

fn main() {
    println!("=== ZED Sovereign Reserve & POL Vault Engine ===");
    let mut vault = SovereignReserveVault::new();

    // 1. Simulate asset backing escrow deposits
    vault.deposit_backing_asset("GOLD_TOKEN", 500_000);
    vault.deposit_backing_asset("USDC", 10_000_000);

    // 2. Simulate receiving the 30 ZED POL fee generated from your GSwap test swap
    let pol_fee_from_gswap = 30; // 0.03% from GSwap swap
    vault.receive_gswap_pol_fee(pol_fee_from_gswap);

    println!("Total POL Reserve Yield Accumulated: {} ZED", vault.total_pol_fee_accumulated);
    println!("Autonomous Vault Routing Active: {}", vault.is_autonomous_yield_enabled);
}
