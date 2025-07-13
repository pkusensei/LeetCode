using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> SubsetsWithDup(int[] nums)
    {
        Array.Sort(nums);
        List<IList<int>> res = [];
        Dfs(nums, []);
        return res;

        void Dfs(ReadOnlySpan<int> nums, List<int> curr)
        {
            if (nums.IsEmpty)
            {
                res.Add([.. curr]);
                return;
            }
            curr.Add(nums[0]);
            Dfs(nums[1..], curr);
            curr.RemoveAt(curr.Count - 1);
            int i = 1;
            while (i < nums.Length && nums[i] == nums[0]) { i += 1; }
            Dfs(nums[i..], curr);
        }
    }
}