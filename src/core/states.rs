//
// N.B. Auto-generated using actus-mp
//

use crate::enums;
use crate::core::time as core;

/// 
/// A data structure representing various states of a contract.  The states of a 
/// contract contain important information used when evaluating payoff. Furthermore, 
/// states themselves contain atomic analytical elements such as the nominal value 
/// or accrued interest for a lending instrument. On the other hand, states are 
/// updated throughout an instrument's lifetime through evaluation of state 
/// transition function in the various contract events.
/// 
pub struct StateSpace {
    // IPAC :: Accrued Interest :: The current value of accrued interest.
    pub accrued_interest: f64,

    // IPAC2 :: Accrued Interest 2 :: The current value of accrued interest of the second leg.
    pub accrued_interest2: f64,

    // PRF :: Contract Performance :: Indicates the current Contract Performance status. Different states of the contract range from performing to default..
    pub contract_performance: enums::ContractPerformance,

    // XA :: Exercise Amount :: The amount fixed for a the contingent event/obligation exercised at Exercise Date.
    pub exercise_amount: f64,

    // XD :: Exercise Date :: The timestamp at which a contingent event/obligation is exercised.
    pub exercise_date: core::Timestamp,

    // FEAC :: Fee Accrued :: The current value of accrued fees.
    pub fee_accrued: f64,

    // ICBA :: Interest Calculation Base Amount :: The basis at which interest is being accrued. Potentially different from NVL..
    pub interest_calculation_base_amount: f64,

    // SCIP :: Interest Scaling Multiplier :: The multiplier being applied to interest cash flows.
    pub interest_scaling_multiplier: f64,

    // MD :: Maturity Date :: The timestamp as per which the contract matures according to the initial terms or as per unscheduled events.
    pub maturity_date: core::Timestamp,

    // PRNXT :: Next Principal Redemption Payment :: The value at which principal is being repaid. This may be including or excluding of interest depending on the Contract Type.
    pub next_principal_redemption_payment: f64,

    // IPNR :: Nominal Interest Rate :: The applicable nominal rate.
    pub nominal_interest_rate: f64,

    // IPNR2 :: Nominal Interest Rate 2 :: The applicable nominal rate.
    pub nominal_interest_rate2: f64,

    // NPD :: Non Performing Date :: The date of the (uncovered) payment event responsible for the current value of the Contract Performance state variable..
    pub non_performing_date: core::Timestamp,

    // NT :: Notional Principal :: The outstanding nominal value.
    pub notional_principal: f64,

    // NT2 :: Notional Principal 2 :: The outstanding nominal value of the second leg.
    pub notional_principal2: f64,

    // SCNT :: Notional Scaling Multiplier :: The multiplier being applied to principal cash flows.
    pub notional_scaling_multiplier: f64,

    // SD :: Status Date :: The timestamp as per which the state is captured at any point in time.
    pub status_date: core::Timestamp,

    // TD :: Termination Date :: The timestamp of unscheduled termination of a contract.
    pub termination_date: core::Timestamp,
}
