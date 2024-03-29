//
// N.B. Auto-generated using actus-mp
//

use crate::types::core::Event;
use crate::types::core::Timestamp;
use crate::types::terms::ExoticLinearAmortizerTermset as ContractTermset;

/// 
/// Executes a step within the calculation engine.
/// 
/// # Arguments
/// 
/// * `events` - A list of contract events that should be applied in time sequence.
/// * `term_set` - The contract term set.
/// * `observer`- The observer for external events & data.
/// 
/// # Returns
/// 
/// The evaluated events and post-event contract states.
/// 
pub fn execute_step(_events: Vec<Event>, _term_set: ContractTermset, _observer: String) -> Vec<Event> {
    unimplemented!();
}

/// 
/// Evaluates next contract event sequence within a certain time period.
/// 
/// The set of contract attributes are mapped to the stream of next contract events
/// within a specified time period according to the legal logic of the respective
/// Contract Type and contingent to the risk factor dynamics provided with the
/// risk factor model.  The contract's status date is used as the reference time
/// as from which the code period is evaluated.
/// 
/// Note, the stream of the next non-contingent contract events matches the portion
/// of the stream of the next contingent events up to the first contingent event.
/// Furthermore, for a contract with purely non-contingent events
/// (e.g. a PrincipalAtMaturity without a RateReset, Scaling, CreditDefault, etc.)
/// contingent and non-contingent event streams are the same.
/// 
/// # Arguments
/// 
/// * `to_date` - The time up to which the events are to be evaluated.
/// * `term_set` - The contract term set.
/// 
/// # Returns
/// 
/// An event sequence -> to_date.
/// 
pub fn get_schedule(_to_date: Timestamp, _term_set: ContractTermset) -> Vec<Event> {
    unimplemented!();
}
