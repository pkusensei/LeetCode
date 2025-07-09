using System.Reflection.Metadata;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> CombinationSum(int[] candidates, int target)
    {
        Array.Sort(candidates);
        int n = candidates.Length;
        List<IList<int>> res = [];
        Dfs(candidates, target, []);
        return res;

        void Dfs(Span<int> nums, int target, List<int> curr)
        {
            if (target == 0)
            {
                res.Add([.. curr]);
                return;
            }
            if (nums.IsEmpty) { return; }
            if (nums[0] > target) { return; }
            Dfs(nums[1..], target, curr);
            curr.Add(nums[0]);
            Dfs(nums, target - nums[0], curr);
            curr.RemoveAt(curr.Count - 1);
        }
    }
}
