using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountCompleteSubarrays(int[] nums)
    {
        int unique = nums.Distinct().Count();
        int res = 0;
        int left = 0;
        Dictionary<int, int> dict = [];
        for (int right = 0; right < nums.Length; right++)
        {
            if (!dict.TryAdd(nums[right], 1)) { dict[nums[right]] += 1; }
            while (dict.Count == unique)
            {
                res += nums.Length - right;
                dict[nums[left]] -= 1;
                if (dict[nums[left]] == 0) { dict.Remove(nums[left]); }
                left += 1;
            }
        }
        return res;
    }
}
