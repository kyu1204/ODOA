from typing import List


class Solution:
    @staticmethod
    def two_sum(nums: List[int], target: int) -> List[int]:
        num_dict = {}
        for idx, num in enumerate(nums):
            if target - num in num_dict:
                return [num_dict[target - num], idx]
            num_dict[num] = idx
