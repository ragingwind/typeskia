import { expect, it } from "vitest";

import { sum } from "../index.js";

it("should return non-zero", () => {
  expect(sum(1, 2)).toBe(3);
});
