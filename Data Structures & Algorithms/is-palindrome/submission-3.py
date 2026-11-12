class Solution:
    def isPalindrome(self, s: str) -> bool:
        l = [c for c in s.lower() if c.isalnum()]
        return l == l[::-1]
