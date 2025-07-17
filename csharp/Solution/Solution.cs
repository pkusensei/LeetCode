using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Candy(int[] ratings)
    {
        int n = ratings.Length;
        int[] nums = new int[n];
        Array.Fill(nums, 1);
        for (int i = 1; i < n; i++)
        {
            if (ratings[i] > ratings[i - 1]) { nums[i] = int.Max(nums[i], 1 + nums[i - 1]); }
        }
        for (int i = n - 2; i >= 0; i -= 1)
        {
            if (ratings[i] > ratings[1 + i]) { nums[i] = int.Max(nums[i], 1 + nums[1 + i]); }
        }
        return nums.Sum();
    }
}