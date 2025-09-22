using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxFrequencyElements(int[] nums)
    {
        Dictionary<int, int> freq = nums.CountBy(x => x).ToDictionary(p => p.Key, p => p.Value);
        int max = freq.Values.Max();
        return freq.Values.Where(v => v == max).Sum();
    }
}