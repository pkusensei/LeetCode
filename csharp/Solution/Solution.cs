using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Trap(int[] height)
    {
        int n = height.Length;
        int[] left = new int[n];
        int curr = height[0];
        for (int i = 1; i < n; i++)
        {
            curr = Math.Max(curr, height[i]);
            left[i] = curr;
        }
        int[] right = new int[n];
        curr = height.Last();
        for (int i = n - 2; i >= 0; i -= 1)
        {
            curr = Math.Max(curr, height[i]);
            right[i] = curr;
        }
        int res = 0;
        for (int i = 0; i < n; i++)
        {
            int val = Math.Min(left[i], right[i]) - height[i];
            res += Math.Max(val, 0);
        }
        return res;
    }
}
