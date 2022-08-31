//
// N.B. Auto-generated using actus-mp
//

use crate::types::core;
use crate::types::enums;

///
/// LAM :: Linear Amortizer.
///
/// Principal payment fully at IED. Principal repaid periodically in constant amounts till MD. Interest gets reduced accordingly. If variable rate, only interest payment is recalculated. Fixed and variable rates.
///
pub struct LinearAmortizerTermset {
    /// IPAC :: Accrued Interest.
    pub accrued_interest: f64,

    /// BDC :: Business Day Convention.
    pub business_day_convention: enums::BusinessDayConvention,

    /// CLDR :: Calendar.
    pub calendar: enums::Calendar,

    /// IPCED :: Capitalization End Date.
    pub capitalization_end_date: core::Timestamp,

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

    /// CLA :: Credit Line Amount.
    pub credit_line_amount: f64,

    /// CUR :: Currency.
    pub currency: String,

    /// FEANX :: Cycle Anchor Date Of Fee.
    pub cycle_anchor_date_of_fee: core::Timestamp,

    /// IPCBANX :: Cycle Anchor Date Of Interest Calculation Base.
    pub cycle_anchor_date_of_interest_calculation_base: core::Timestamp,

    /// IPANX :: Cycle Anchor Date Of Interest Payment.
    pub cycle_anchor_date_of_interest_payment: core::Timestamp,

    /// OPANX :: Cycle Anchor Date Of Optionality.
    pub cycle_anchor_date_of_optionality: core::Timestamp,

    /// PRANX :: Cycle Anchor Date Of Principal Redemption.
    pub cycle_anchor_date_of_principal_redemption: core::Timestamp,

    /// RRANX :: Cycle Anchor Date Of Rate Reset.
    pub cycle_anchor_date_of_rate_reset: core::Timestamp,

    /// SCANX :: Cycle Anchor Date Of Scaling Index.
    pub cycle_anchor_date_of_scaling_index: core::Timestamp,

    /// FECL :: Cycle Of Fee.
    pub cycle_of_fee: core::Cycle,

    /// IPCBCL :: Cycle Of Interest Calculation Base.
    pub cycle_of_interest_calculation_base: core::Cycle,

    /// IPCL :: Cycle Of Interest Payment.
    pub cycle_of_interest_payment: core::Cycle,

    /// OPCL :: Cycle Of Optionality.
    pub cycle_of_optionality: core::Cycle,

    /// PRCL :: Cycle Of Principal Redemption.
    pub cycle_of_principal_redemption: core::Cycle,

    /// RRCL :: Cycle Of Rate Reset.
    pub cycle_of_rate_reset: core::Cycle,

    /// SCCL :: Cycle Of Scaling Index.
    pub cycle_of_scaling_index: core::Cycle,

    /// IPPNT :: Cycle Point Of Interest Payment.
    pub cycle_point_of_interest_payment: enums::CyclePointOfInterestPayment,

    /// RRPNT :: Cycle Point Of Rate Reset.
    pub cycle_point_of_rate_reset: enums::CyclePointOfRateReset,

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

    /// RRFIX :: Fixing Period.
    pub fixing_period: core::Period,

    /// GRP :: Grace Period.
    pub grace_period: core::Period,

    /// IED :: Initial Exchange Date.
    pub initial_exchange_date: core::Timestamp,

    /// IPCB :: Interest Calculation Base.
    pub interest_calculation_base: enums::InterestCalculationBase,

    /// IPCBA :: Interest Calculation Base Amount.
    pub interest_calculation_base_amount: f64,

    /// SCIP :: Interest Scaling Multiplier.
    pub interest_scaling_multiplier: f64,

    /// RRLC :: Life Cap.
    pub life_cap: f64,

    /// RRLF :: Life Floor.
    pub life_floor: f64,

    /// MOC :: Market Object Code.
    pub market_object_code: String,

    /// RRMO :: Market Object Code Of Rate Reset.
    pub market_object_code_of_rate_reset: String,

    /// SCMO :: Market Object Code Of Scaling Index.
    pub market_object_code_of_scaling_index: String,

    /// MVO :: Market Value Observed.
    pub market_value_observed: f64,

    /// MD :: Maturity Date.
    pub maturity_date: core::Timestamp,

    /// PRNXT :: Next Principal Redemption Payment.
    pub next_principal_redemption_payment: f64,

    /// RRNXT :: Next Reset Rate.
    pub next_reset_rate: f64,

    /// IPNR :: Nominal Interest Rate.
    pub nominal_interest_rate: f64,

    /// NPD :: Non Performing Date.
    pub non_performing_date: core::Timestamp,

    /// NT :: Notional Principal.
    pub notional_principal: f64,

    /// SCNT :: Notional Scaling Multiplier.
    pub notional_scaling_multiplier: f64,

    /// OPXED :: Option Exercise End Date.
    pub option_exercise_end_date: core::Timestamp,

    /// PYRT :: Penalty Rate.
    pub penalty_rate: f64,

    /// PYTP :: Penalty Type.
    pub penalty_type: enums::PenaltyType,

    /// RRPC :: Period Cap.
    pub period_cap: f64,

    /// RRPF :: Period Floor.
    pub period_floor: f64,

    /// PDIED :: Premium Discount At IED.
    pub premium_discount_at_ied: f64,

    /// PPEF :: Prepayment Effect.
    pub prepayment_effect: enums::PrepaymentEffect,

    /// PPP :: Prepayment Period.
    pub prepayment_period: core::Period,

    /// PPRD :: Price At Purchase Date.
    pub price_at_purchase_date: f64,

    /// PTD :: Price At Termination Date.
    pub price_at_termination_date: f64,

    /// PRD :: Purchase Date.
    pub purchase_date: core::Timestamp,

    /// RRMLT :: Rate Multiplier.
    pub rate_multiplier: f64,

    /// RRSP :: Rate Spread.
    pub rate_spread: f64,

    /// SCEF :: Scaling Effect.
    pub scaling_effect: enums::ScalingEffect,

    /// SCCDD :: Scaling Index At Contract Deal Date.
    pub scaling_index_at_contract_deal_date: f64,

    /// SEN :: Seniority.
    pub seniority: enums::Seniority,

    /// CURS :: Settlement Currency.
    pub settlement_currency: String,

    /// SD :: Status Date.
    pub status_date: core::Timestamp,

    /// TD :: Termination Date.
    pub termination_date: core::Timestamp,
}
