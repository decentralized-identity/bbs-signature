%%%
title = "The BBS Signature Scheme"
abbrev = "The BBS Signature Scheme"
ipr= "none"
area = "Internet"
workgroup = "none"
submissiontype = "IETF"
keyword = [""]

[seriesInfo]
name = "Individual-Draft"
value = "draft-bbs-signatures-latest"
status = "informational"

[[author]]
initials = "M."
surname = "Lodder"
fullname = "Mike Lodder"
#role = "editor"
organization = "CryptID"
  [author.address]
  email = "redmike7gmail.com"

[[author]]
initials = "T."
surname = "Looker"
fullname = "Tobias Looker"
#role = "editor"
organization = "Mattr"
  [author.address]
  email = "tobias.looker@mattr.global"

[[author]]
initials = "A."
surname = "Whitehead"
fullname = "Andrew Whitehead"
#role = "editor"
organization = ""
  [author.address]
  email = "cywolf@gmail.com"
%%%

.# Abstract

BBS is a digital signature scheme categorized as a form of short group signature that supports several novel properties. Notably, the scheme supports signing multiple messages whilst producing a single output digital signature. Through this capability, the possessor of a signature is able to derive proofs that selectively reveal subsets of the originally signed set of messages, whilst preserving the verifiable authenticity and integrity of the messages. Furthermore, these derived proofs are said to be zero-knowledge in nature as they do not reveal the underlying signature; instead, what they reveal is a proof of knowledge of the undisclosed signature.

{mainmatter}

# Introduction

A digital signature scheme is a fundamental cryptographic primitive that is used to provide data integrity and verifiable authenticity in various protocols. The core premise of digital signature technology is built upon asymmetric cryptography where-by the possessor of a private key is able to sign a message, where anyone in possession of the corresponding public key matching that of the private key is able to verify the signature.

However, traditional digital signature schemes require both the signature and the entire message to be disclosed during verification, constraining its usage in certain applications.

The BBS Signature scheme on the other hand allows a party to sign multiple messages and produce a single output signature.This then allows the possessor a signature to derive proofs from it that selectively reveal from the originally signed set of messages, whilst continuing to retain the underlying core properties of a digital signature which are verifiable authenticity and integrity of the revealed messages back to the original signer. Futhermore these derived proofs are said to be zero-knowledge in nature as they do not reveal the underlying signature, instead the generated proof is considered a proof of knowledge of the signature which is beneficial for applications where revealing the underlying signature can cause undesirable correlation.

## Terminology

The following terminology is used throughout this document:

SK
: The secret key for the signature scheme.

PK
: The public key for the signature scheme.

L
: The total number of messages that the signature scheme can sign.

R
: The set of message indices that are retained or hidden in a signature proof of knowledge.

D
: The set of message indices that are disclosed in a signature proof of knowledge.

msg
: An input message to be signed by the signature scheme.

generator
: A valid point on the selected sub-group of the curve being used that is used to commit a value.

H\[i\]
: The generator corresponding to a given msg.

H0
: A generator for the blinding value in the signature.

signature
: The digital signature output.

commitment
: A pedersen commitment composed of 1 or more messages.

nonce
: A cryptographic nonce

presentation_message (pm)
: A message generated and bound to the context of a specific spk.

spk
: Zero-Knowledge Signature Proof of Knowledge.

nizk
: A non-interactive zero-knowledge proof from fiat-shamir heuristic.

dst
: The domain separation tag.

I2OSP
: As defined by Section 4 of [@!RFC8017]

OS2IP
: As defined by Section 4 of [@!RFC8017].

## Notation

The following notation and primitives are used:

a || b
: Denotes the concatenation of octet strings a and b.

I \ J
: For sets I and J, denotes the difference of the two sets i.e., all the elements of I that do not appear in J, in the same order as they were in I.

\[n\]: Denotes all integers from 1 to n.

Terms specific to pairing-friendly elliptic curves that are relevant to this document are restated below, originally defined in [@!I-D.irtf-cfrg-pairing-friendly-curves]

E1, E2
: elliptic curve groups defined over finite fields. This document assumes that E1 has a more compact representation than E2, i.e., because E1 is defined over a smaller field than E2.

G1, G2
: subgroups of E1 and E2 (respectively) having prime order p.

GT
: a subgroup, of prime order p, of the multiplicative group of a field extension.

e
: G1 x G2 -> GT: a non-degenerate bilinear map.

q
: The prime order of the G1 and G2 subgroups.

P1, P2
: points on G1 and G2 respectively. For a pairing-friendly curve, this document denotes operations in E1 and E2 in additive notation, i.e., P + Q denotes point addition and x \* P denotes scalar multiplication. Operations in GT are written in multiplicative notation, i.e., a \* b is field multiplication.

Identity_G1, Identity_G1
: The identity element for the G1 and G2 subgroups respectively.

hash\_to\_curve\_g1(ostr) -> P
: A cryptographic hash function that takes as an arbitrary octet string input and returns a point in G1 as defined in [@!I-D.irtf-cfrg-hash-to-curve].

point\_to\_octets(P) -> ostr
: returns the canonical representation of the point P as an octet string. This operation is also known as serialization.

octets\_to\_point(ostr) -> P
: returns the point P corresponding to the canonical representation ostr, or INVALID if ostr is not a valid output of point\_to\_octets.  This operation is also known as deserialization.

subgroup\_check(P) -> VALID or INVALID
: returns VALID when the point P is an element of the subgroup of order p, and INVALID otherwise. This function can always be implemented by checking that p \* P is equal to the identity element.  In some cases, faster checks may also exist, e.g., [Bowe19].

## Organization of this document

This document is organized as follows:

* (#scheme-definition) defines the BBS signature scheme including any parameters required to define a concrete ciphersuite.

* (#security-considerations) defines security considerations associated to the signature scheme.

* (#ciphersuites) defines the format of a ciphersuite alongside a concrete ciphersuite based on the BLS12-381 curve.

# Conventions

The keywords **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**,
**SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL**, when they appear in this
document, are to be interpreted as described in [@!RFC2119].

# Scheme Definition

This section defines the BBS signature scheme, including the parameters required to define a concrete cipher-suite.

## Parameters

The schemes operations defined in (#operations) depend the following parameters:

* A pairing-friendly elliptic curve, plus associated functionality given in Section 1.4.

* HASH, a hash function that MUST be a secure cryptographic hash function. For security, H MUST output at least ceil(log2(q)) bits, where q is the order of the subgroups G1 and G2 defined by the pairing-friendly elliptic curve.

* XOF, a cryptographically secure extendable-output function like SHAKE128 or SHAKE256. XOF input is an octet string and outputs any desirable amount of bytes using the `.read(int)` method.

* PRF(n): a pseudo-random function similar to [@!RFC4868]. Returns n pseudo randomly generated bytes.

## Considerations

### Subgroup Selection

In definition of this signature scheme there are two possible variations based upon the sub-group selection, namely where public keys are defined in G2 and signatures in G1 OR the opposite where public keys are defined in G1 and signatures in G2. Some pairing cryptography based digital signature schemes such as [@I-D.irtf-cfrg-bls-signature] elect to allow for both variations, because they optimize for different things. However, in the case of this scheme, due to the operations involved in both signature and proof generation being computational in-efficient when performed in G2 and in the pursuit of simplicity, the scheme is limited to a construction where public keys are in G2 and signatures in G1.

### Messages and generators

Throughout the operations of this signature scheme, each message that is signed is paired with a specific generator (point in G1). Specifically, if a generator `H_1` is raised to the power of `msg_1` during signing, then `H_1` should be raised to the power of `msg_1` in all other operations as well (signature verification, proof generation and proof verification). For simplicity, each function will take as input the list of generators to be used with the messages. Those generators can be any distinct element from the generators list `H`. Applications for efficiency can elect to pass the indexes of those generators to the list `H` instead. Care must be taken for the correct generator to be raised to the correct message in that case.

## Operations

### KeyGen

The KeyGen algorithm generates a secret key SK deterministically from a secret octet string IKM.

KeyGen uses an HKDF [@!RFC5869] instantiated with the hash function H.

For security, IKM MUST be infeasible to guess, e.g., generated by a trusted source of randomness.

IKM MUST be at least 32 bytes long, but it MAY be longer.

Because KeyGen is deterministic, implementations MAY choose either to store the resulting SK or to store IKM and call KeyGen to derive SK when necessary.

KeyGen takes an optional parameter, key\_info. This parameter MAY be used to derive multiple independent keys from the same IKM.  By default, key\_info is the empty string.

```
SK = KeyGen(IKM)

Inputs:

- IKM, a secret octet string. See requirements above.

Outputs:

- SK, a uniformly random integer such that 0 < SK < q.

Parameters:

- key_info, an optional octet string. if this is not supplied, it MUST default to an empty string.

Definitions:

- HKDF-Extract is as defined in [@!RFC5869], instantiated with hash H.
- HKDF-Expand is as defined in [@!RFC5869], instantiated with hash H.
- I2OSP and OS2IP are as defined in [@!RFC8017], Section 4.
- L is the integer given by ceil((3 * ceil(log2(q))) / 16).
- "BBS-SIG-KEYGEN-SALT-" is an ASCII string comprising 20 octets.

Procedure:
1. salt = "BBS-SIG-KEYGEN-SALT-"

2. SK = 0

3. while SK == 0:

4.     salt = H(salt)

5.     PRK = HKDF-Extract(salt, IKM || I2OSP(0, 1))

6.     OKM = HKDF-Expand(PRK, key_info || I2OSP(L, 2), L)

7.     SK = OS2IP(OKM) mod q

8. return SK
```

### SkToPk

SkToPk algorithm takes a secret key SK and outputs a corresponding public key.

```
PK = SkToPk(SK)

Inputs:

- SK, a secret integer such that 0 < SK < q

Outputs:

- PK, a public key encoded as an octet string

Procedure:

1. W = SK * P2

2. PK = W

3. return point_to_octets(PK)
```

### KeyValidate

KeyValidate checks if the public key is valid.

As an optimization, implementations MAY cache the result of KeyValidate in order to avoid unnecessarily repeating validation for known keys.

```
result = KeyValidate(PK)

Inputs:

- PK, a public key in the format output by SkToPk.

Outputs:

- result, either VALID or INVALID

Procedure:

1. (W, H0, H) = octets_to_point(PK)

2. If W == Identity_G2, return INVALID

3. result = subgroup_check(W) && subgroup_check(H0)

4. for i in 0 to len(H): result &= subgroup_check(H[i])

5. return result
```

### Sign

Sign computes a signature from SK, PK, over a vector of messages. This method
describes deterministic signing. For threshold signing, XOF can be replaced
with a PRF due to the insecurity of deterministic threshold signing.

```
signature = Sign(SK, PK, (msg_1,..., msg_L), (H_1,..., H_L))

Inputs:

- msg_1,...,msg_L, octet strings. Messages to be signed.
- H_1,..., H_L, points of G1. Generators used to sign the messages.
- SK, a secret key output from KeyGen
- PK, a public key output from SkToPk

Outputs:

- signature, an octet string

Procedure:

1. (W, H0, H) = octets_to_point(PK)

2. h = XOF(SK  || msg[i] || ... || msg[L])

3. k = h.read(64)

4. e = OS2IP(k) mod q. If e = 0, go back to step 3.

5. r = h.read(64)

6. s = OS2IP(r) mod q. If s = 0, go back to step 5.

7. B = P1 + H0 * s + H_1 * msg_1 + ... + H_L * msg_L

8. A = B * (1 / (SK + e))

9. signature = (point_to_octets_min(A), e, s)

10. return signature
```

### Verify

Verify checks that a signature is valid for the octet string messages under the public key.

```
result = Verify(PK, (msg_1,..., msg_L), (H_1,..., H_L), signature)

Inputs:

- msg_1,..., msg_L, octet strings. Messages in input to Sign.
- H_1,..., H_L, points of G1. The generators in input to Sign.
- signature, octet string.
- PK, a public key in the format output by SkToPk.

Outputs:

- result, either VALID or INVALID.

Procedure:

1. (A, e, s) = (octets_to_point(signature.A), OS2IP(signature.e), OS2IP(signature.s))

2. pub_key = octets_to_point(PK)

3. if subgroup_check(A) is INVALID

4. if KeyValidate(pub_key) is INVALID

5. B = P1 + H0 * s + H_1 * msg_1 + ... + H_L * msg_L

6. C1 = e(A, W + P2 * e)

7. C2 = e(B, P2)

8. return C1 == C2
```

### SpkGen

A signature proof of knowledge generating algorithm that creates a zero-knowledge proof of knowledge of a signature while selectively disclosing messages from a signature given a vector of messages, a vector of indices of the revealed messages, the signer's public key, and a presentation message.

If an application chooses to pass the indexes of the generators instead, then it will also need to pass the indexes of the generators corresponding to the revealed messages.

```
spk = SpkGen(PK, (msg_1,..., msg_L), (H_1,..., H_L), RevealedIndexes, signature, pm)

Inputs:

- PK, octet string in output form from SkToPk
- msg_1,..., msg_L, octet strings. Messages in input to Sign.
- H_1,..., H_L, points of G1. The generators in input to Sign.
- RevealedIndexes, vector of unsigned integers. Indexes of revealed messages.
- signature, octet string in output form from Sign
- pm, octet string

Outputs:

- spk, octet string

Procedure:

1. (A, e, s) = (octets_to_point(signature.A), OS2IP(signature.e), OS2IP(signature.s))

2. (i1, i2,..., iR) = RevealedIndexes

3. (j1, j2,..., jU) = [L] \ RevealedIndexes

4. if subgroup_check(A) is INVALID abort

5. if KeyValidate(PK) is INVALID abort

6. b = P1 + H0 * s + H_1 * msg_1 + ... + H_L * msg_L

7. r1 = HASH(PRF(8*ceil(log2(q)))) mod q

8. r2 = HASH(PRF(8*ceil(log2(q)))) mod q

9. e~ = HASH(PRF(8*ceil(log2(q)))) mod q

10. r2~ = HASH(PRF(8*ceil(log2(q)))) mod q

11. r3~ = HASH(PRF(8*ceil(log2(q)))) mod q

12. s~ = HASH(PRF(8*ceil(log2(q)))) mod q

13. r3 = r1 ^ -1 mod q

14. for j in (j1, j2,..., jU): m~_j = HASH(PRF(8*ceil(log2(q)))) mod q

15. A' = A * r1

16. Abar = A' * (-e) + B * r1

17. D = B * r1 + h0 * r2

18. s' = s + r2 * r3

19. C1 = A' * e~ + H0 * r2~

20. C2 = D * (-r3~) + H0 * s~ + H_j1 * m~_j1 + ... + H_jU * m~_jU

21. c = HASH(PK || Abar || A' || D || C1 || C2 || pm)

22. e^ = e~ + c * e

23. r2^ = r2~ + c * r2

24. r3^ = r3~ + c * r3

25. s^ = s~ + c * s'

26. for j in (j1, j2,..., jU): m^_j = m~_j + c * msg_j

27. spk = ( A', Abar, D, c, e^, r2^, r3^, s^, (m^_j1, ..., m^_jU))

28. return spk
```

#### Algorithmic Explanation

The following section provides an explanation of how the Signature Proof Of Knowledge Generation (SpkGen) works.

Let the prover be in possession of a BBS signature `(A, e, s)` with `A = B * (1/(e + Sk))` where `Sk` the signer's secret key and,

    B = P1 + h0 * s + h[1] * msg_1 + ... + h[L] * msg_L

(without loss of generality we assume that the messages and generators are indexed from 0 to L). Let `(i1,...,iR)` be the indexes of generators corresponding to messages the prover wants to disclose and `(j1,...,jU)` be the indexes corresponding to undisclosed messages (i.e., `(j1,...,jU) = [L] \ (i1,...,iR)`). To prove knowledge of a signature on the disclosed messages, work as follows,

- Randomize the signature `(A, e, s)`, by taking uniformly random `r1`, `r2` in [1, q-1], and calculate,

        1.  A' = A * r1,
        2.  Abar = A' * (-e) + B * r1
        3.  D = B * r1 + H0 * r2.

  Also set,

        4.  r3 = r1 ^ -1 mod q
        5.  s' = s + r2 * r3.

  The values `(A', Abar, d)` will be part of the spk and are used to prove possession of a BBS signature, without revealing the signature itself. Note that; `e(A', Pk) = e(Abar, P2)` where `Pk` the signer's public key and P2 the base element in G2 (used to create the signer’s `Pk`, see [SkToPk](#sktopk)). This also serves to bind the spk to the signer's `Pk`.

- Set the following,

        1.  C1 = Abar - D
        2.  C2 = P1 +  H_i1 * msg_i1 + ... + H_iR * msg_iR

  Create a non-interactive zero-knowledge generalized Schnorr proof of knowledge (`nizk`) of the values `e, r2, r3, s'` and `msg_j1,...,msg_jU` (the undisclosed messages) so that both of the following equalities hold,

        EQ1.  C1 = A' * (-e) - H0 * r2
        EQ2.  C2 = D * (-r3) + H0 * s' + H_j1 * msg_j1 + ... + H_jU * msg_jU.

  If both EQ1 and EQ2 hold, and `e(A', Pk) = e(Abar, P2)`, an extractor can return a valid BBS signature from the signers `Sk`, on the disclosed messages. The spk returned is `(A', Abar, d, nizk)`. To validate the spk, a verifier checks that `e(A', Pk) = e(Abar, P2)` and verifies `nizk`.

### SpkVerify

SpkVerify checks if a signature proof of knowledge is VALID given the proof, the signer's public key, a vector of revealed messages, a vector with the indices of these revealed messages, and the presentation message used in SpkGen.

```
result = SpkVerify(spk, PK, (msg_i1,..., msg_iR), (H_1,..., H_L), RevealedIndexes, pm)

Inputs:

- spk, octet string.
- PK, octet string in output form from SkToPk.
- msg_i1,..., msg_iR, octet strings. The revealed messages in input to spkGen.
- H_1,..., H_L, points of G1. The generators in input to Sign.
- RevealedIndexes, vector of unsigned integers. Indexes of revealed messages.
- pm, octet string

Outputs:

- result, either VALID or INVALID.

Procedure:

1. if KeyValidate(PK) is INVALID

2. (i1, i2, ..., iR) = RevealedIndexes

3. (j1, j2, ..., jU) = [L]\RevealedIndexes

4. (A', Abar, D, c, e^, r2^, r3^, s^, (m^_j1,...,m^_jU)) = spk

5. C1 = (Abar - D) * c + A' * e^ + H0 * r2^

6. T = P1 + H_i1 * msg_i1 + ... H_iR * msg_iR

7. C2 = T * c + D * (-r3^) + H0 * s^ + H_j1 * m^_j1 + ... + H_jU * m^_jU

8. cv = HASH(PK || Abar || A' || D || C1 || C2 || pm)

9. if c != cv return INVALID

10. if A' == 1 return INVALID

11. if e(A', W) * e(Abar, -P2) != 1 return INVALID

12. return VALID
```

### CreateGenerators

The CreateGenerators operation defines how to create a set of generators that form a part of the public parameters used by the BBS Signature scheme to accomplish operations such as sign, verify, spkgen and spkverify.

*Note* The scope in which the seed used below is determined, is still an active conversation in the working group see (#ciphersuites) for the current method being used.

```
generators = CreateGenerators(dst, message_generator_seed, length);

Inputs:

dst, octet string - Domain Separation Tag
message_generator_seed, octet string
length, unsigned integer - Number of generators to create from the seed and dst

Outputs:

- generators, an array of generators

Procedure:

1. h = XOF(seed)

2. for i in 0 to length:

3.    generator_i = Identity_G1

4.    while(generator_i == Identity_G1 or generator_i == P1)

5.        candidate = hash_to_curve_g1(h.read(64), dst)

6.        if candidate not in generators: generator_i = candidate

3. return generators
```

# Security Considerations

## Validating public keys

All algorithms in Section 2 that operate on points in public keys require first validating those keys.  For the sign, verify and proof schemes, the use of KeyValidate is REQUIRED.

## Skipping membership checks

Some existing implementations skip the subgroup\_check invocation in Verify (Section 2.8), whose purpose is ensuring that the signature is an element of a prime-order subgroup.  This check is REQUIRED of conforming implementations, for two reasons.

1.  For most pairing-friendly elliptic curves used in practice, the pairing operation e (Section 1.3) is undefined when its input points are not in the prime-order subgroups of E1 and E2. The resulting behavior is unpredictable, and may enable forgeries.

2.  Even if the pairing operation behaves properly on inputs that are outside the correct subgroups, skipping the subgroup check breaks the strong unforgeability property [ADR02].

## Side channel attacks

Implementations of the signing algorithm SHOULD protect the secret key from side-channel attacks.  One method for protecting against certain side-channel attacks is ensuring that the implementation executes exactly the same sequence of instructions and performs exactly the same memory accesses, for any value of the secret key. In other words, implementations on the underlying pairing-friendly elliptic curve SHOULD run in constant time.

## Randomness considerations

The IKM input to KeyGen MUST be infeasible to guess and MUST be kept secret. One possibility is to generate IKM from a trusted source of randomness.  Guidelines on constructing such a source are outside the scope of this document.

Secret keys MAY be generated using other methods; in this case they MUST be infeasible to guess and MUST be indistinguishable from uniformly random modulo q.

BBS signatures are nondeterministic, meaning care must be taken against attacks arising from signing with bad randomness, for example, the nonce reuse attack on ECDSA [HDWH12]. It is RECOMMEDNED that the nonces and presentation messages used in this specification are chosen at random from a trusted source of randomness (see "Presentation message selection" section below for additional considerations).

BlindSign as discussed in 2.10 uses randomness from two parties so care MUST be taken that both sources of randomness are trusted. If one party uses weak randomness, it could compromise the signature.

When a trusted source of randomness is used, signatures and proofs are much harder to forge or break due to the use of multiple nonces.

## Presentation message selection

The signature proofs of knowledge generated in this specification are created using a specified presentation message. A verifier-specified cryptographically random value (e.g., a nonce) featuring in the presentation message provides strong protections against replay attacks, and is RECOMMENDED in most use cases. In some settings, proofs can be generated in a non-interactive fashion, in which case verifiers MUST be able to verify the uniqueness of the presentation message values.

## Implementing hash\_to\_curve\_g1

The security analysis models hash\_to\_curve\_g1 as random oracles.  It is crucial that these functions are implemented using a cryptographically secure hash function.  For this purpose, implementations MUST meet the requirements of [@!I-D.irtf-cfrg-hash-to-curve].

In addition, ciphersuites MUST specify unique domain separation tags for hash\_to\_curve.  Some guidance around defining this can be found in (#ciphersuites).

## Use of Contexts

Contexts can be used to separate uses of the protocol between different protocols (which is very hard to reliably do otherwise) and between different uses within the same protocol. However, the following SHOULD be kept in mind:

The context SHOULD be a constant string specified by the protocol using it. It SHOULD NOT incorporate variable elements from the message itself.

Contexts SHOULD NOT be used opportunistically, as that kind of use is very error prone. If contexts are used, one SHOULD require all signature schemes available for use in that purpose support contexts.

Contexts are an extra input, which percolate out of APIs; as such, even if the signature scheme supports contexts, those may not be available for use. This problem is compounded by the fact that many times the application is not invoking the signing, verification, and proof functions directly but via some other protocol.

The ZKP protocols use nonces which MUST be different in each context.

## Choice of underlying curve

BBS signatures can be implemented on any pairing-friendly curve. However care MUST be taken when selecting one that is appropriate, this specification defines a profile for using the BLS12-381 curve in (#ciphersuites) which as a curve currently achieves close to 128-bit security.

## Security against the signer.

The BBS proof, as returned by spkGen, is a zero-knowledge proof-of-knowledge against an honest verifier. This guarantees that two different BBS proofs derived from the same signature will be indifferentiable by a verifier (or a coalition of verifiers). This does not hold against the original signer of the signature, or against a signer/verifier coalition. Specifically, given a proof derived from a BBS signature, the original signer will be able to match the proof to the signature used to derive it. As a result, a signer/ verifier coalition could correlate and track the prover. This also means that a verifier could ask the original signer to reveal the messages that the prover choose to keep undisclosed during a proof generation. Hence, the BBS proof is NOT zero-knowledge against the signer, or against dishonest verifiers cooperating with the signer.

In applications where zero-knowledge is desired even against the signer, the signer MUST NOT keep a copy of the signatures they issued (at least not of the signed messages and the `s` value). This is RECOMMENDED for all applications to alleviate the danger of a leaked signer’s key compromising the secrecy of the messages the prover choose to keep un-disclosed during a proof generation, since any adversary in possession of both the secret key of the signer and the value `s` of the signature could reveal those un-disclosed messages (by calculating `B` as calculated at step 7 of spkGen).

# Ciphersuites

This section defines the format for a BLS ciphersuite. It also gives concrete ciphersuites based on the BLS12-381 pairing-friendly elliptic curve [@!I-D.irtf-cfrg-pairing-friendly-curves].

## Ciphersuite Format

- H: a cryptographic hash function.

- point\_to\_octets:
a function that returns the canonical representation of the point P as an octet string.

- octets\_to\_point:
a function that returns the point P corresponding to the canonical representation ostr, or INVALID if ostr is not a valid output of point_to_octets.

- hash\_to\_curve\_g1:
A cryptographic hash function that takes as an arbitrary octet string input and returns a point in G1 as defined in [@!I-D.irtf-cfrg-hash-to-curve].

- dst: Domain separation tag used in the hash\_to\_curve\_g1 operation

- message_generator_seed: The seed used to generate the message generators which form part of the public parameters used by the BBS signature scheme, Note there are multiple possible scopes for this seed including; a globally shared seed (where the resulting message generators are common across all BBS signatures); a signer specific seed (where the message generators are specific to a signer); signature specific seed (where the message generators are specific per signature). The ciphersuite MUST define this seed OR how to compute it as a pre-cursor operations to any others.

## BLS12-381 Ciphersuite

H
: SHAKE-256 as defined in [@!SHA3]

point\_to\_octets
: follows the format documented in Appendix C section 1 of [@!I-D.irtf-cfrg-pairing-friendly-curves].

octets\_to\_point
: follows the format documented in Appendix C section 2 of [@!I-D.irtf-cfrg-pairing-friendly-curves].

hash\_to\_curve_g1
: follows the suite defined in (#bls12-381-hash-to-curve-definition-using-shake-256) for the G1 subgroup

dst
: "BBS_BLS12381G1_XOF:SHAKE-256_SSWU_RO_"

message_generator_seed
: A global seed value of "BBS_BLS12381G1_XOF:SHAKE-256_SSWU_RO_MESSAGE_GENERATOR_SEED" which is used by the (#creategenerators) operation to compute the required set of message generators.

### Test Vectors

The following section details a basic set of test vectors that can be used to confirm an implementations correctness

**NOTE** All binary data below is represented as octet strings encoded in hexadecimal format

**NOTE** These fixtures are a work in progress and subject to change

Further fixtures are available in (#additional-bls12-381-ciphersuite-test-vectors)

#### Message Generators

Following the procedure defined in (#creategenerators) with an input seed value of

```
BBS_BLS12381G1_XOF:SHAKE-256_SSWU_RO_MESSAGE_GENERATOR_SEED
```

a dst of

```
BBS_BLS12381G1_XOF:SHAKE-256_SSWU_RO_
```

and a length value of `10`

Outputs the following values

```
{{ $generators[0] }}

{{ $generators[1] }}

{{ $generators[2] }}

{{ $generators[3] }}

{{ $generators[4] }}

{{ $generators[5] }}

{{ $generators[6] }}

{{ $generators[7] }}

{{ $generators[8] }}

{{ $generators[9] }}
```

#### Key Pair

Following the procedure defined in (#keygen) with an input IKM value as follows

```
{{ $keyPair.seed }}
```

Outputs the following SK value

```
{{ $keyPair.keyPair.secretKey }}
```

Following the procedure defined in (#sktopk) with an input SK value as above produces the following PK value

```
{{ $keyPair.keyPair.publicKey }}
```

#### Valid Single Message Signature

Using the following message

```
{{ $signatureFixtures.signature001.messages[0] }}
```

Along with the SK value as defined in (#key-pair) as inputs into the Sign operations, yields the following output signature

```
{{ $signatureFixtures.signature001.signature }}
```

#### Valid Multi-Message Signature

Using the following messages (**Note** the ordering of the messages MUST be preserved)

```
{{ $signatureFixtures.signature004.messages[0] }}

{{ $signatureFixtures.signature004.messages[1] }}

{{ $signatureFixtures.signature004.messages[2] }}

{{ $signatureFixtures.signature004.messages[3] }}

{{ $signatureFixtures.signature004.messages[4] }}

{{ $signatureFixtures.signature004.messages[5] }}

{{ $signatureFixtures.signature004.messages[6] }}

{{ $signatureFixtures.signature004.messages[7] }}

{{ $signatureFixtures.signature004.messages[8] }}

{{ $signatureFixtures.signature004.messages[9] }}
```

Along with the SK value as defined in (#key-pair) as inputs into the Sign operations, yields the following output signature

```
{{ $signatureFixtures.signature004.signature }}
```

# IANA Considerations

This document does not make any requests of IANA.

{backmatter}

# Appendix

## BLS12-381 hash\_to\_curve definition using SHAKE-256

The following defines a hash_to_curve suite [@!I-D.irtf-cfrg-hash-to-curve] for the BLS12-381 curve for both the G1 and G2 subgroups using the extendable output function (XOF) of SHAKE-256 as per the guidance defined in section 8.9 of [@!I-D.irtf-cfrg-hash-to-curve].

Note the notation used in the below definitions is sourced from [@!I-D.irtf-cfrg-hash-to-curve].

### BLS12-381 G1

The suite of `BLS12381G1_XOF:SHAKE-256_SSWU_R0_` is defined as follows:

```
* encoding type: hash_to_curve (Section 3 of [@!I-D.irtf-cfrg-hash-to-curve])

* E: y^2 = x^3 + 4

* p: 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab

* m: 1

* k: 128

* expand_message: expand_message_xof (Section 5.4.2 of [@!I-D.irtf-cfrg-hash-to-curve])

* H: SHAKE-256

* L: 64

* f: Simplified SWU for AB == 0 (Section 6.6.3 of [@!I-D.irtf-cfrg-hash-to-curve])

* Z: 11

*  E': y'^2 = x'^3 + A' * x' + B', where

      -  A' = 0x144698a3b8e9433d693a02c96d4982b0ea985383ee66a8d8e8981aef
         d881ac98936f8da0e0f97f5cf428082d584c1d

      -  B' = 0x12e2908d11688030018b12e8753eee3b2016c1f0f24f4070a0b9c14f
         cef35ef55a23215a316ceaa5d1cc48e98e172be0

*  iso_map: the 11-isogeny map from E' to E given in Appendix E.2 of [@!I-D.irtf-cfrg-hash-to-curve]

*  h_eff: 0xd201000000010001
```

Note that the h_eff values for this suite are copied from that defined for the `BLS12381G1_XMD:SHA-256_SSWU_RO_` suite defined in section 8.8.1 of [@!I-D.irtf-cfrg-hash-to-curve].

An optimized example implementation of the Simplified SWU mapping to the curve E' isogenous to BLS12-381 G1 is given in Appendix F.2 [@!I-D.irtf-cfrg-hash-to-curve].

### BLS12-381 G2

The suite of `BLS12381G2_XOF:SHAKE-256_SSWU_R0_` is defined as follows:

```
* encoding type: hash_to_curve (Section 3 of [@!I-D.irtf-cfrg-hash-to-curve])

* E: y^2 = x^3 + 4 * (1 + I)

* base field F is GF(p^m), where

  -  p: 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6
         b0f6241eabfffeb153ffffb9feffffffffaaab

  -  m: 2

  -  (1, I) is the basis for F, where I^2 + 1 == 0 in F

* k: 128

* expand_message: expand_message_xof (Section 5.4.2 of [@!I-D.irtf-cfrg-hash-to-curve])

* H: SHAKE-256

* L: 64

* f: Simplified SWU for AB == 0 (Section 6.6.3 of [@!I-D.irtf-cfrg-hash-to-curve])

* Z: -(2 + I)

*  E': y'^2 = x'^3 + A' * x' + B', where

      -  A' = 240 * I

      -  B' = 1012 * (1 + I)

*  iso_map: the isogeny map from E' to E given in Appendix E.3 of [@!I-D.irtf-cfrg-hash-to-curve]

*  h_eff: 0xbc69f08f2ee75b3584c6a0ea91b352888e2a8e9145ad7689986ff0315
  08ffe1329c2f178731db956d82bf015d1212b02ec0ec69d7477c1ae954cbc06689
  f6a359894c0adebbf6b4e8020005aaa95551
```

Note that the h_eff values for this suite are copied from that defined for the `BLS12381G2_XMD:SHA-256_SSWU_RO_` suite defined in section 8.8.1 of [@!I-D.irtf-cfrg-hash-to-curve].

An optimized example implementation of the Simplified SWU mapping to the curve E' isogenous to BLS12-381 G2 is given in Appendix F.2 [@!I-D.irtf-cfrg-hash-to-curve].

## Usecases

### Non-correlating Security Token

In the most general sense BBS signatures can be used in any application where a cryptographically secured token is required but correlation caused by usage of the token is un-desirable.

For example in protocols like OAuth2.0 the most commonly used form of the access token leverages the JWT format alongside conventional cryptographic primitives such as traditional digital signatures or HMACs. These access tokens are then used by a relying party to prove authority to a resource server during a request. However, because the access token is most commonly sent by value as it was issued by the authorization server (e.g in a bearer style scheme), the access token can act as a source of strong correlation for the relying party. Relevant prior art can be found [here](https://www.ietf.org/archive/id/draft-private-access-tokens-01.html).

BBS Signatures due to their unique properties removes this source of correlation but maintains the same set of guarantees required by a resource server to validate an access token back to its relevant authority (note that an approach to signing JSON tokens with BBS that may be of relevance is the [JWP](https://json-web-proofs.github.io/json-web-proofs/draft-jmiller-json-web-proof.html) format and serialization). In the context of a protocol like OAuth2.0 the access token issued by the authorization server would feature a BBS Signature, however instead of the relying party providing this access token as issued, in their request to a resource server, they derive a unique proof from the original access token and include that in the request instead, thus removing this vector of correlation.

### Improved Bearer Security Token

Bearer based security tokens such as JWT based access tokens used in the OAuth2.0 protocol are a highly popular format for expressing authorization grants. However their usage has several security limitations. Notably a bearer based authorization scheme often has to rely on a secure transport between the authorized party (client) and the resource server to mitigate the potential for a MITM attack or a malicious interception of the access token. The scheme also has to assume a degree of trust in the resource server it is presenting an access token to, particularly when the access token grants more than just access to the target resource server, because in a bearer based authorization scheme, anyone who possesses the access token has authority to what it grants. Bearer based access tokens also suffer from the threat of replay attacks.

Improved schemes around authorization protocols often involve adding a layer of proof of cryptographic key possession to the presentation of an access token, which mitigates the deficiencies highlighted above as well as providing a way to detect a replay attack. However, approaches that involve proof of cryptographic key possession such as DPoP (https://datatracker.ietf.org/doc/html/draft-ietf-oauth-dpop-04) suffer from an increase in protocol complexity. A party requesting authorization must pre-generate appropriate key material, share the public portion of this with the authorization server alongside proving possession of the private portion of the key material. The authorization server must also be-able to accommodate receiving this information and validating it.

BBS Signatures ofter an alternative model that solves the same problems that proof of cryptographic key possession schemes do for bearer based schemes, but in a way that doesn't introduce new up-front protocol complexity. In the context of a protocol like OAuth2.0 the access token issued by the authorization server would feature a BBS Signature, however instead of the relying party providing this access token as issued, in their request to a resource server, they derive a unique proof from the original access token and include that in the request instead. Because the access token is not shared in a request to a resource server, attacks such as MITM are mitigated. A resource server also obtains the ability to detect a replay attack by ensuring the proof presented is unique.

### Hardware Attestations

TODO

### Selectively Disclosure Enabled Identity Assertions

TODO

### Privacy preserving bound signatures

TODO

## Additional BLS12-381 Ciphersuite Test Vectors

**NOTE** These fixtures are a work in progress and subject to change

### Modified Message Signature

Using the following message

```
{{ $signatureFixtures.signature002.messages[0] }}
```

And the following signature

```
{{ $signatureFixtures.signature002.signature }}
```

Along with the PK value as defined in (#key-pair) as inputs into the Verify operation should fail signature validation due to the message value being different from what was signed

### Extra Unsigned Message Signature

Using the following messages

```
{{ $signatureFixtures.signature003.messages[0] }}

{{ $signatureFixtures.signature003.messages[1] }}
```

And the following signature

```
{{ $signatureFixtures.signature002.signature }}
```

Along with the PK value as defined in (#key-pair) as inputs into the Verify operation should fail signature validation due to an additional message being supplied that was not signed

### Missing Message Signature

Using the following messages

```
{{ $signatureFixtures.signature005.messages[0] }}

{{ $signatureFixtures.signature005.messages[1] }}
```

And the following signature

```
{{ $signatureFixtures.signature005.signature }}
```

Along with the PK value as defined in (#key-pair) as inputs into the Verify operation should fail signature validation due to missing messages that were originally present during the signing

### Reordered Message Signature

Using the following messages

```
{{ $signatureFixtures.signature006.messages[0] }}

{{ $signatureFixtures.signature006.messages[1] }}

{{ $signatureFixtures.signature006.messages[2] }}

{{ $signatureFixtures.signature006.messages[3] }}

{{ $signatureFixtures.signature006.messages[4] }}

{{ $signatureFixtures.signature006.messages[5] }}

{{ $signatureFixtures.signature006.messages[6] }}

{{ $signatureFixtures.signature006.messages[7] }}

{{ $signatureFixtures.signature006.messages[8] }}

{{ $signatureFixtures.signature006.messages[9] }}
```

And the following signature

```
{{ $signatureFixtures.signature006.signature }}
```

Along with the PK value as defined in (#key-pair) as inputs into the Verify operation should fail signature validation due to messages being re-ordered from the order in which they were signed

### Wrong Public Key Signature

Using the following messages

```
{{ $signatureFixtures.signature007.messages[0] }}

{{ $signatureFixtures.signature007.messages[1] }}

{{ $signatureFixtures.signature007.messages[2] }}

{{ $signatureFixtures.signature007.messages[3] }}

{{ $signatureFixtures.signature007.messages[4] }}

{{ $signatureFixtures.signature007.messages[5] }}

{{ $signatureFixtures.signature007.messages[6] }}

{{ $signatureFixtures.signature007.messages[7] }}

{{ $signatureFixtures.signature007.messages[8] }}

{{ $signatureFixtures.signature007.messages[9] }}
```

And the following signature

```
{{ $signatureFixtures.signature007.signature }}
```

Along with the PK value as defined in (#key-pair) as inputs into the Verify operation should fail signature validation due to public key used to verify is in-correct

<reference anchor="SHA3" target="https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-208.pdf">
 <front>
   <title>Recommendation for Stateful Hash-Based Signature Schemes</title>
   <author><organization>NIST</organization></author>
 </front>
</reference>
