import { isEven, isOdd } from "is-odd-rs";

describe("isOdd", () => {
    it("should be true", () => {
        expect(isOdd(1)).toBe(true);
        expect(isOdd("1")).toBe(true);
        expect(isOdd(1.1)).toBe(false);
        expect(isOdd("1.1")).toBe(false);
    });
    it("should be false", () => {
        expect(isOdd(2)).toBe(false);
        expect(isOdd("2")).toBe(false);
        expect(isOdd(2.2)).toBe(false);
        expect(isOdd("2.2")).toBe(false);
    });
});

describe("isEven", () => {
    it("should be true", () => {
        expect(isEven(2)).toBe(true);
        expect(isEven("2")).toBe(true);
        expect(isEven(2.2)).toBe(false);
        expect(isEven("2.2")).toBe(false);
    });
    it("should be false", () => {
        expect(isEven(1)).toBe(false);
        expect(isEven("1")).toBe(false);
        expect(isEven(1.1)).toBe(false);
        expect(isEven("1.1")).toBe(false);
    });
});

