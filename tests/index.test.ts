import { describe, test } from "vitest";
import { fsSnapshot, walkLeafDir } from "./utils";
import * as path from "path";
import { Compiler, TransformConfig } from "../";

async function transform(option: Partial<TransformConfig>, filename: string, code: string) {
  const compiler = new Compiler(option);
  return await compiler.transformSync(filename, code);
}

describe("extensions", () => {
  test("plugin-import", async () => {
    await walkLeafDir(
      path.resolve(__dirname, "../crates/plugin_import/tests/fixtures/style-tpl"),
      async (dir) => {
        await fsSnapshot(dir, transform);
      }
    );
  });
});
