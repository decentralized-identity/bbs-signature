## Meeting - Monday 21st March 2022 - (6pm UTC)

### Chair

Tobias Looker

### Agenda

- IPR reminder, and Introductions
- Agenda bashing
- PR review
    - [BBS spk security against the issuer](https://github.com/decentralized-identity/bbs-signature/pull/86)
    - [Change usages of HASH function to use XOF](https://github.com/decentralized-identity/bbs-signature/pull/84)
    - [Add message mapping to scalar](https://github.com/decentralized-identity/bbs-signature/pull/61)
- Issue Review

### Attendees

- Christian Paquin (MSR)
- Seth Back (Trinsic)
- Jeremie Miller (Ping)
- Vasileios Kalos (MATTR)

### Notes

https://github.com/decentralized-identity/bbs-signature/pull/86
- Approved and merged

https://github.com/decentralized-identity/bbs-signature/pull/84
- Are there performance tradeoffs here by using XOF everywhere instead of other digest algorithms

https://github.com/decentralized-identity/bbs-signature/pull/88
- Resolved not to head in this direction, instead going to use hash_to_field in places throughout the draft where the input is not a message (e.g challenge)

https://github.com/decentralized-identity/bbs-signature/issues/48
- Resolved to close based on other upstream drafts filling this purpose
