//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// CAPFL :: Cap Floors.
///
/// Interest rate option expressed in a maximum or minimum interest rate.
///
pub struct CapFloorTermset {
    // CDD :: Contract Deal Date.
    pub contract_deal_date: core::Timestamp,

    // CID :: Contract Identifier.
    pub contract_id: String,

    // PRF :: Contract Performance.
    pub contract_performance: enums::ContractPerformance,

    // CNTRL :: Contract Role.
    pub contract_role: enums::ContractRole,

    // CTS :: Contract Structure.
    pub contract_structure: Vec<core::ContractReference>,

    // CT :: Contract Type.
    pub contract_type: enums::ContractType,

    // CPID :: Counterparty Identifier.
    pub counterparty_id: String,

    // CRID :: Creator Identifier.
    pub creator_id: String,

    // CUR :: Currency.
    pub currency: String,

    // DQP :: Delinquency Period.
    pub delinquency_period: core::Period,

    // DQR :: Delinquency Rate.
    pub delinquency_rate: f64,

    // GRP :: Grace Period.
    pub grace_period: core::Period,

    // RRLC :: Life Cap.
    pub life_cap: f64,

    // RRLF :: Life Floor.
    pub life_floor: f64,

    // MOC :: Market Object Code.
    pub market_object_code: String,

    // MVO :: Market Value Observed.
    pub market_value_observed: f64,

    // NPD :: Non Performing Date.
    pub non_performing_date: core::Timestamp,

    // PPRD :: Price At Purchase Date.
    pub price_at_purchase_date: f64,

    // PTD :: Price At Termination Date.
    pub price_at_termination_date: f64,

    // PRD :: Purchase Date.
    pub purchase_date: core::Timestamp,

    // SEN :: Seniority.
    pub seniority: enums::Seniority,

    // CURS :: Settlement Currency.
    pub settlement_currency: String,

    // SD :: Status Date.
    pub status_date: core::Timestamp,

    // TD :: Termination Date.
    pub termination_date: core::Timestamp,
}
