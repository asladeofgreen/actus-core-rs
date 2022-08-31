//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// CSH :: Cash.
///
/// Cash or cash equivalent position
///
pub struct CashTermset {
    // CID :: Contract Identifier.
    pub contract_id: String,

    // CNTRL :: Contract Role.
    pub contract_role: enums::ContractRole,

    // CT :: Contract Type.
    pub contract_type: enums::ContractType,

    // CRID :: Creator Identifier.
    pub creator_id: String,

    // CUR :: Currency.
    pub currency: String,

    // NT :: Notional Principal.
    pub notional_principal: f64,

    // SD :: Status Date.
    pub status_date: core::Timestamp,
}
