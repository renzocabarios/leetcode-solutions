import { containsDuplicate } from "../index";
import { describe, expect, test } from "@jest/globals";

describe("testing index file", () => {
  test("1", () => {
    const nums: number[] = [1, 2, 3, 1];
    expect(containsDuplicate(nums)).toBe(true);
  });
  test("2", () => {
    const nums: number[] = [1, 2, 3, 4];
    expect(containsDuplicate(nums)).toBe(false);
  });
  test("3", () => {
    const nums: number[] = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    expect(containsDuplicate(nums)).toBe(true);
  });
});
