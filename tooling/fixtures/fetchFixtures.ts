import * as keyPair from "./keyPair.json";
import * as generators from "./generators.json";
import * as path from "path";

const isObject = (value: unknown) => value && typeof value === "object";

// tslint:disable-next-line:no-var-requires
const resolveFixtures = (subDirectory: string) =>
  require("require-all")({
    dirname: `${__dirname}/${subDirectory}`,
    filter: /.json$/,
    excludeDirs: [".github", "tests"],
    map: (__: unknown, path: unknown) => {
      return `${path}`;
    },
  });

export interface SignatureFixtureData {
  readonly caseName: string;
  readonly signature: string;
  readonly header: string;
  readonly messages: string[];
  result: { valid: false; reason: string } | { valid: true };
  readonly signerKeyPair: {
    readonly publicKey: string;
    readonly secretKey: string;
  };
}

export interface SignatureFixture {
  readonly name: string;
  readonly value: SignatureFixtureData;
}

const fetchNestedFixtures = <T>(name: string, input: any): ReadonlyArray<T> => {
  if (input.caseName) {
    return [
      {
        name: path.basename(name).split(".")[0] as string,
        value: input,
      } as any,
    ];
  }
  if (!isObject(input)) {
    return [];
  }

  const extractedFixtures = Object.keys(input).map((key) =>
    fetchNestedFixtures(key, input[key])
  );
  return Array.prototype.concat.apply([], extractedFixtures);
};

export const signatureFixtures = fetchNestedFixtures<SignatureFixture>(
  "",
  resolveFixtures("signature")
).reduce((map, item) => {
  map = {
    ...map,
    [item.name]: item.value,
  };
  return map;
}, {});

export { keyPair, generators };
