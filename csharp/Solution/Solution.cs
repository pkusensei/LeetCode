using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] GetMaximumXor(int[] nums, int maximumBit)
    {
        List<int> prefix = [];
        foreach (var item in nums)
        {
            prefix.Add(item ^ prefix.LastOrDefault());
        }
        var max = (1 << maximumBit) - 1;
        var res = prefix.Select(p => p ^ max).ToArray();
        Array.Reverse(res);
        return res;
    }
}
