using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindSmallestInteger(int[] nums, int value)
    {
        Dictionary<int, int> dict = Enumerable.Range(0, value).ToDictionary(v => v, v => v);
        foreach (var num in nums)
        {
            int v = num % value;
            if (v < 0) { v += value; }
            dict[v] += value;
        }
        return dict.Values.Min();
    }
}
