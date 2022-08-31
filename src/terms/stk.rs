//
// N.B. Auto-generated using actus-mp
//

use crate::core;
use crate::enums;

/// STK :: Stock.
///
/// Any instrument which is bought at a certain amount (market price normally) and then follows an index.
///
pub struct StockTermset {
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

    // DVANX :: Cycle Anchor Date Of Dividend.
    pub cycle_anchor_date_of_dividend: f64,

    // DVCL :: Cycle Of Dividend.
    pub cycle_of_dividend: core::Cycle,

    // EOMC :: End Of Month Convention.
    pub end_of_month_convention: enums::EndOfMonthConvention,

    // DVEX :: Ex Dividend Date.
    pub ex_dividend_date: f64,

    // MOC :: Market Object Code.
    pub market_object_code: String,

    // MVO :: Market Value Observed.
    pub market_value_observed: f64,

    // DVNP :: Next Dividend Payment Amount.
    pub next_dividend_payment_amount: f64,

    // NPD :: Non Performing Date.
    pub non_performing_date: f64,

    // NT :: Notional Principal.
    pub notional_principal: f64,

    // PPRD :: Price At Purchase Date.
    pub price_at_purchase_date: f64,

    // PTD :: Price At Termination Date.
    pub price_at_termination_date: f64,

    // PRD :: Purchase Date.
    pub purchase_date: f64,

    // QT :: Quantity.
    pub quantity: f64,

    // SEN :: Seniority.
    pub seniority: enums::Seniority,

    // CURS :: Settlement Currency.
    pub settlement_currency: String,

    // SD :: Status Date.
    pub status_date: f64,

    // TD :: Termination Date.
    pub termination_date: f64,
}
