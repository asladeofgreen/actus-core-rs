//
// N.B. Auto-generated using actus-mp
//

use crate::types::core;
use crate::types::enums;

///
/// UMP :: Undefined Maturity Profile.
///
/// Principal paid in and out at any point in time without prefixed schedule. Interest calculated on outstanding and capitalized periodically. Needs link to a behavioral function describing expected flows.
///
pub struct UndefinedMaturityProfileTermset {
    /// IPAC :: Accrued Interest.
    pub accrued_interest: f64,

    /// BDC :: Business Day Convention.
    pub business_day_convention: enums::BusinessDayConvention,

    /// CLDR :: Calendar.
    pub calendar: enums::Calendar,

    /// CDD :: Contract Deal Date.
    pub contract_deal_date: core::Timestamp,

    /// CID :: Contract Identifier.
    pub contract_id: String,

    /// PRF :: Contract Performance.
    pub contract_performance: enums::ContractPerformance,

    /// CNTRL :: Contract Role.
    pub contract_role: enums::ContractRole,

    /// CT :: Contract Type.
    pub contract_type: enums::ContractType,

    /// CPID :: Counterparty Identifier.
    pub counterparty_id: String,

    /// CRID :: Creator Identifier.
    pub creator_id: String,

    /// CUR :: Currency.
    pub currency: String,

    /// FEANX :: Cycle Anchor Date Of Fee.
    pub cycle_anchor_date_of_fee: core::Timestamp,

    /// IPANX :: Cycle Anchor Date Of Interest Payment.
    pub cycle_anchor_date_of_interest_payment: core::Timestamp,

    /// RRANX :: Cycle Anchor Date Of Rate Reset.
    pub cycle_anchor_date_of_rate_reset: core::Timestamp,

    /// FECL :: Cycle Of Fee.
    pub cycle_of_fee: core::Cycle,

    /// IPCL :: Cycle Of Interest Payment.
    pub cycle_of_interest_payment: core::Cycle,

    /// RRCL :: Cycle Of Rate Reset.
    pub cycle_of_rate_reset: core::Cycle,

    /// IPDC :: Day Count Convention.
    pub day_count_convention: enums::DayCountConvention,

    /// DQP :: Delinquency Period.
    pub delinquency_period: core::Period,

    /// DQR :: Delinquency Rate.
    pub delinquency_rate: f64,

    /// EOMC :: End Of Month Convention.
    pub end_of_month_convention: enums::EndOfMonthConvention,

    /// FEAC :: Fee Accrued.
    pub fee_accrued: f64,

    /// FEB :: Fee Basis.
    pub fee_basis: enums::FeeBasis,

    /// FER :: Fee Rate.
    pub fee_rate: f64,

    /// GRP :: Grace Period.
    pub grace_period: core::Period,

    /// IED :: Initial Exchange Date.
    pub initial_exchange_date: core::Timestamp,

    /// RRMO :: Market Object Code Of Rate Reset.
    pub market_object_code_of_rate_reset: String,

    /// MPFD :: Maximum Penalty Free Disbursement.
    pub maximum_penalty_free_disbursement: f64,

    /// IPNR :: Nominal Interest Rate.
    pub nominal_interest_rate: f64,

    /// NPD :: Non Performing Date.
    pub non_performing_date: core::Timestamp,

    /// NT :: Notional Principal.
    pub notional_principal: f64,

    /// PPP :: Prepayment Period.
    pub prepayment_period: core::Period,

    /// PTD :: Price At Termination Date.
    pub price_at_termination_date: f64,

    /// RRSP :: Rate Spread.
    pub rate_spread: f64,

    /// SEN :: Seniority.
    pub seniority: enums::Seniority,

    /// CURS :: Settlement Currency.
    pub settlement_currency: String,

    /// SD :: Status Date.
    pub status_date: core::Timestamp,

    /// TD :: Termination Date.
    pub termination_date: core::Timestamp,

    /// XDN :: X Day Notice.
    pub x_day_notice: core::Period,
}
