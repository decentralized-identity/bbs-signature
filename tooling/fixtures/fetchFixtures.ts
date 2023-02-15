import * as keyPair from "./fixture_data/keyPair.json";
import * as messages from "./fixture_data/messages.json";
import * as path from "path";
import { readdirSync  } from 'fs';

const FIXTURES_FILE = "./fixture_data"

const isObject = (value: unknown) => value && typeof value === "object";

// tslint:disable-next-line:no-var-requires
const resolveFixtures = (subDirectory: string, filter: any) =>
  require("require-all")({
    dirname: `${__dirname}/${subDirectory}`,
    filter: filter,
    excludeDirs: [".github", "tests"],
    map: (__: unknown, path: unknown) => {
      return `${path}`;
    },
  });

const suites = readdirSync(FIXTURES_FILE, { withFileTypes: true })
                .filter(dirent => dirent.isDirectory())
                .map(dirent => dirent.name);

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

export interface ProofFixtureData {
  readonly caseName: string;
  readonly signerPublicKey: string;
  readonly header: string;
  readonly presentationMessage: string;
  readonly revealedMessages: { [index: string]: string };
  readonly totalMessageCount: number;
  readonly proof: string;
  result: { valid: false; reason: string } | { valid: true };
}

export interface GeneratorFixtureData {
  readonly P1: string;
  readonly Q1: string;
  readonly Q2: string;
  readonly MsgGenerators: string[];
}

export interface H2sFixtureData {
  readonly caseName: string;
  readonly message: string;
  readonly dst: string;
  readonly count: number;
  readonly scalars: string[];
}

export interface MapMessageToScalarCase {
  message: string;
  scalar: string;
}

export interface MapMessageToScalarFixtureData {
  readonly caseName: string;
  readonly dst: string;
  readonly cases: ReadonlyArray<MapMessageToScalarCase>
}

export interface MockRngFixtureData {
  readonly mockedScalars: string[];
}

export interface Fixture<T> {
  readonly name: string
  readonly value: T
}

const fetchNestedFixtures = <T>(name: string, input: any): ReadonlyArray<Fixture<T>> => {
  if (input.caseName || input.MsgGenerators || input.mockedScalars) {
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


const fetchPerSuiteFixtures = <T>(dir:string, filter = /.json$/) => {
  let fixtureMap = {}
  for (let suite of suites) {
    let suiteFixturesData = fetchNestedFixtures<T>(
      "", resolveFixtures(FIXTURES_FILE+"/"+suite+dir, filter)
      )
      .reduce((map, item: Fixture<T>) => {
        map = {
          ...map,
          [item.name]: item.value
        }
        return map
      }, {})

    fixtureMap = {
      ...fixtureMap,
      [suite]: suiteFixturesData
    }
  }
  
  return fixtureMap
}

export const signatureFixtures = fetchPerSuiteFixtures<SignatureFixtureData>("/signature");
export const proofFixtures = fetchPerSuiteFixtures<ProofFixtureData>("/proof");
export const H2sFixture = fetchPerSuiteFixtures<H2sFixtureData>("/h2s")
export const generatorFixtures = fetchPerSuiteFixtures<GeneratorFixtureData>("", /generators.json/);
export const MapMessageToScalarFixtures = 
  fetchPerSuiteFixtures<MapMessageToScalarFixtureData>("", /MapMessageToScalarAsHash.json/);
export const MockRngFixtures = fetchPerSuiteFixtures<MockRngFixtureData>("", /mockedRng.json/);

console.log("MapMessageToScalarFixtures = ", MapMessageToScalarFixtures);
console.log("H2sFixture = ", H2sFixture);
console.log("MockRngFixtures = ", MockRngFixtures);

export { keyPair, messages };
