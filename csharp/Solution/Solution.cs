using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<bool> PrefixesDivBy5(int[] nums)
    {
        int curr = 0;
        List<bool> res = new(nums.Length);
        foreach (var num in nums)
        {
            curr = (curr << 1) | num;
            curr %= 5;
            res.Add(curr == 0);
        }
        return res;
    }
}

