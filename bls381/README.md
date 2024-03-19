# BLS12-381

This crate provides an implementation for arithmetic operation in the fields of the BLS12-381 curve which is a pairing friendly curve. The BLS12-381 curve belongs to the **BLS** family of curves with embedding degree 12. The fields defined here are further used in the implementaion of the BLS12-381 curve.
These fields include :
* $\ F_p$ with p = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787 of 381-bit.
* $\ F_{p^2}$ is a quadratic extension of $\ F_p$ defined as $\ F_p(u) / (u^2 + 1)$. The elements of $\ F_{p^2}$ field are written as first degree polynomials in $\ u$, with coefficients from $\ F_p$ field.
* $\ F_{p^6}$ is a cubic extension of $\ F_{p^2}$ defined as $\ F_{p^2}(v) / (v^2 - (u +1))$. The elements of $\ F_{p^6}$ field are written as second degree polynomials in $\ v$, with coefficients from $\ F_{p^2}$ field.
* $\ F_{p^{12}}$ is a quadratic extension over $\ F_{p^6}$ defined as $\ F_{p^6}(w) / (w^2 - v)$. The elements of $\ F_{p^{12}}$ field are written as first degree polynomials in $\ w$, with coefficients from $\ F_{p^6}$ field.
* Scalar field($\ F_{q}$) of prime order with characteristic q = 52435875175126190479447740508185965837690552500527637822603658699938581184513 of 255-bit.

It also contains implementation for :
* A scalar field($\ F_{q}$) of prime order for jubjub curve with characteristic q = 6554484396890773809930967563523245729705921265872317281365359162392183254199 of 252-bit.
* A scalar field($\ F_{q}$) of prime orderfor bandersnatch curve  with characteristic q = 13108968793781547619861935127046491459309155893440570251786403306729687672801 of 253-bit.
* Algebraic hash functions are also defined over the scalar field of BLS12-381.