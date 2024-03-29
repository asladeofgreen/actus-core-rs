//
// N.B. Auto-generated using actus-mp
//

/// FEB :: Fee Basis.
///
/// Basis, on which Fee is calculated. For FEB=’A’, FER is interpreted as an absolute amount to be paid at every FP event and for FEB=’N’, FER represents a rate at which FP amounts accrue on the basis of the contract’s NT.
///
pub enum FeeBasis {
    /// Absolute Value: The fee rate represents an absolute value.
    A = 0,

    /// Nominal Value of the Underlying: The fee rate represents a rate that accrues fees on the basis of the nominal value of the underlying.
    N = 1,
}
