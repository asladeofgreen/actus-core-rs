//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// CEG :: Guarantee.
///
/// Guarantee creates a relationship between a guarantor, an obligee and a debtor, moving the exposure from the debtor to the guarantor.
///
pub struct GuaranteeTermset {
    // BDC :: Business Day Convention.
    pub business_day_convention: enums::BusinessDayConvention,

    // CLDR :: Calendar.
    pub calendar: enums::Calendar,

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

    // CECV :: Coverage Of Credit Enhancement.
    pub coverage_of_credit_enhancement: f64,

    // CRID :: Creator Identifier.
    pub creator_id: String,

    // CETC :: Credit Event Type Covered.
    pub credit_event_type_covered: Vec<enums::CreditEventTypeCovered>,

    // CUR :: Currency.
    pub currency: String,

    // FEANX :: Cycle Anchor Date Of Fee.
    pub cycle_anchor_date_of_fee: core::Timestamp,

    // FECL :: Cycle Of Fee.
    pub cycle_of_fee: core::Cycle,

    // DQP :: Delinquency Period.
    pub delinquency_period: core::Period,

    // DQR :: Delinquency Rate.
    pub delinquency_rate: f64,

    // EOMC :: End Of Month Convention.
    pub end_of_month_convention: enums::EndOfMonthConvention,

    // XA :: Exercise Amount.
    pub exercise_amount: f64,

    // XD :: Exercise Date.
    pub exercise_date: core::Timestamp,

    // FEAC :: Fee Accrued.
    pub fee_accrued: f64,

    // FEB :: Fee Basis.
    pub fee_basis: enums::FeeBasis,

    // FER :: Fee Rate.
    pub fee_rate: f64,

    // GRP :: Grace Period.
    pub grace_period: core::Period,

    // CEGE :: Guaranteed Exposure.
    pub guaranteed_exposure: enums::GuaranteedExposure,

    // MD :: Maturity Date.
    pub maturity_date: core::Timestamp,

    // NPD :: Non Performing Date.
    pub non_performing_date: core::Timestamp,

    // NT :: Notional Principal.
    pub notional_principal: f64,

    // PPRD :: Price At Purchase Date.
    pub price_at_purchase_date: f64,

    // PTD :: Price At Termination Date.
    pub price_at_termination_date: f64,

    // PRD :: Purchase Date.
    pub purchase_date: core::Timestamp,

    // CURS :: Settlement Currency.
    pub settlement_currency: String,

    // STP :: Settlement Period.
    pub settlement_period: core::Period,

    // SD :: Status Date.
    pub status_date: core::Timestamp,

    // TD :: Termination Date.
    pub termination_date: core::Timestamp,
}
