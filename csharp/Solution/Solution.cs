using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FindDisappearedNumbers(int[] nums)
    {
        int n = nums.Length;
        for (int i = 0; i < n; i++)
        { // mark all seen as negative
            int val = int.Abs(nums[i]);
            nums[val - 1] = -int.Abs(nums[val - 1]);
        }
        List<int> res = [];
        for (int i = 0; i < n; i++)
        { // positive ones left are missing numbers
            if (nums[i] > 0) { res.Add(1 + i); }
        }
        return res;
    }
}