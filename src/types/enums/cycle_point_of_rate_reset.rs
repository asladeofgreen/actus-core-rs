//
// N.B. Auto-generated using actus-mp
//

/// RRPNT :: Cycle Point Of Rate Reset.
///
/// Normally rates get reset at the beginning of any resetting cycles. There are contracts where the rate is not set at the beginning but at the end of the cycle and then applied to the previous cycle (post-fixing); in other words the rate applies before it is fixed. Hence, the new rate is not known during the entire cycle where it applies. Therefore, the rate will be applied backwards at the end of the cycle. This happens through a correction of interest accrued.
///
pub enum CyclePointOfRateReset {
    /// Beginning: The new rate is applied at the beginning of the reset period.
    B = 0,

    /// End: The new rate is applied at the end of the reset period.
    E = 1,
}
