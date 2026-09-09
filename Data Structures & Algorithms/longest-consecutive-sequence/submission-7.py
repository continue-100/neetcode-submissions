class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        nums.sort()
        l = m = 1
        for i in range(len(nums) - 1):
            m = max(l, m)
            if nums[i] + 1 == nums[i + 1]:
                l += 1
            elif nums[i] == nums[i + 1]:
                continue
            else:
                l = 1
        m = max(l, m)
        return 0 if nums == [] else m