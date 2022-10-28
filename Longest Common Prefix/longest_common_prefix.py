from typing import List, Optional


class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> Optional[str]:
        if len(strs) == 0:
            return ""
        else:
            pre = min(strs, key=len)
            low = 0
            high = len(pre)
            while low <= high:
                middle = int((low + high) / 2) + 1
                if self.middleCheck(strs, pre[:middle]):
                    low = middle + 1
                else:
                    high = middle - 1

            return pre[:int((low+high)/2)]

    def middleCheck(self, strs: List[str], pre: str) -> bool:
        for i in strs:
            if not i.startswith(pre):
                return False
        return True


if __name__ == "__main__":
    so = Solution().longestCommonPrefix(["teacher", "tesla", "test"])
    print(so)
