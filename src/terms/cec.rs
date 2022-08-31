//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// CEC :: Collateral.
///
/// Collateral creates a relationship between a collateral an obligee and a debtor, covering the exposure from the debtor with the collateral.
///
pub struct CollateralTermset {
    // BDC :: Business Day Convention.
    pub business_day_convention: enums::BusinessDayConvention,

    // CLDR :: Calendar.
    pub calendar: enums::Calendar,

    // CDD :: Contract Deal Date.
    pub contract_deal_date: f64,

    // CID :: Contract Identifier.
    pub contract_id: String,

    // CNTRL :: Contract Role.
    pub contract_role: enums::ContractRole,

    // CTS :: Contract Structure.
    pub contract_structure: Vec<core::ContractReference>,

    // CT :: Contract Type.
    pub contract_type: enums::ContractType,

    // CPID :: Counterparty Identifier.
    pub counterparty_id: String,

    // CECV :: Coverage Of Credit Enhancement.
    pub coverage_of_credit_enhancement: f64,

    // CRID :: Creator Identifier.
    pub creator_id: String,

    // CETC :: Credit Event Type Covered.
    pub credit_event_type_covered: Vec<enums::CreditEventTypeCovered>,

    // EOMC :: End Of Month Convention.
    pub end_of_month_convention: enums::EndOfMonthConvention,

    // XA :: Exercise Amount.
    pub exercise_amount: f64,

    // XD :: Exercise Date.
    pub exercise_date: f64,

    // CEGE :: Guaranteed Exposure.
    pub guaranteed_exposure: enums::GuaranteedExposure,

    // STP :: Settlement Period.
    pub settlement_period: core::Period,

    // SD :: Status Date.
    pub status_date: f64,
}
