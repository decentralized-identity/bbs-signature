## Meeting - Monday 11th July 2022 - (6pm UTC)

### Chair

Tobias Looker

### Agenda

- IPR reminder, and Introductions
- Agenda bashing
- PR review
    - [ Update KeyGen procedure to use hash_to_scalar #186 ](https://github.com/decentralized-identity/bbs-signature/pull/186)
- Issue Review:
    - [ H2C using SHA256 #143 ](https://github.com/decentralized-identity/bbs-signature/issues/143)
    - [ expand_message domain separation #194 ](https://github.com/decentralized-identity/bbs-signature/issues/194)

### Attendees

- Vasilis Kalos
- Mike Lodder

### Notes

- Discussed open PRs and Issues.
- Discussed the suitability of hash-to-scalar as an alternative of the HKDF based KeyGen operation. It was agreed that hash-to-scalar is most likely a good replacement for HKDF KeyGen.
- Discussed alternative to hash-to-scalar that does not depend to exapnd_message from the hash to curve spec, and more specifically the approach from section 5 of [[CDMP07]](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.365.1590&rep=rep1&type=pdf).
- Similarly, discussed alternative to hash-to-curve for creating generators and more specifically the approach from section 3.3 of [[BLS01]](https://link.springer.com/content/pdf/10.1007/3-540-45682-1_30.pdf).
