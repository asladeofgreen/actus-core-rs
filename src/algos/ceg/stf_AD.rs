//
// N.B. Auto-generated using actus-mp
//
use crate::types::core::StateSpace;
use crate::types::core::Timestamp;
use crate::types::terms::GuaranteeTermset as ContractTermset;

/// 
/// Executes a CEG contract AD state transition function.
/// 
/// # Arguments
/// 
/// `time` - The schedule time of this particular event.
/// `states` - The current state of contract states.
/// `term_set` - The set of contract terms.
/// `risk_factor_model` - An external market model.
/// `day_counter` - The day count convention used to calculate day count fractions.
/// `time_adjuster` - The business day convention used to shift the schedule time.
/// 
/// # Returns
/// 
/// An array of post-event states of numerical contract states.
/// 
pub fn execute(
    _time: Timestamp,
    _states: StateSpace,
    _term_set: ContractTermset,
    _risk_factor_model: String,
    _day_counter: String,
    _time_adjuster: String
    ) {
    unimplemented!()
}