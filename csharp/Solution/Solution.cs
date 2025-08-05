using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxRotateFunction(int[] nums)
    {
        int n = nums.Length;
        int sum = 0;
        int curr = 0;
        for (int i = 0; i < n; i++)
        {
            sum += nums[i];
            curr += i * nums[i];
        }
        int res = curr;
        for (int i = 1; i < n; i++)
        {
            curr -= nums[^i] * n;
            curr += sum;
            res = int.Max(res, curr);
        }
        return res;
    }
}
