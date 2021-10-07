# BBS Signature Specification

[Latest Draft](https://identity.foundation/bbs-signature/)

This repository is home to the "BBS Signature" work item of the DIF [Applied Cryptography Working Group](https://identity.foundation/working-groups/crypto.html). The goal of this work item is to describe the short group signature scheme of BBS in a manner suitable for standardization.

## Contributing

The main specification is written in Markdown, however to preview the changes you have made in the final format, the following steps can be followed.

The tool `markdown2rfc` is used to convert the raw Markdown representation to both HTML and XML formats. In order to run this tool you must have [Docker](https://www.docker.com/) installed.

### Updating Docs

Update `spec.md` file with your desired changes.

Run the following to compile the new txt into the output HTML.

```sh
./scripts/build-html.sh
```

### Meetings

Regular meetings are held bi-weekly on Mondays, on the same weeks as the Applied Crypto Working Group call is held.

- [Meeting details](https://calendar.google.com/calendar/event?eid=NXJ2Z29jaGJwcTlraXZnbGNxOHZudWc4YXRfMjAyMTEwMDRUMTgwMDAwWiBkZWNlbnRyYWxpemVkLmlkZW50aXR5QG0)
- [Direct Zoom link](https://us02web.zoom.us/j/87409761657?pwd=SXVSUGtVQXUyYzdxbnVvQkNJcXdGQT09)

Meeting agendas and minutes can be found in [/meetings](https://github.com/decentralized-identity/bbs-signature/tree/main/meetings).
