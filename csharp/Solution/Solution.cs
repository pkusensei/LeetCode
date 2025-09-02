using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> FindSubsequences(int[] nums)
    {
        List<IList<int>> res = [];
        Backtrack(0, 0, 0);
        return res;

        void Backtrack(int idx, int prev, int mask)
        {
            if (idx >= nums.Length)
            {
                if (int.PopCount(mask) >= 2)
                {
                    List<int> curr = new(int.PopCount(mask));
                    for (int bit = 0; bit < nums.Length; bit++)
                    {
                        if (((mask >> bit) & 1) == 1) { curr.Add(nums[bit]); }
                    }
                    res.Add(curr);
                }
                return;
            }
            if (mask == 0 || prev <= nums[idx])
            {
                Backtrack(1 + idx, nums[idx], mask | (1 << idx));
            }
            if (mask == 0 || prev != nums[idx])
            {
                Backtrack(1 + idx, prev, mask);
            }
        }
    }
}
