import * as fs from "fs";
import * as R from "ramda";

const NUMBER_PATTERN = /^[0-9]+$/;

let file = fs.readFileSync("../input.txt").toString();
let lines = file.split("\n");

type MyDirectory = {
  name: string;
  children: MyResource[];
  size: number | null;
  type: "directory";
};

type MyFile = {
  name: string;
  size: number;
  type: "file";
};

type MyResource = MyDirectory | MyFile;

let createDirectory = (name: string): MyDirectory => ({
  name,
  children: [],
  size: null,
  type: "directory",
});

let createFile = (name: string, size: number): MyFile => ({
  name,
  size,
  type: "file",
});

let rootDirectory = createDirectory("/");

let path = [rootDirectory];

let findDirectoryByName = (
  files: MyResource[],
  name: string
): MyDirectory | undefined => {
  let matcher = R.whereEq({ name, type: "directory" });
  return R.find(matcher, files) as MyDirectory;
};

// Ignore first line: "$ cd /"
lines.shift();

// Build directory structure
lines.forEach((line) => {
  let words = line.split(" ");
  let currentDirectory = R.last(path);

  if (!currentDirectory) throw new Error("Empty path");

  if (words[0] === "dir") {
    currentDirectory.children.push(createDirectory(words[1]));
  } else if (words[0].match(NUMBER_PATTERN)) {
    let size = parseInt(words[0]);
    currentDirectory.children.push(createFile(words[1], size));
  } else if (words[1] === "cd") {
    let filename = words[2];
    if (filename === "..") {
      path.pop();
    } else {
      let newDirectory = findDirectoryByName(
        currentDirectory.children,
        filename
      );

      if (!newDirectory)
        throw new Error(`Could not find directory: ${newDirectory}`);

      path.push(newDirectory);
    }
  }
});

// Compute directory sizes
function calculateSize(directory: MyDirectory): number {
  directory.size = R.sum(
    directory.children.map((resource) =>
      resource.type === "directory" ? calculateSize(resource) : resource.size
    )
  );

  return directory.size;
}

calculateSize(rootDirectory);

// Part 1

const MAX_SIZE = 100000;

function getDirectories(directory: MyDirectory): MyDirectory[] {
  let childDirectories = directory.children.filter<MyDirectory>(
    (resource): resource is MyDirectory => resource.type === "directory"
  );

  return R.flatten([directory, ...R.map(getDirectories, childDirectories)]);
}

const partOne = R.pipe<
  [MyDirectory],
  MyDirectory[],
  number[],
  number[],
  number
>(
  getDirectories,
  R.map<MyDirectory, number>(R.propOr(0, "size")),
  R.filter(R.gt(MAX_SIZE)),
  R.sum
)(rootDirectory);

// Part 2

const TOTAL_SPACE = 70000000;
const SPACE_REQUIRED = 30000000;

const needToClearSize =
  (rootDirectory.size || 0) - TOTAL_SPACE + SPACE_REQUIRED;

const partTwo = R.pipe<
  [MyDirectory],
  MyDirectory[],
  number[],
  number[],
  number[],
  number
>(
  getDirectories,
  R.map<MyDirectory, number>(R.propOr(0, "size")),
  R.filter(R.lte(needToClearSize)),
  R.sort(R.subtract),
  R.head
)(rootDirectory);

// ---

console.log({ partOne, partTwo });
