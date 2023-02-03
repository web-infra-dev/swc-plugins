import { describe, test } from "vitest";
import { fsSnapshot, walkLeafDir } from "./utils";
import * as path from "path";
import { Compiler, TransformConfig } from "../";

async function transform(
  option: Partial<TransformConfig>,
  filename: string,
  code: string
) {
  const compiler = new Compiler(option);
  return await compiler.transformSync(filename, code);
}

describe("extensions", () => {
  test("plugin-import", async () => {
    await walkLeafDir(
      path.resolve(
        __dirname,
        "../crates/plugin_import/tests/fixtures/style-tpl"
      ),
      async (dir) => {
        await fsSnapshot(dir, transform);
      }
    );
  });

  test("plugin-emotion", async () => {
    const oldEnv = process.env.NODE_ENV;
    await walkLeafDir(
      path.resolve(__dirname, "./fixtures/emotion/bool"),
      async (dir) => {
        await fsSnapshot(dir, transform);
      }
    );

    process.env.NODE_ENV = "development";
    await walkLeafDir(
      path.resolve(__dirname, "./fixtures/emotion/dev"),
      async (dir) => {
        await fsSnapshot(dir, transform);
      }
    );

    process.env.NODE_ENV = "production";
    await walkLeafDir(
      path.resolve(__dirname, "./fixtures/emotion/prod"),
      async (dir) => {
        await fsSnapshot(dir, transform);
      }
    );

    process.env.NODE_ENV = oldEnv;
  });

  test("plugin-styled-components", async () => {
    await walkLeafDir(
      path.resolve(__dirname, "./fixtures/styled-components"),
      async (dir) => {
        await fsSnapshot(dir, transform);
      }
    );
  })
});
