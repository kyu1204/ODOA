from typing import List, Optional


class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> Optional[str]:
        if len(strs) == 0:
            return ""
        else:
            pre = strs[0]
            for i in strs:
                while not i.startswith(pre):
                    pre = pre[:-1]

            return pre


if __name__ == "__main__":
    so = Solution().longestCommonPrefix(["teacher", "tesla", "test"])
    print(so)
