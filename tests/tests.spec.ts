import { expect, it } from "vitest";

import { sum, draw } from "../index.js";

it("should return non-zero", () => {
  expect(sum(1, 2)).toBe(3);
  draw('./image.png');
});
