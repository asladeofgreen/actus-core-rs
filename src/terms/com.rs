//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// COM :: Commodity.
///
/// This is not a financial contract in its propper sense. However it traks movements of commodities such as oil, gas or even houses. Such commodities can serve as underlyings of commodity futures, guarantees or simply asset positions.
///
pub struct CommodityTermset {
    // CDD :: Contract Deal Date.
    pub contract_deal_date: core::Timestamp,

    // CID :: Contract Identifier.
    pub contract_id: String,

    // CNTRL :: Contract Role.
    pub contract_role: enums::ContractRole,

    // CT :: Contract Type.
    pub contract_type: enums::ContractType,

    // CPID :: Counterparty Identifier.
    pub counterparty_id: String,

    // CRID :: Creator Identifier.
    pub creator_id: String,

    // CUR :: Currency.
    pub currency: String,

    // MOC :: Market Object Code.
    pub market_object_code: String,

    // MVO :: Market Value Observed.
    pub market_value_observed: f64,

    // PPRD :: Price At Purchase Date.
    pub price_at_purchase_date: f64,

    // PTD :: Price At Termination Date.
    pub price_at_termination_date: f64,

    // PRD :: Purchase Date.
    pub purchase_date: core::Timestamp,

    // QT :: Quantity.
    pub quantity: f64,

    // SD :: Status Date.
    pub status_date: core::Timestamp,

    // TD :: Termination Date.
    pub termination_date: core::Timestamp,

    // UT :: Unit.
    pub unit: enums::Unit,
}
