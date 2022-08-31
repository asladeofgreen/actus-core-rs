//
// N.B. Auto-generated using actus-mp
//

use crate::types::core;
use crate::types::enums;

///
/// OPTNS :: Option.
///
/// Calculates straight option pay-off for any basic CT as underlying (PAM, ANN etc.) but also SWAPS, FXOUT, STK and COM. Single, periodic and continuous strike is supported.
///
pub struct OptionTermset {
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

    /// CTS :: Contract Structure.
    pub contract_structure: Vec<core::ContractReference>,

    /// CT :: Contract Type.
    pub contract_type: enums::ContractType,

    /// CPID :: Counterparty Identifier.
    pub counterparty_id: String,

    /// CRID :: Creator Identifier.
    pub creator_id: String,

    /// CUR :: Currency.
    pub currency: String,

    /// OPANX :: Cycle Anchor Date Of Optionality.
    pub cycle_anchor_date_of_optionality: core::Timestamp,

    /// OPCL :: Cycle Of Optionality.
    pub cycle_of_optionality: core::Cycle,

    /// DQP :: Delinquency Period.
    pub delinquency_period: core::Period,

    /// DQR :: Delinquency Rate.
    pub delinquency_rate: f64,

    /// DS :: Delivery Settlement.
    pub delivery_settlement: enums::DeliverySettlement,

    /// EOMC :: End Of Month Convention.
    pub end_of_month_convention: enums::EndOfMonthConvention,

    /// XA :: Exercise Amount.
    pub exercise_amount: f64,

    /// XD :: Exercise Date.
    pub exercise_date: core::Timestamp,

    /// GRP :: Grace Period.
    pub grace_period: core::Period,

    /// MOC :: Market Object Code.
    pub market_object_code: String,

    /// MVO :: Market Value Observed.
    pub market_value_observed: f64,

    /// MD :: Maturity Date.
    pub maturity_date: core::Timestamp,

    /// NPD :: Non Performing Date.
    pub non_performing_date: core::Timestamp,

    /// OPXED :: Option Exercise End Date.
    pub option_exercise_end_date: core::Timestamp,

    /// OPXT :: Option Exercise Type.
    pub option_exercise_type: enums::OptionExerciseType,

    /// OPS1 :: Option Strike 1.
    pub option_strike1: f64,

    /// OPS2 :: Option Strike 2.
    pub option_strike2: f64,

    /// OPTP :: Option Type.
    pub option_type: enums::OptionType,

    /// PPRD :: Price At Purchase Date.
    pub price_at_purchase_date: f64,

    /// PTD :: Price At Termination Date.
    pub price_at_termination_date: f64,

    /// PRD :: Purchase Date.
    pub purchase_date: core::Timestamp,

    /// SEN :: Seniority.
    pub seniority: enums::Seniority,

    /// CURS :: Settlement Currency.
    pub settlement_currency: String,

    /// STP :: Settlement Period.
    pub settlement_period: core::Period,

    /// SD :: Status Date.
    pub status_date: core::Timestamp,

    /// TD :: Termination Date.
    pub termination_date: core::Timestamp,
}
