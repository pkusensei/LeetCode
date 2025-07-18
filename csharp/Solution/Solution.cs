using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Kadanes(int[] nums)
    {
        if (nums.Length < 2) { return nums.FirstOrDefault(); }
        long res = nums[0];
        long min = nums[0];
        long max = nums[0];
        foreach (var num in nums[1..])
        {
            if (num == 0)
            {
                min = 1;
                max = 1;
                res = long.Max(res, 0);
            }
            else
            {
                long next_min = long.Min(num, long.Min(min * num, max * num));
                long next_max = long.Max(num, long.Max(min * num, max * num));
                (min, max) = (next_min, next_max);
                res = long.Max(res, max);
            }
        }
        return (int)res;
    }

    public int WithPrefSuf(int[] nums)
    {
        int n = nums.Length;
        if (n < 2) { return nums.FirstOrDefault(); }
        long res = nums[0];
        long prod_left = 1;
        long prod_right = 1;
        for (int i = 0; i < n; i++)
        {
            if (prod_left == 0) { prod_left = 1; }
            prod_left *= nums[i];
            if (prod_right == 0) { prod_right = 1; }
            prod_right *= nums[n - i - 1];
            res = long.Max(res, long.Max(prod_left, prod_right));
        }
        return (int)res;
    }
}