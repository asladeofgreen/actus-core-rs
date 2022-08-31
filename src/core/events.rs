use crate::enums;
use crate::core::states;
use crate::core::time;

/// 
/// Represent a single event generated during the lifetime of a contract.  Such events
/// represent the atomic analytical elements comprising all events that possibly
/// occur during the lifetime of a contract.  That is, contract events mark specific
/// times at which either a state transition, a payoff, or both is generated from a contract.
/// 
/// Contract events are a generic representation of a specific state transition function and
/// pay off function with an event time according to which all events in a contract are ordered
/// (in an series) and processed sequentially.  Thereby, processing an event in fact means that 
/// it's state transition and payoff functions are evaluated.
/// 
pub struct Event {
    // The Contract ID of the contract which created the event.
    pub contract_id: String,

    // The currency in which the event payoff is scheduled.
    pub currency: String,

    // Time offset within context of an epoch.
    pub epoch_offset: f64,

    // The timestamp of the event.
    pub event_time: time::Timestamp,

    // The type of the event. Different types have their own business logic in terms of payoff and state transition functions.
    pub event_type: enums::EventType,

    // The event state-transition function.
    pub f_state_transition: f64,

    // The event pay-off function.
    pub f_payoff: f64,

    // The event payoff (if any). Is zero if no payoff needs be settled for the event.
    pub payoff: f64,

    // The scheduled timestamp for updating event payoff and post-event state.
    pub schedule_time: time::Timestamp,

    // The post-event state. Results from applying the event’s state transition function to the pre-event state. 
    pub state: states::StateSpace
}
