using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long CountInterestingSubarrays(IList<int> nums, int modulo, int k)
    {
        List<int> prefix = [.. nums.Select(v => v % modulo == k ? 1 : 0)];
        for (int i = 1; i < nums.Count; i++)
        {
            prefix[i] += prefix[i - 1];
        }
        Dictionary<int, int> dict = new() { { 0, 1 } };
        long res = 0;
        foreach (var item in prefix)
        {
            if (dict.TryGetValue((item + modulo - k) % modulo, out var v)) { res += v; }
            int key = item % modulo;
            if (!dict.TryAdd(key, 1)) { dict[key] += 1; }
        }
        return res;
    }
}
