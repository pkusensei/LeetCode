using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> Combine(int n, int k)
    {
        List<IList<int>> res = [];
        for (int mask = 1; mask < (1 << n); mask++)
        {
            if (BitCount(mask) == k)
            {
                List<int> curr = new(k);
                for (int i = 0; i < n; i++)
                {
                    if (((mask >> i) & 1) == 1) { curr.Add(1 + i); }
                }
                res.Add(curr);
            }
        }
        return res;

        static int BitCount(int v)
        {
            int res = 0;
            while (v > 0)
            {
                res += 1;
                v &= v - 1;
            }
            return res;
        }
    }

    public IList<IList<int>> Subsets(int[] nums)
    {
        List<IList<int>> res = [];
        for (int mask = 0; mask < (1 << nums.Length); mask++)
        {
            List<int> curr = [];
            for (int i = 0; i < nums.Length; i++)
            {
                if (((mask >> i) & 1) == 1) { curr.Add(nums[i]); }
            }
            res.Add(curr);
        }
        return res;
    }
}