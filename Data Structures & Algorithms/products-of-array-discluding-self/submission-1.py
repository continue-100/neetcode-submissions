class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        prefixes = [1]
        ctr = 1
        for i in nums:
            ctr *= i
            prefixes.append(ctr)

        prefixes_rev = [1]
        ctr = 1
        for i in reversed(nums):
            ctr *= i
            prefixes_rev.append(ctr)
        
        out = []
        for i in range(len(nums)):
            out.append(prefixes_rev[-(i + 2)] * prefixes[i])
        return out
