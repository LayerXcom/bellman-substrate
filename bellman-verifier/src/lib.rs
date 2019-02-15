#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[macro_use]
extern crate parity_codec_derive;
#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

use pairing::{
    Engine,
    CurveAffine,
    EncodedPoint,
    IoError,
    GroupDecodingError,
};

use rstd::prelude::*;
#[cfg(feature = "std")]
use std::{io, fmt::{self, Debug}, error::Error};

mod verifier;
mod dummy_engine;

pub use self::verifier::*;
pub use pairing::utils::*;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, Default, Eq)]
pub struct Proof<E: Engine> {
    pub a: E::G1Affine,
    pub b: E::G2Affine,
    pub c: E::G1Affine
}

impl<E: Engine> PartialEq for Proof<E> {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a &&
        self.b == other.b &&
        self.c == other.c
    }
}

impl<E: Engine> Proof<E> {        
    pub fn write(
        &self,
        writer: &mut Vec<u8>           
    ) -> Result<(), IoError>
    {                
        writer.write_all(self.a.into_compressed().as_ref())?;
        writer.write_all(self.b.into_compressed().as_ref())?;
        writer.write_all(self.c.into_compressed().as_ref())?;

        Ok(())
    }

    pub fn read(        
        mut reader: &[u8]
    ) -> Result<Self, IoError>
    {
        let mut g1_repr = <E::G1Affine as CurveAffine>::Compressed::empty();
        let mut g2_repr = <E::G2Affine as CurveAffine>::Compressed::empty();

        reader.read_exact(g1_repr.as_mut())?;

        let a = g1_repr
                .into_affine()
                // .map_err(|e| Error::new(io::ErrorKind::InvalidData, e))
                // .map_err(|e| Err(e))
                .and_then(|e| if e.is_zero() {
                    // Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                    Err(GroupDecodingError::NotOnCurve)
                } else {
                    Ok(e)
                })?;

        reader.read_exact(g2_repr.as_mut())?;
        
        let b = g2_repr
                .into_affine()
                // .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                // .map_err(|e| Err(e))
                .and_then(|e| if e.is_zero() {
                    // Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                    Err(GroupDecodingError::NotOnCurve)
                } else {
                    Ok(e)
                })?;

        reader.read_exact(g1_repr.as_mut())?;
        let c = g1_repr
                .into_affine()
                // .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                // .map_err(|e| Err(e))
                .and_then(|e| if e.is_zero() {
                    // Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                    Err(GroupDecodingError::NotOnCurve)
                } else {
                    Ok(e)
                })?;

        Ok(Proof {
            a: a,
            b: b,
            c: c
        })        
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone)]
pub struct VerifyingKey<E: Engine> {
    // alpha in g1 for verifying and for creating A/C elements of
    // proof. Never the point at infinity.
    pub alpha_g1: E::G1Affine,

    // beta in g1 and g2 for verifying and for creating B/C elements
    // of proof. Never the point at infinity.
    pub beta_g1: E::G1Affine,
    pub beta_g2: E::G2Affine,

    // gamma in g2 for verifying. Never the point at infinity.
    pub gamma_g2: E::G2Affine,

    // delta in g1/g2 for verifying and proving, essentially the magic
    // trapdoor that forces the prover to evaluate the C element of the
    // proof with only components from the CRS. Never the point at
    // infinity.
    pub delta_g1: E::G1Affine,
    pub delta_g2: E::G2Affine,

    // Elements of the form (beta * u_i(tau) + alpha v_i(tau) + w_i(tau)) / gamma
    // for all public inputs. Because all public inputs have a dummy constraint,
    // this is the same size as the number of inputs, and never contains points
    // at infinity.
    pub ic: Vec<E::G1Affine>
}

impl<E: Engine> PartialEq for VerifyingKey<E> {
    fn eq(&self, other: &Self) -> bool {
        self.alpha_g1 == other.alpha_g1 &&
        self.beta_g1 == other.beta_g1 &&
        self.beta_g2 == other.beta_g2 &&
        self.gamma_g2 == other.gamma_g2 &&
        self.delta_g1 == other.delta_g1 &&
        self.delta_g2 == other.delta_g2 &&
        self.ic == other.ic
    }
}

impl<E: Engine> VerifyingKey<E> {
    pub fn write(
        &self,
        writer: &mut Vec<u8>
    ) -> Result<(), IoError>
    {        
        writer.write_all(self.alpha_g1.into_uncompressed().as_ref())?;        
        writer.write_all(self.beta_g1.into_uncompressed().as_ref())?;
        writer.write_all(self.beta_g2.into_uncompressed().as_ref())?;
        writer.write_all(self.gamma_g2.into_uncompressed().as_ref())?;
        writer.write_all(self.delta_g1.into_uncompressed().as_ref())?;
        writer.write_all(self.delta_g2.into_uncompressed().as_ref())?;
        writer.write_u32(self.ic.len() as u32)?;
        for ic in &self.ic {
            writer.write_all(ic.into_uncompressed().as_ref())?;
        }

        Ok(())
    }

    pub fn read(
        mut reader: &[u8]
    ) -> Result<Self, IoError>
    {        
        let mut g1_repr = <E::G1Affine as CurveAffine>::Uncompressed::empty();        
        let mut g2_repr = <E::G2Affine as CurveAffine>::Uncompressed::empty();

        reader.read_exact(g1_repr.as_mut())?;               
        
        let alpha_g1 = g1_repr.into_affine().map_err(|e| Err(e))?;                    

        reader.read_exact(g1_repr.as_mut())?;
        let beta_g1 = g1_repr.into_affine().map_err(|e| Err(e))?;        

        reader.read_exact(g2_repr.as_mut())?;
        let beta_g2 = g2_repr.into_affine().map_err(|e| Err(e))?;

        reader.read_exact(g2_repr.as_mut())?;
        let gamma_g2 = g2_repr.into_affine().map_err(|e| Err(e))?;

        reader.read_exact(g1_repr.as_mut())?;
        let delta_g1 = g1_repr.into_affine().map_err(|e| Err(e))?;

        reader.read_exact(g2_repr.as_mut())?;
        let delta_g2 = g2_repr.into_affine().map_err(|e| Err(e))?;
            
        let ic_len = reader.read_u32().unwrap() as usize;        

        let mut ic = vec![];
        
        for _ in 0..ic_len {
            reader.read_exact(g1_repr.as_mut())?;            
            
            let g1 = g1_repr
                     .into_affine()
                    //  .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                     .and_then(|e| if e.is_zero() {                         
                        //  Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                         Err(GroupDecodingError::NotOnCurve)
                     } else {
                         Ok(e)
                     })?;

            ic.push(g1);
        }                

        Ok(VerifyingKey {
            alpha_g1: alpha_g1,
            beta_g1: beta_g1,
            beta_g2: beta_g2,
            gamma_g2: gamma_g2,
            delta_g1: delta_g1,
            delta_g2: delta_g2,
            ic: ic
        })
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, Default, PartialEq, Eq)]
pub struct PreparedVerifyingKey<E: Engine> {
    /// Pairing result of alpha*beta
    alpha_g1_beta_g2: E::Fqk,
    /// -gamma in G2
    neg_gamma_g2: <E::G2Affine as CurveAffine>::Prepared,
    /// -delta in G2
    neg_delta_g2: <E::G2Affine as CurveAffine>::Prepared,
    /// Copy of IC from `VerifiyingKey`.
    ic: Vec<E::G1Affine>
}

/// This is an error that could occur during circuit synthesis contexts,
/// such as CRS generation, proving or verification.
#[derive(Debug)]
pub enum SynthesisError {
    /// During synthesis, we lacked knowledge of a variable assignment.
    AssignmentMissing,
    /// During synthesis, we divided by zero.
    DivisionByZero,
    /// During synthesis, we constructed an unsatisfiable constraint system.
    Unsatisfiable,
    /// During synthesis, our polynomials ended up being too high of degree
    PolynomialDegreeTooLarge,
    /// During proof generation, we encountered an identity in the CRS
    UnexpectedIdentity,
    /// During proof generation, we encountered an I/O error with the CRS
    #[cfg(feature = "std")]
    IoError(io::Error),
    #[cfg(not(feature = "std"))]
    IoError,
    /// During verification, our verifying key was malformed.
    MalformedVerifyingKey,
    /// During CRS generation, we observed an unconstrained auxillary variable
    UnconstrainedVariable
}

impl SynthesisError {
    #[inline]
    fn description_str(&self) -> &'static str {
        match *self {
            SynthesisError::AssignmentMissing => "an assignment for a variable could not be computed",
            SynthesisError::DivisionByZero => "division by zero",
            SynthesisError::Unsatisfiable => "unsatisfiable constraint system",
            SynthesisError::PolynomialDegreeTooLarge => "polynomial degree is too large",
            SynthesisError::UnexpectedIdentity => "encountered an identity element in the CRS",
            #[cfg(feature = "std")]
            SynthesisError::IoError(_) => "encountered an I/O error",
            #[cfg(not(feature = "std"))]
            SynthesisError::IoError => "encountered an I/O error",
            SynthesisError::MalformedVerifyingKey => "malformed verifying key",
            SynthesisError::UnconstrainedVariable => "auxillary variable was unconstrained"
        }
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for SynthesisError {
    fn from(e: io::Error) -> SynthesisError {
        SynthesisError::IoError(e)
    }
}

#[cfg(feature = "std")]
impl Error for SynthesisError {
    fn description(&self) -> &str {
        self.description_str()       
    }
}

#[cfg(feature = "std")]
impl fmt::Display for SynthesisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // if let &SynthesisError::IoError(ref e) = self {
        //     write!(f, "I/O error: ")?;
        //     e.fmt(f)
        // } else {
            write!(f, "{}", self.description())
        // }
    }
}
