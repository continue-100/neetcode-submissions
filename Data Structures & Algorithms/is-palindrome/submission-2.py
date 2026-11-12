class Solution:
    def isPalindrome(self, s: str) -> bool:
        l = list(filter(lambda x: x != " ",filter(str.isalnum, s.lower())))
        return l == list(reversed(l))