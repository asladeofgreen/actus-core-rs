//
// N.B. Auto-generated using actus-mp
//

use crate::types::core;
use crate::types::enums;

///
/// FXOUT :: Foreign Ex-change Outright.
///
/// Two parties agree to exchange two fixed cash flows in different currencies at a certain point in time in future.
///
pub struct ForeignExchangeOutrightTermset {
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

    /// CUR2 :: Currency 2.
    pub currency2: String,

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

    /// NT :: Notional Principal.
    pub notional_principal: f64,

    /// NT2 :: Notional Principal 2.
    pub notional_principal2: f64,

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
