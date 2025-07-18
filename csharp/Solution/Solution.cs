using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MinimumDifference(int[] nums)
    {
        int n = nums.Length / 3;
        PriorityQueue<int, int> pq = new();
        long sum = 0;
        long[] left_min = new long[1 + n];
        for (int i = 0; i < 2 * n; i++)
        {
            sum += nums[i];
            pq.Enqueue(nums[i], -nums[i]); // pop max
            if (i >= n)
            {
                int top = pq.Dequeue();
                sum -= top;
            }
            if (i >= n - 1) { left_min[i - n + 1] = sum; }
        }
        pq.Clear();
        sum = 0;
        long[] right_max = new long[1 + n];
        for (int i = 3 * n - 1; i >= n; i -= 1)
        {
            sum += nums[i];
            pq.Enqueue(nums[i], nums[i]); // pop min
            if (i < 2 * n)
            {
                int top = pq.Dequeue();
                sum -= top;
            }
            if (i <= 2 * n) { right_max[i - n] = sum; }
        }
        return left_min.Zip(right_max).Min(p => p.First - p.Second);
    }
}