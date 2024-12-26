using System.Collections.Immutable;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool IsPossibleDivide(int[] nums, int k)
    {
        if (nums.Length % k > 0) { return false; }
        SortedDictionary<int, int> count = [];
        foreach (var item in nums)
        {
            if (count.ContainsKey(item)) { count[item] += 1; }
            else { count[item] = 1; }
        }
        while (count.Count > 0)
        {
            var first = count.Keys.First();
            for (int i = 0; i < k; i++)
            {
                if (count.TryGetValue(first + i, out var val))
                {
                    val -= 1;
                    if (val == 0) { count.Remove(first + i); }
                    else { count[first + i] = val; }
                }
                else { return false; }
            }
        }
        return true;
    }
}