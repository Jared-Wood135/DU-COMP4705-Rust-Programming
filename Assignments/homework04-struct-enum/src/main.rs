//! FILE OVERVIEW:
//! - COMP3705 Daily Homework 04
//! - Selecting Golden Dog Game Company:
//!     - You have been asked to design an in-game monetary system for their latest role-playing-game title
//!     - The system needs to represent coins of different denominations, material types, sizes, and so on (As designer you choose what these possibilities are).
//!
//! =================================================
//!
//! MISC COMMENTS:
//! - 
//!
//! =================================================
//!
//! FILE CONTENTS:
//! - File Overview, Imports, Global Variables
//! - Enumerations
//!     - CoinType
//!         - value
//!         - diam_size_mm
//!         - thickness_mm
//!         - weight_g
//! - Structures
//!     - Currency
//!         - new
//!         - total_copper_val
//! - Test Functions
//!     - test_coin_values
//!     - test_coin_specs
//!     - test_coin_stacking
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// NA

// ----- Global Variables -------------------------------------------------------------------------
// NA

// ================================================================================================
// END File Overview, Imports, Global Variables
// START Enumerations
// ================================================================================================

/// About
/// -----
/// - Defines 4 types of allowed coin types and maps to respective information
/// - In order of least to most value:
///     - Copper
///     - Silver
///     - Gold
///     - Platinum
#[derive(Debug, PartialEq)]
enum CoinType {
    Copper,
    Silver,
    Gold,
    Platinum
}

impl CoinType {
    /// About
    /// -----
    /// - Returns raw value of CoinType as a u32
    /// 
    /// Parameters
    /// ----------
    /// - &self
    ///     - Simply calls itself to return a monetary value
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - u32
    ///     - Integer monetary value of &self
    fn value(&self) -> u32 {
        match self {
            CoinType::Copper => 1,
            CoinType::Silver => 10,
            CoinType::Gold => 100,
            CoinType::Platinum => 1000
        }
    }

    /// About
    /// -----
    /// - Returns diameter size of CoinType in mm as a u32
    /// 
    /// Parameters
    /// ----------
    /// - &self
    ///     - Simply calls itself to return diameter value
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - u32
    ///     - Integer mm diameter value of &self
    fn diam_size_mm(&self) -> u32 {
        match self {
            CoinType::Copper => 19,
            CoinType::Silver => 21,
            CoinType::Gold => 23,
            CoinType::Platinum => 25
        }
    }

    /// About
    /// -----
    /// - Returns thickness of CoinType in mm as a u32
    /// 
    /// Parameters
    /// ----------
    /// - &self
    ///     - Simply calls itself to return thickness value
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - u32
    ///     - Integer mm thickness value of &self
    fn thickness_mm(&self) -> u32 {
        match self {
            CoinType::Copper => 1,
            CoinType::Silver => 1,
            CoinType::Gold => 2,
            CoinType::Platinum => 3
        }
    }
    /// About
    /// -----
    /// - Returns weight of CoinType in g as a u32
    /// 
    /// Parameters
    /// ----------
    /// - &self
    ///     - Simply calls itself to return weight value
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - u32
    ///     - Integer g weight value of &self
    fn weight_g(&self) -> u32 {
        match self {
            CoinType::Copper => 2,
            CoinType::Silver => 4,
            CoinType::Gold => 6,
            CoinType::Platinum => 8
        }
    }
}

// ================================================================================================
// END Enumerations
// START Structures
// ================================================================================================

/// About
/// -----
/// - Defines a CoinType and how much of it exists
/// 
/// Parameters
/// ----------
/// - coin_type (CoinType)
///     - Type of coin (Copper, Silver, Gold, Platinum)
/// - amount (u32)
///     - How many of coin_type there are
#[derive(Debug)]
struct Currency {
    coin_type: CoinType,
    amount: u32
}

impl Currency {
    /// About
    /// -----
    /// - Create a new CoinType instance and an amount
    /// 
    /// Parameters
    /// ----------
    /// - coin_type (CoinType)
    ///     - Type of coin (Copper, Silver, Gold, Platinum)
    /// - amount (u32)
    ///     - How many of coin_type there are
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - Currency
    ///     - An instance of Currency
    fn new(coin_type: CoinType, amount: u32) -> Self {
        Self{coin_type, amount}
    }

    /// About
    /// -----
    /// - Returns the total copper value
    /// 
    /// Parameters
    /// ----------
    /// - &self
    ///     - Simply calls itself to determine total copper value
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - u32
    ///     - Total copper value of &self
    fn total_copper_val(&self) -> u32 {
        self.amount * self.coin_type.value()
    }
}

// ================================================================================================
// END Structures
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// About
    /// -----
    /// - Validate coin values
    #[test]
    fn test_coin_values() {
        assert_eq!(CoinType::Copper.value(), 1);
        assert_eq!(CoinType::Silver.value(), 10);
        assert_eq!(CoinType::Gold.value(), 100);
        assert_eq!(CoinType::Platinum.value(), 1000);
    }

    /// About
    /// -----
    /// - Validate coin specs
    #[test]
    fn test_coin_specs() {
        // Copper
        assert_eq!(CoinType::Copper.diam_size_mm(), 19);
        assert_eq!(CoinType::Copper.thickness_mm(), 1);
        assert_eq!(CoinType::Copper.weight_g(), 2);

        // Silver
        assert_eq!(CoinType::Silver.diam_size_mm(), 21);
        assert_eq!(CoinType::Silver.thickness_mm(), 1);
        assert_eq!(CoinType::Silver.weight_g(), 4);

        // Gold
        assert_eq!(CoinType::Gold.diam_size_mm(), 23);
        assert_eq!(CoinType::Gold.thickness_mm(), 2);
        assert_eq!(CoinType::Gold.weight_g(), 6);

        // Platinum
        assert_eq!(CoinType::Platinum.diam_size_mm(), 25);
        assert_eq!(CoinType::Platinum.thickness_mm(), 3);
        assert_eq!(CoinType::Platinum.weight_g(), 8);
    }

    /// About
    /// -----
    /// - Validate coin stacking and value
    #[test]
    fn test_coin_stacking() {
        let copper_stack = Currency::new(CoinType::Copper, 10);
        let silver_stack = Currency::new(CoinType::Silver, 10);
        let gold_stack = Currency::new(CoinType::Gold, 10);
        let plat_stack = Currency::new(CoinType::Platinum, 10);
        
        // Copper
        assert_eq!(copper_stack.coin_type, CoinType::Copper);
        assert_eq!(copper_stack.amount, 10);
        assert_eq!(copper_stack.total_copper_val(), 10);

        // Silver
        assert_eq!(silver_stack.coin_type, CoinType::Silver);
        assert_eq!(silver_stack.amount, 10);
        assert_eq!(silver_stack.total_copper_val(), 100);

        // Gold
        assert_eq!(gold_stack.coin_type, CoinType::Gold);
        assert_eq!(gold_stack.amount, 10);
        assert_eq!(gold_stack.total_copper_val(), 1000);

        // Platinum
        assert_eq!(plat_stack.coin_type, CoinType::Platinum);
        assert_eq!(plat_stack.amount, 10);
        assert_eq!(plat_stack.total_copper_val(), 10000);
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply prints 4 coin values and their specs for rapid validation of assignment
fn main() {
    println!("==================== Coin Values ====================");
    let copper_pouch = Currency::new(CoinType::Copper, 1);
    let silver_pouch = Currency::new(CoinType::Silver, 1);
    let gold_pouch = Currency::new(CoinType::Gold, 1);
    let plat_pouch = Currency::new(CoinType::Platinum, 1);

    // Print off coin values
    println!(
        "{:?}:   {} coins  = {} base copper value",
        copper_pouch.coin_type,
        copper_pouch.amount,
        copper_pouch.total_copper_val()
    );
    println!(
        "{:?}:   {} coins  = {} base copper value",
        silver_pouch.coin_type,
        silver_pouch.amount,
        silver_pouch.total_copper_val()
    );
    println!(
        "{:?}:     {} coins  = {} base copper value",
        gold_pouch.coin_type,
        gold_pouch.amount,
        gold_pouch.total_copper_val()
    );
    println!(
        "{:?}: {} coins  = {} base copper value",
        plat_pouch.coin_type,
        plat_pouch.amount,
        plat_pouch.total_copper_val()
    );

    // Print off total wallet value
    let total_wallet_val =  copper_pouch.total_copper_val()
                            + silver_pouch.total_copper_val()
                            + gold_pouch.total_copper_val()
                            + plat_pouch.total_copper_val();

    println!("\nTotal wallet copper value: {}", total_wallet_val);

    println!("\n==================== Coin Specs ====================");
    let copper = CoinType::Copper;
    let silver = CoinType::Silver;
    let gold = CoinType::Gold;
    let platinum = CoinType::Platinum;

    // Print off coin specs
    println!("Coin: {:?}", copper);
    println!("  Diameter:  {} mm", copper.diam_size_mm());
    println!("  Thickness: {} mm", copper.thickness_mm());
    println!("  Weight:    {} g", copper.weight_g());

    println!("Coin: {:?}", silver);
    println!("  Diameter:  {} mm", silver.diam_size_mm());
    println!("  Thickness: {} mm", silver.thickness_mm());
    println!("  Weight:    {} g", silver.weight_g());

    println!("Coin: {:?}", gold);
    println!("  Diameter:  {} mm", gold.diam_size_mm());
    println!("  Thickness: {} mm", gold.thickness_mm());
    println!("  Weight:    {} g", gold.weight_g());

    println!("Coin: {:?}", platinum);
    println!("  Diameter:  {} mm", platinum.diam_size_mm());
    println!("  Thickness: {} mm", platinum.thickness_mm());
    println!("  Weight:    {} g", platinum.weight_g());
}

// ================================================================================================
// END Main Function
// ================================================================================================