//
// N.B. Auto-generated using actus-mp
//
mod ann;
mod anx;
mod bndcp;
mod bndwr;
mod capfl;
mod cdswp;
mod cec;
mod ceg;
mod clm;
mod clnte;
mod com;
mod csh;
mod futur;
mod fxout;
mod lam;
mod lax;
mod mar;
mod nam;
mod nax;
mod optns;
mod pam;
mod pbn;
mod rep;
mod scrcr;
mod scrmr;
mod stk;
mod swaps;
mod swppv;
mod trswp;
mod ump;

pub use ann::AnnuityTermset;
pub use anx::ExoticAnnuityTermset;
pub use bndcp::ConvertibleNoteTermset;
pub use bndwr::WarrantTermset;
pub use capfl::CapFloorTermset;
pub use cdswp::CreditDefaultSwapTermset;
pub use cec::CollateralTermset;
pub use ceg::GuaranteeTermset;
pub use clm::CallMoneyTermset;
pub use clnte::CreditLinkedNoteTermset;
pub use com::CommodityTermset;
pub use csh::CashTermset;
pub use futur::FutureTermset;
pub use fxout::ForeignExchangeOutrightTermset;
pub use lam::LinearAmortizerTermset;
pub use lax::ExoticLinearAmortizerTermset;
pub use mar::MarginingTermset;
pub use nam::NegativeAmortizerTermset;
pub use nax::ExoticNegativeAmortizerTermset;
pub use optns::OptionTermset;
pub use pam::PrincipalAtMaturityTermset;
pub use pbn::PerpetualBondsTermset;
pub use rep::RepurchaseAgreementTermset;
pub use scrcr::SecuritizationCreditRiskTermset;
pub use scrmr::SecuritizationMarketRiskTermset;
pub use stk::StockTermset;
pub use swaps::SwapTermset;
pub use swppv::PlainVanillaSwapTermset;
pub use trswp::TotalReturnSwapTermset;
pub use ump::UndefinedMaturityProfileTermset;

pub enum ContractTermset {
   ANN(AnnuityTermset),
   ANX(ExoticAnnuityTermset),
   BNDCP(ConvertibleNoteTermset),
   BNDWR(WarrantTermset),
   CAPFL(CapFloorTermset),
   CDSWP(CreditDefaultSwapTermset),
   CEC(CollateralTermset),
   CEG(GuaranteeTermset),
   CLM(CallMoneyTermset),
   CLNTE(CreditLinkedNoteTermset),
   COM(CommodityTermset),
   CSH(CashTermset),
   FUTUR(FutureTermset),
   FXOUT(ForeignExchangeOutrightTermset),
   LAM(LinearAmortizerTermset),
   LAX(ExoticLinearAmortizerTermset),
   MAR(MarginingTermset),
   NAM(NegativeAmortizerTermset),
   NAX(ExoticNegativeAmortizerTermset),
   OPTNS(OptionTermset),
   PAM(PrincipalAtMaturityTermset),
   PBN(PerpetualBondsTermset),
   REP(RepurchaseAgreementTermset),
   SCRCR(SecuritizationCreditRiskTermset),
   SCRMR(SecuritizationMarketRiskTermset),
   STK(StockTermset),
   SWAPS(SwapTermset),
   SWPPV(PlainVanillaSwapTermset),
   TRSWP(TotalReturnSwapTermset),
   UMP(UndefinedMaturityProfileTermset),
}
