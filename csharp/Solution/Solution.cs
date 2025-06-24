using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FindKDistantIndices(int[] nums, int key, int k)
    {
        HashSet<int> set = [];
        for (int i = 0; i < nums.Length; i++)
        {
            if (nums[i] == key)
            {
                for (int val = Math.Max(i - k, 0); val < Math.Min(i + k + 1, nums.Length); val++)
                {
                    set.Add(val);
                }
            }
        }
        return [.. set.Order()];
    }
}
