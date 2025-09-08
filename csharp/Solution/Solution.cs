using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SubarraySum(int[] nums, int k)
    {
        Dictionary<long, int> freq = new() { [0] = 1 };
        long prefix = 0;
        int res = 0;
        foreach (var num in nums)
        {
            prefix += num;
            res += freq.GetValueOrDefault(prefix - k, 0);
            if (!freq.TryAdd(prefix, 1)) { freq[prefix] += 1; }
        }
        return res;
    }
}