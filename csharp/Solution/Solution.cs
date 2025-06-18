using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[][] DivideArray(int[] nums, int k)
    {
        Array.Sort(nums);
        List<int[]> res = [];
        foreach (var ch in nums.Chunk(3))
        {
            if (ch[2] - ch[0] <= k) { res.Add(ch); }
            else { return []; }
        }
        return [.. res];
    }
}
