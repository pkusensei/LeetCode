using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinZeroArray(int[] nums, int[][] queries)
    {
        int n = nums.Length;
        int[] diff = new int[1 + n];
        int left = 0;
        int right = queries.Length;
        if (!Check(right)) { return -1; }
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (Check(mid)) { right = mid; }
            else { left = 1 + mid; }
        }
        return left;

        bool Check(int count)
        {
            Array.Fill(diff, 0);
            foreach (var q in queries.Take(count))
            {
                diff[q[0]] += q[2];
                diff[1 + q[1]] -= q[2];
            }
            int prefix = 0;
            foreach (var item in nums.Zip(diff))
            {
                (var num, var d) = item;
                prefix += d;
                if (prefix < num) { return false; }
            }
            return true;
        }
    }
}
