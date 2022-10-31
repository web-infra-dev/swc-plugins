import { describe, expect, it } from "vitest"

describe('find binding', () => {
  it('dd', () => {
    const binding = require("../index.js")
    expect(binding.Compiler).toBeDefined()
    expect(binding.minify).toBeDefined()
    expect(binding.minifySync).toBeDefined()
  })
})
