## Meeting - Monday 27th June 2022 - (6pm UTC)

### Chair

Tobias Looker

### Agenda

- IPR reminder, and Introductions
- Agenda bashing
- Other items
    - Reminder IETF submission for CFRG due start of july.
- PR review
    - [Update KeyGen procedure to use hash_to_scalar](https://github.com/decentralized-identity/bbs-signature/pull/186)
    - [Editorial updates](https://github.com/decentralized-identity/bbs-signature/pull/187)
    - [Add the revealed messages to the challenge](https://github.com/decentralized-identity/bbs-signature/pull/188)
    - [encode for hash operation](https://github.com/decentralized-identity/bbs-signature/pull/190)
    - [Minor update to terminology](https://github.com/decentralized-identity/bbs-signature/pull/191)
    - [Consistency updates](https://github.com/decentralized-identity/bbs-signature/pull/192)
- Issue Review
    - [Defintions of P1 and P2 in Ciphersuites](https://github.com/decentralized-identity/bbs-signature/issues/164)
    - [Integer endianness](https://github.com/decentralized-identity/bbs-signature/issues/157)

### Attendees

- Andrew Whitehead
- Vasilis Kalos
- Christian Paquin

### Notes

- Merged PRs #187, #188, #191
- Closed Issue #74 as completed
- Reviewed open PR's and Issues
- For practical reasons, the spec no longer has concrete definitions for some variables (like the output length of the PRF etc.). Will address those in the operation descriptions (as examples) and or in the test vectors section (similar to the [H2C spec](https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-16.html#name-bls12-381-g1-2)).
