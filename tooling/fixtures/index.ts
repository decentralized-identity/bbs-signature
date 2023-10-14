import { promises } from "fs";
import * as path from "path";
import * as fixtures from "./fetchFixtures";
import get from "lodash.get";

// matching lines of the form "name = {{ $<fixture_path> }}" (for
// example "m_1 = {{ $messages[1] }}" etc).
const VARIABLE_REGEX = /(([^\S\n\t]*[a-zA-Z0-9_]+\d*)\s=\s)?({{ \$)([a-zA-Z|.|\-|\d|\[|\]]*)( }})$/gm

const DRAFT_NAME = "../../draft-irtf-cfrg-bbs-signatures.md";

const main = async () => {
  // Read the text of the draft out
  const filePath = path.join(process.env.PWD as string, DRAFT_NAME);
  let fileContents = (await promises.readFile(filePath)).toString();

  const results = Array.from(fileContents.matchAll(VARIABLE_REGEX)).map(
    (item: any) => {
      return { match: "{{ $" + item[4] + " }}", path: item[4], intent: item[1] };
    }
  );

  results.forEach((result) => {
    var value = get(fixtures, result.path);

    // handle values that are arrays
    if (Array.isArray(value)) {
      let array_value = "[ ";
      for (let el of value.slice(0, -1)) {
        array_value = array_value + el + ", ";
      }
      array_value = array_value + value.slice(-1) + " ]";
      value = array_value;
    }

    value = "\x22" + value + "\x22";

    let intent_len = result.intent ? result.intent.length : 0;
    let max_len = 71 - intent_len;
    if (max_len <= 0) {throw Error("Not enough space in the line to add the fixture")}

    // make everything 72 chars long
    if (value.length + intent_len > 72) {
      value = value.slice(0, max_len + 1) + "\n" + " ".repeat(intent_len + 1) + value.slice(max_len + 1);
    }
                                                                        
    for (let i = 1; i < ~~(value.length/72); i++) {
      value = value.slice(0, 145 - intent_len + (i - 1)*73) + "\n" + " ".repeat(intent_len + 1) + value.slice(145 - intent_len + (i - 1)*73);
    }

    // remove trailing whitespace from the value to be added in the draft
    value = value.trim();

    if (value || value === '') {
      fileContents = fileContents.replace(result.match, value);
    }
  });

  // Write an updated copy of the file
  await promises.writeFile(filePath, fileContents);
};

main();
