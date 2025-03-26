using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinCapability(int[] nums, int k)
    {
        int left = nums.Min();
        int right = nums.Max();
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (Count(mid) >= k) { right = mid; }
            else { left = mid + 1; }
        }
        return left;

        int Count(int mid)
        {
            int res = 0;
            int prev = -1;
            for (int i = 0; i < nums.Length; i++)
            {
                if (nums[i] <= mid && (i - prev > 1 || prev == -1))
                {
                    res += 1;
                    prev = i;
                }
            }
            return res;
        }
    }
}
