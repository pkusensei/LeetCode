using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool PredictTheWinner(int[] nums)
    {
        return Dfs(nums, 0, 0, false);

        static bool Dfs(ReadOnlySpan<int> nums, int p1, int p2, bool turn)
        {
            if (nums is []) { return p1 >= p2; }
            if (nums is [int v])
            {
                if (!turn) { return Dfs([], p1 + v, p2, !turn); }
                else { return Dfs([], p1, p2 + v, !turn); }
            }
            if (!turn)
            {
                bool a = Dfs(nums[1..], p1 + nums[0], p2, !turn);
                bool b = Dfs(nums[..^1], p1 + nums[^1], p2, !turn);
                return a || b;
            }
            else
            {
                bool a = Dfs(nums[1..], p1, p2 + nums[0], !turn);
                bool b = Dfs(nums[..^1], p1, p2 + nums[^1], !turn);
                return a && b;
            }
        }
    }
}