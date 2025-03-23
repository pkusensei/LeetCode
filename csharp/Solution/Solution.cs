using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int LongestSquareStreak(int[] nums)
    {
        HashSet<int> set = [];
        Array.Sort(nums);
        int res = -1;
        foreach (var item in nums)
        {
            int num = item;
            if (!set.Add(num)) { continue; }
            int count = 0;
            while (Array.BinarySearch(nums, num) >= 0)
            {
                count += 1;
                set.Add(num);
                try { checked { num *= num; } }
                catch (OverflowException) { break; }
            }
            if (count > 1) { res = Math.Max(res, count); }
        }
        return res;
    }
}
