using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindPairs(int[] nums, int k)
    {
        Dictionary<int, int> freq = nums.GroupBy(x => x).ToDictionary(g => g.Key, g => g.Count());
        int res = 0;
        foreach (var key in freq.Keys)
        {
            if (freq.TryGetValue(key + k, out var f) && (k > 0 || f > 1))
            { // Special case k==0&&f>1
                res += 1;
            }
        }
        return res;
    }
}
