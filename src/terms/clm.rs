//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// CLM :: Call Money.
///
/// Loans that are rolled over as long as they are not called. Once called it has to be paid back after the stipulated notice period.
///
pub struct CallMoneyTermset {
    // IPAC :: Accrued Interest.
    pub accrued_interest: f64,

    // BDC :: Business Day Convention.
    pub business_day_convention: enums::BusinessDayConvention,

    // CLDR :: Calendar.
    pub calendar: enums::Calendar,

    // CDD :: Contract Deal Date.
    pub contract_deal_date: f64,

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

    // FEANX :: Cycle Anchor Date Of Fee.
    pub cycle_anchor_date_of_fee: f64,

    // IPANX :: Cycle Anchor Date Of Interest Payment.
    pub cycle_anchor_date_of_interest_payment: f64,

    // RRANX :: Cycle Anchor Date Of Rate Reset.
    pub cycle_anchor_date_of_rate_reset: f64,

    // FECL :: Cycle Of Fee.
    pub cycle_of_fee: core::Cycle,

    // IPCL :: Cycle Of Interest Payment.
    pub cycle_of_interest_payment: core::Cycle,

    // RRCL :: Cycle Of Rate Reset.
    pub cycle_of_rate_reset: core::Cycle,

    // IPDC :: Day Count Convention.
    pub day_count_convention: enums::DayCountConvention,

    // DQP :: Delinquency Period.
    pub delinquency_period: core::Period,

    // DQR :: Delinquency Rate.
    pub delinquency_rate: f64,

    // EOMC :: End Of Month Convention.
    pub end_of_month_convention: enums::EndOfMonthConvention,

    // FEAC :: Fee Accrued.
    pub fee_accrued: f64,

    // FEB :: Fee Basis.
    pub fee_basis: enums::FeeBasis,

    // FER :: Fee Rate.
    pub fee_rate: f64,

    // RRFIX :: Fixing Period.
    pub fixing_period: core::Period,

    // GRP :: Grace Period.
    pub grace_period: core::Period,

    // IED :: Initial Exchange Date.
    pub initial_exchange_date: f64,

    // RRMO :: Market Object Code Of Rate Reset.
    pub market_object_code_of_rate_reset: String,

    // MD :: Maturity Date.
    pub maturity_date: f64,

    // RRNXT :: Next Reset Rate.
    pub next_reset_rate: f64,

    // IPNR :: Nominal Interest Rate.
    pub nominal_interest_rate: f64,

    // NPD :: Non Performing Date.
    pub non_performing_date: f64,

    // NT :: Notional Principal.
    pub notional_principal: f64,

    // PPP :: Prepayment Period.
    pub prepayment_period: core::Period,

    // RRMLT :: Rate Multiplier.
    pub rate_multiplier: f64,

    // RRSP :: Rate Spread.
    pub rate_spread: f64,

    // SEN :: Seniority.
    pub seniority: enums::Seniority,

    // CURS :: Settlement Currency.
    pub settlement_currency: String,

    // SD :: Status Date.
    pub status_date: f64,

    // XDN :: X Day Notice.
    pub x_day_notice: core::Period,
}
