# Cheetah

This crate provides an implementation for arithmetic operations in the fields of the STARK friendly cheetah curve, which are then utilized in the cheetah curve's implementation. These fields include :
* $\ F_p$ with p = 18446744069414584321.
* $\ F_{p^3}$ is a cubic extension of $\ F_p$.
* $\ F_{p^6}$ is a sextic extension of $\ F_{p}$ . The extension $\ F_{p^6}$ has been specifically constructed with a sparse polynomial of the form $\ X^6 - A$, where A is a small quadratic and cubic non-residue.
* Scalar field($\ F_{q}$) of prime order  with characteristic q = 55610362957290864006699123731285679659474893560816383126640993521607086746831 of 255-bit.