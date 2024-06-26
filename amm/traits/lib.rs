#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod factory;
mod pair;
mod router;
mod swap_callee;

pub type Balance = <ink::env::DefaultEnvironment as ink::env::Environment>::Balance;

pub use amm_helpers::math::MathError;
pub use factory::{Factory, FactoryError};
pub use pair::{Pair, PairError};
pub use router::{Router, RouterError};
pub use swap_callee::SwapCallee;
