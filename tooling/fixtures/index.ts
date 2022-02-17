import { promises } from "fs";
import * as path from "path";
import { signatureFixtures } from "./fetchFixtures";

const DRAFT_NAME = "draft-bbs-signatures.md";

const PRIVATE_KEY = "{{ $private_key }}";
const PUBLIC_KEY = "{{ $public_key }}";

const main = async () => {
  // Read the text of the draft out
  const filePath = path.join(process.env.PWD as string, DRAFT_NAME);
  let fileContents = (await promises.readFile(filePath)).toString();

  // All of the fixtures share the same key pair so reading the first one is reliable
  const keyPair = signatureFixtures[0].value.signerKeyPair;

  // Populate private key variable in the draft
  fileContents = fileContents.replaceAll(PRIVATE_KEY, keyPair.secretKey);

  // Populate public key variable in the draft
  fileContents = fileContents.replaceAll(PUBLIC_KEY, keyPair.publicKey);

  // Write an updated copy of the file
  await promises.writeFile(filePath, fileContents);
};

main();
