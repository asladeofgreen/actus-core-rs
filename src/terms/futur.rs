//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// FUTUR :: Future.
///
/// Keeps track of value changes for any basic CT as underlying (PAM, ANN etc. but also FXOUT, STK, COM). Handles margining calls.
///
pub struct FutureTermset {
    // BDC :: Business Day Convention.
    pub business_day_convention: enums::BusinessDayConvention,

    // CLDR :: Calendar.
    pub calendar: enums::Calendar,

    // MRCLH :: Clearing House.
    pub clearing_house: enums::ClearingHouse,

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

    // MRANX :: Cycle Anchor Date Of Margining.
    pub cycle_anchor_date_of_margining: core::Timestamp,

    // MRCL :: Cycle Of Margining.
    pub cycle_of_margining: core::Cycle,

    // DQP :: Delinquency Period.
    pub delinquency_period: core::Period,

    // DQR :: Delinquency Rate.
    pub delinquency_rate: f64,

    // DS :: Delivery Settlement.
    pub delivery_settlement: enums::DeliverySettlement,

    // EOMC :: End Of Month Convention.
    pub end_of_month_convention: enums::EndOfMonthConvention,

    // XA :: Exercise Amount.
    pub exercise_amount: f64,

    // XD :: Exercise Date.
    pub exercise_date: core::Timestamp,

    // PFUT :: Futures Price.
    pub futures_price: f64,

    // GRP :: Grace Period.
    pub grace_period: core::Period,

    // MRIM :: Initial Margin.
    pub initial_margin: f64,

    // MRMML :: Maintenance Margin Lower Bound.
    pub maintenance_margin_lower_bound: f64,

    // MRMMU :: Maintenance Margin Upper Bound.
    pub maintenance_margin_upper_bound: f64,

    // MOC :: Market Object Code.
    pub market_object_code: String,

    // MVO :: Market Value Observed.
    pub market_value_observed: f64,

    // MD :: Maturity Date.
    pub maturity_date: core::Timestamp,

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

    // STP :: Settlement Period.
    pub settlement_period: core::Period,

    // SD :: Status Date.
    pub status_date: core::Timestamp,

    // TD :: Termination Date.
    pub termination_date: core::Timestamp,

    // MRVM :: Variation Margin.
    pub variation_margin: f64,
}
