using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MovesToMakeZigzag(int[] nums)
    {
        int n = nums.Length;
        int a = 0;
        int b = 0;
        for (int i = 0; i < n; i++)
        {
            int left = i > 0 ? nums[i - 1] : 1001;
            int right = 1 + i < n ? nums[1 + i] : 1001;
            int curr = nums[i] - (int.Min(left, right) - 1);
            curr = int.Max(curr, 0);
            if ((i & 1) == 0) { a += curr; } else { b += curr; }
        }
        return int.Min(a, b);
    }
}
