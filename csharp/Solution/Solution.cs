using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int BestRotation(int[] nums)
    {
        int n = nums.Length;
        int score = 0;
        int[] diff = new int[1 + n];
        for (int i = 0; i < n; i++)
        {
            int num = nums[i];
            if (num <= i)
            {
                score += 1;
                diff[1 + i] += 1;
                diff[1 + i - num] -= 1;
            }
            else
            {
                diff[1 + i] += 1;
                diff[1 + i + n - num] -= 1;
            }
        }
        int max = 0;
        int res = 0;
        for (int rotate = 0; rotate < n; rotate++)
        {
            score += diff[rotate];
            if (max < score)
            {
                res = rotate;
                max = score;
            }
        }
        return res;
    }
}
