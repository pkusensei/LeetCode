using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinOperations(int[][] grid, int x)
    {
        List<int> nums = [.. grid.SelectMany(row => row)];
        nums.Sort();
        int med = nums[nums.Count / 2];
        int res = 0;
        foreach (var item in nums)
        {
            int diff = Math.Abs(item - med);
            if (diff % x == 0) { res += diff / x; }
            else{ return -1; }
        }
        return res;
    }
}
