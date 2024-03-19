# BN254

This crate provides an implementation for arithmetic operations in the fields of the BN254 curve which is a pairing friendly curve and further used in implementing the BN254 curve. The embedding degree of a BN254 curve is 12.
These fields include :
* $\ F_p$ with p = 21888242871839275222246405745257275088696311157297823662689037894645226208583 of 254-bit.
* $\ F_{p^2}$ is a quadratic extension of $\ F_p$ defined as $\ F_p(u) / (u^2 + 1)$. The elements of $\ F_{p^2}$ field are written as first degree polynomials in $\ u$, with coefficients from $\ F_p$ field.
* $\ F_{p^6}$ is a cubic extension of $\ F_{p^2}$ defined as $\ F_{p^2}(v) / ( v^3 - (u + 9))$. The elements of $\ F_{p^6}$ field are written as second degree polynomials in $\ v$, with coefficients from $\ F_{p^2}$ field.
* $\ F_{p^{12}}$ is a quadratic extension over $\ F_{p^6}$ defined as $\ F_{p^6}(w) / ( w^2 - v)$. The elements of $\ F_{p^{12}}$ field are written as first degree polynomials in $\ w$, with coefficients from $\ F_{p^6}$ field.
* Scalar field($\ F_{q}$) of prime order  with characteristic q = 21888242871839275222246405745257275088548364400416034343698204186575808495617 of 255 bits.

It also contains implementation for :
* Scalar field($\ F_{q}$) of prime order for baby jubjub curve  with characteristic q = 2736030358979909402780800718157159386076813972158567259200215660948447373041 of 251-bit.
* Algebraic hash functions are defined over the scalar field of BN254.