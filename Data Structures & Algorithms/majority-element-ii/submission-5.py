
class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        nums.sort()
        
        result = set()
        for i in range(len(nums) - len(nums) // 3):
            if nums[i] == nums[i + len(nums) // 3]:
                result.add(nums[i])
                
        return list(result)