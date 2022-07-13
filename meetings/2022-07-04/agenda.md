## Meeting - Monday 4th July 2022 - (6pm UTC)

### Chair

Tobias Looker

### Agenda

- IPR reminder, and Introductions
- Agenda bashing
- Other items
- PR review:
    - [ Update KeyGen procedure to use hash_to_scalar #186 ](https://github.com/decentralized-identity/bbs-signature/pull/186)
- Issue review:
    - [ expand_message domain separation #194 ](https://github.com/decentralized-identity/bbs-signature/issues/194)
    - [ Handling subgroup checks #179 ](https://github.com/decentralized-identity/bbs-signature/issues/179)
    - [ H2C using SHA256 #143 ](https://github.com/decentralized-identity/bbs-signature/issues/143)

### Attendees

- Andrew Whitehead
- Vasilis Kalos

### Notes

- Discussed and closed issues prior to the submision to the CFRG.
- Closed issues:
    -  [Update security considerations #196](https://github.com/decentralized-identity/bbs-signature/issues/196)
    - [Consider splitting operations into Core and Higher level definitions #131](https://github.com/decentralized-identity/bbs-signature/issues/131)
    - [ Add an IsValidPoint operation #126 ](https://github.com/decentralized-identity/bbs-signature/issues/126)
    - [ Consider making "messages" and therefore "message generators" optional to all operation APIs #117 ](https://github.com/decentralized-identity/bbs-signature/issues/117)
    - [ API Update #159 ](https://github.com/decentralized-identity/bbs-signature/issues/159)
    - [ Elements to be hashed update #185 ](https://github.com/decentralized-identity/bbs-signature/issues/185)
- Merged PRs:
    - [ editorial updates #195 ](https://github.com/decentralized-identity/bbs-signature/pull/195)
    - [ encode for hash operation #190 ](https://github.com/decentralized-identity/bbs-signature/pull/190)
- Will leave Issue #143 open until we decide if we will define a sha256-based suite.
- Discussed Issue #179. The agreed direction is for the spec to assume that `octet_to_point_g*` will return VALID. Will keep open until this assumption is made explicit.