%%%
title = "Blind Signatures extension of the BBS Signature Scheme"
abbrev = "Blind Signatures extension of the BBS Signature Scheme"
ipr= "none"
area = "Internet"
workgroup = "none"
submissiontype = "IETF"
keyword = [""]

[seriesInfo]
name = "Individual-Draft"
value = "draft-blind-bbs-signatures-latest"
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

This document defines an extension to the BBS Signature scheme, a form of short group digital signature scheme that supports multi-message signing that produces a single output digital signature. To enable blind signing capabilities which in the most general sense provide the ability for a signer to blindly sign a set of messages.

{mainmatter}

# Introduction

// TODO

## Terminology

All terminology defined by [@BBS-DRAFT] is applicable to this draft

The following further terminology is defined by this document:

U
: The set of messages that are blinded from the signer during a blind signing.

K
: The set of messages that are known to the signer during a blind signing.

s'
: The signature blinding factor held by the signature recipient.

blind\_signature
: The blind digital signature output.

## Notation

Terms specific to pairing-friendly elliptic curves that are relevant to this document are restated in [@BBS-DRAFT], originally defined in [@!I-D.irtf-cfrg-pairing-friendly-curves]

# Conventions

The keywords **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**,
**SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL**, when they appear in this
document, are to be interpreted as described in [@!RFC2119].

# Overview

//TODO

# Example Flow

The example below illustrates the creation of a blind signature. Let the Signer have a public key PK = (w, h0, h[1],...,h[L]) where (h[1],...,h[L]) generators. The end result will be a signature to the messages (m[1],...,m[K]) (K less than L). The messages (m[1],...,m[U]) (U less than K), will be committed by the Client using the first U generators from the Signers PK (i.e., h[1],,,,h[U]). The messages (m[U+1],...,m[K]) will be known to the Signer and will be signed using the generators (h[U+1],...,h[K]) from their PK.

~~~ ascii-art
+--------+                               +--------+
|        | <-(1)------- nonce ---------- |        |
|        |                               |        |
| Client | --(2)- Commitment, nizk, U -> | Signer |
|        |                               |        |
|        | <-(3)--- Blind Signature ---- |        |
+--------+                               +--------+
~~~

1. The Signer and the Client will agree on a nonce to be used by the BlindMessagesProofGen and BlindMessagesProofVerify functions.

2. The Client will use the PreBlindSign function to calculate a Pedersen commitment for the messages (m[1],...,m[U]), using the generators (h[1],...,h[U]). Then they will create a proof of knowledge (nizk) for that commitment using BlindMessagesProofGen. The Signer will receive the commitment, the proof of knowledge (nizk) and the number of committed messages (U).

3. Before sending the blinded signature to the Client the Signer must execute the following steps,
    - Validate the proof of knowledge of the commitment using BlindMessagesProofVerify, on input the commitment, nizk, the nonce (from step 1) and the U first generators from their PK. Then check that the intersection between the generators used by the Client for the commitment, and the generators (h[U+1],...,h[K]), used by the Signer for the known messages, is indeed empty.
    - Create the blind signature using the BlindSign function. Note that the blinded signature is not a valid BBS signature.

    After the Client receives the blind signature they will use the UnblindSign function to unblinded it, getting a valid BBS signature on the messages (m[1],...,m[K]).

# Operations

## PreBlindSign

The PreBlindSign algorithm allows a holder of a signature to blind messages that when signed, are unknown to the signer.

The algorithm returns a generated blinding factor that is used to un-blind the signature from the signer, and a pedersen commitment from a vector of messages and the domain parameters h and h0.

```
(s', commitment) = PreBlindSign((msg[1],...,msg[U]), CGIdxs)

Inputs:

- msg\[1\],...,msg\[U\], octet strings of the messages to be blinded.
- CGIdxs, vector of unsigned integers. Indices of the generators from the domain parameter h, used for the messages to be blinded.

Outputs:

- s', octet string.
- commitment, octet string

Procedure:

1. (i1,...,iU) = CGIdxs

2. s' = HASH(PRF(8 \* ceil(log2(r)))) mod r

3. if subgroup\_check(h0) is INVALID abort

4. if (subgroup\_check(h\[i1\]) && ... && subgroup\_check(h\[iU\])) is INVALID abort

5. commitment = h0 \* s' + h\[i1\] \* msg\[1\] + ... + Ch\[iU\] \* msg\[U\]

6. return s', commitment
```

## BlindSign

BlindSign generates a blind signature from a commitment received from a holder, known messages, a secret key, the domain parameter h0 and generators from the domain parameter h. The signer also validates the commitment using the proof of knowledge of committed messages received from the holder and checks that the generators used in the commitment are not also used for the known messages.

```
blind_signature = BlindSign(commitment, (msg[1],...msg[K]), SK, GIdxs, CGIdxs, nizk, nonce)

Inputs:

- commitment, octet string receive from the holder in output form PreBlindSign.
- nizk, octet string received from the holder in output from BlindMessagesProofGen.
- msg\[1\],...,msg\[K\], octet strings.
- SK, a secret key output from KeyGen.
- GIdxs, vector of unsigned integers. Indices of the generators from the domain parameter h, used for the known messages.
- CGIdxs, vector of unsigned integers. Indices of the generators from the domain parameter h, used for the commited messages.
- nonce, octet string, suplied to the holder by the signer to be used with BlindMessagesProofGen.

Outputs:

- blind\_signature, octet string

Procedure:

1. (j1, ..., jK) = GIdxs

2. e = HASH(PRF(8 \* ceil(log2(r)))) mod r

3. s'' = HASH(PRF(8 \* ceil(log2(r)))) mod r

4. if BlindMessagesProofVerify(commitment, nizk, CGIdxs, nonce) is INVALID abort

5. if GIdxs intersection with CGIdxs is not empty abort

6. b = commitment + P1 + h0 \* s'' + h\[j1\] \* msg\[1\] + ... + h\[jK\] \* msg\[K\]

7. A = b \* (1 / (SK + e))

8. blind\_signature = (A, e, s'')

9. return blind\_signature
```

## UnblindSign

UnblindSign computes the unblinded signature given a blind signature and the holder's blinding factor. It is advised to verify the signature after un-blinding.

```
signature = UnblindSign(blind_signature, s')

Inputs:

- s', octet string in output form from PreBlindSign
- blind\_signature, octet string in output form from BlindSign

Outputs:

- signature, octet string

Procedure:

1. (A, e, s'') = blind\_signature

2. if subgroup\_check(A) is INVALID abort

3. if (subgroup\_check(blind\_signature)) is INVALID abort

4. s = s' + s''

5. signature = (A, e, s)

6. return signature
```

## BlindMessagesProofGen

BlindMessagesProofGen creates a proof of committed messages zero-knowledge proof. The proof should be verified before a signer computes a blind signature. The proof is created from a nonce given to the holder from the signer, a vector of messages, a blinding factor output from PreBlindSign, the domain parameter h0 and generators from the domain parameter h.

```
nizk = BlindMessagesProofGen(commitment, s', (msg[1],...,msg[U]), CGIdxs, nonce)

Inputs:

- commitment, octet string as output from PreBlindSign
- s', octet string as output from PreBlindSign
- msg\[1\],...,msg\[U\], octet strings of the blinded messages.
- CGIdxs, vector of unsigned integers. Indices of the generators from the domain parameter h, used for the commited messages.
- nonce, octet string.

Outputs:

- nizk, octet string

Procedure:

1. (i1,...,iU) = CGIdxs

2. r\~ = \[U\]

3. s\~ = HASH(PRF(8 \* ceil(log2(r)))) mod r

4. for i in 1 to U: r\~\[i\] = HASH(PRF(8 \* ceil(log2(r)))) mod r

5. U~ = h0 \* s\~ + h\[i1\] \* r\~\[1\] + ... + h\[iU\] \* r\~\[U\]

6. c = HASH(commitment || U\~ || nonce)

7. s^ = s\~ + c \* s'

8. for i in 1 to U: r^\[i\] = r\~\[i\] + c \* msg\[i\]

9. nizk = (c, s^, r^)
```

## BlindMessagesProofVerify

BlindMessagesProofVerify checks whether a proof of committed messages zero-knowledge proof is valid.

```
result = BlindMessagesProofVerify(commitment, nizk, CGIdxs, nonce)

Inputs:

- commitment, octet string in output form from PreBlindSign
- nizk, octet string in output form from BlindMessagesProofGen
- CGIdxs, vector of unsigned integers. Indices of the generators from the domain parameter h, used for the commited messages.
- nonce, octet string

Outputs:

- result, either VALID or INVALID.

Procedure:

1. (i1,...,iU) = CGIdxs

2. ( c, s^, r^ ) = nizk

3. U^ = commitment \* -c + h0 \* s^ + h\[i1\] \* r^\[1\] + ... + h\[iU\] \* r^\[U\]

4. c\_v = HASH(U || U^ || nonce)

5. return c == c\_v
```

# Security Considerations

Implementers should consider the security considerations of [@BBS-DRAFT] when implementing this work.

# IANA Considerations

This document does not make any requests of IANA.

{backmatter}

# Appendix

## Test Vectors

//TODO

<reference anchor="BBS-DRAFT" target="https://identity.foundation/bbs-signature/draft-bbs-signatures.html">
 <front>
   <title>The BBS Signature Scheme</title>
   <author initials="M." surname="Lodder" fullname="Mike Lodder">
      <organization>CryptID</organization>
    </author>
    <author initials="T." surname="Looker" fullname="Tobias Looker">
      <organization>MATTR</organization>
    </author>
    <author initials="A." surname="Whitehead" fullname="Andrew Whitehead">
    </author>
    <author initials="V." surname="Kalos" fullname="Vasileios Kalos">
      <organization>MATTR</organization>
    </author>
 </front>
</reference>
