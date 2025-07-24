using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] ProductExceptSelf(int[] nums)
    {
        int n = nums.Length;
        int[] res = new int[n];
        int left = 1; // product from left to right
        for (int i = 0; i < n; i++)
        {
            res[i] = left;
            left *= nums[i];
        }
        int right = 1; // product from right to left
        for (int i = n - 1; i >= 0; i -= 1)
        {
            res[i] *= right;
            right *= nums[i];
        }
        return res;
    }
}