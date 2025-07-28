using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountMaxOrSubsets(int[] nums)
    {
        int n = nums.Length;
        int total = nums.Aggregate((acc, v) => acc | v);
        int res = 0;
        for (int mask = 1; mask < 1 << n; mask++)
        {
            int curr = 0;
            for (int i = 0; i < n; i++)
            {
                if (((mask >> i) & 1) == 1) { curr |= nums[i]; }
            }
            if (curr == total) { res += 1; }
        }
        return res;
    }
}