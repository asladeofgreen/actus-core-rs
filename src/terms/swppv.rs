//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// SWPPV :: Plain Vanilla Swap.
///
/// Plain vanilla swaps where the underlyings are always two identical PAM´s however with one leg fixed and the other variable.
///
pub struct PlainVanillaSwapTermset {
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

    // CT :: Contract Type.
    pub contract_type: enums::ContractType,

    // CPID :: Counterparty Identifier.
    pub counterparty_id: String,

    // CRID :: Creator Identifier.
    pub creator_id: String,

    // CUR :: Currency.
    pub currency: String,

    // IPANX :: Cycle Anchor Date Of Interest Payment.
    pub cycle_anchor_date_of_interest_payment: core::Timestamp,

    // RRANX :: Cycle Anchor Date Of Rate Reset.
    pub cycle_anchor_date_of_rate_reset: core::Timestamp,

    // IPCL :: Cycle Of Interest Payment.
    pub cycle_of_interest_payment: core::Cycle,

    // RRCL :: Cycle Of Rate Reset.
    pub cycle_of_rate_reset: core::Cycle,

    // RRPNT :: Cycle Point Of Rate Reset.
    pub cycle_point_of_rate_reset: enums::CyclePointOfRateReset,

    // IPDC :: Day Count Convention.
    pub day_count_convention: enums::DayCountConvention,

    // DQP :: Delinquency Period.
    pub delinquency_period: core::Period,

    // DQR :: Delinquency Rate.
    pub delinquency_rate: f64,

    // DS :: Delivery Settlement.
    pub delivery_settlement: enums::DeliverySettlement,

    // EOMC :: End Of Month Convention.
    pub end_of_month_convention: enums::EndOfMonthConvention,

    // RRFIX :: Fixing Period.
    pub fixing_period: core::Period,

    // GRP :: Grace Period.
    pub grace_period: core::Period,

    // IED :: Initial Exchange Date.
    pub initial_exchange_date: core::Timestamp,

    // MOC :: Market Object Code.
    pub market_object_code: String,

    // RRMO :: Market Object Code Of Rate Reset.
    pub market_object_code_of_rate_reset: String,

    // MVO :: Market Value Observed.
    pub market_value_observed: f64,

    // MD :: Maturity Date.
    pub maturity_date: core::Timestamp,

    // RRNXT :: Next Reset Rate.
    pub next_reset_rate: f64,

    // IPNR :: Nominal Interest Rate.
    pub nominal_interest_rate: f64,

    // IPNR2 :: Nominal Interest Rate 2.
    pub nominal_interest_rate2: f64,

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

    // RRMLT :: Rate Multiplier.
    pub rate_multiplier: f64,

    // RRSP :: Rate Spread.
    pub rate_spread: f64,

    // SEN :: Seniority.
    pub seniority: enums::Seniority,

    // CURS :: Settlement Currency.
    pub settlement_currency: String,

    // SD :: Status Date.
    pub status_date: core::Timestamp,

    // TD :: Termination Date.
    pub termination_date: core::Timestamp,
}
