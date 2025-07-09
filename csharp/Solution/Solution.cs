using System.Reflection.Metadata;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> CombinationSum2(int[] candidates, int target)
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
            for (int i = 0; i < nums.Length; i++)
            {
                if (i == 0 || nums[i] != nums[i - 1])
                {
                    curr.Add(nums[i]);
                    Dfs(nums[(1 + i)..], target - nums[i], curr);
                    curr.RemoveAt(curr.Count - 1);
                }
            }
        }
    }
}
