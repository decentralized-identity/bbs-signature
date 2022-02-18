import { promises } from "fs";
import * as path from "path";
import * as fixtures from "./fetchFixtures";
import get from "lodash.get";

const VARIABLE_REGEX = /^({{ \$)([a-zA-Z|.|\d|\[|\]]*)( }})$/gm;

const DRAFT_NAME = "../../draft-bbs-signatures.md";

const main = async () => {
  // Read the text of the draft out
  const filePath = path.join(process.env.PWD as string, DRAFT_NAME);
  let fileContents = (await promises.readFile(filePath)).toString();

  const results = Array.from(fileContents.matchAll(VARIABLE_REGEX)).map(
    (item) => {
      return { match: item[0], path: item[2] };
    }
  );

  results.forEach((result) => {
    console.log(result);
    const value = get(fixtures, result.path);

    if (value) {
      fileContents = fileContents.replace(result.match, value);
    }
  });

  // Write an updated copy of the file
  await promises.writeFile(filePath, fileContents);
};

main();
