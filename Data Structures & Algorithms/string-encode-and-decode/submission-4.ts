class Solution {
    /**
     * @param {string[]} strs
     * @returns {string}
     */
    encode(strs: string[]): string {
        if (strs.length === 0) {
            return "NEETCODE_SOLUTION/novec";
        } else if (strs.length === 1) {
            return `NEETCODE_SOLUTION/onestr/${strs[0]}`;
        } else {
            let s = String.fromCharCode(strs.length);
            for (const i of strs) {
                s += String.fromCharCode(i.length);
                s += i;
            }
            return s;
        }
    }

    /**
     * @param {string} str
     * @returns {string[]}
     */
    decode(str: string): string[] {
        if (str === "NEETCODE_SOLUTION/novec") {
            return [];
        } else if (str.startsWith("NEETCODE_SOLUTION/onestr/")) {
            return [str.slice(25)];
        }

        let idx = 0;
        if (idx >= str.length) {
            throw new Error("Empty s");
        }
        const finLen = str.charCodeAt(idx++);
        const v: string[] = [];

        for (let i = 0; i < finLen; i++) {
            if (idx >= str.length) {
                throw new Error(`Didn't find string ${i}`);
            }
            const len = str.charCodeAt(idx++);

            let e = "";
            for (let j = 0; j < len; j++) {
                if (idx >= str.length) {
                    throw new Error("Incorrect len");
                }
                e += str[idx++];
            }
            v.push(e);
        }
        return v;
    }
}
