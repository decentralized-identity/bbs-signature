# BBS Signature Specification

[Latest Draft](https://mattrglobal.github.io/bbs-signatures-spec/)

This repository is home to the "BBS Signature" work item of the Applied Cryptography Working group, who's goal is
to describe the short group signature scheme of BBS in a manner suitable for standardization.

## Contributing

The main specification is written in the markdown, however to preview the changes you have made in the final format, the following steps can be followed.

The tool `markdown2rfc` is used to convert the raw markdown representation to both an HTML and XML format. In order to run this tool you must have [docker](https://www.docker.com/) installed.

### Updating Docs

Update `spec.md` file with your desired changes.

Run the following to compile the new txt into the output HTML.

```./scripts/build-html.sh```