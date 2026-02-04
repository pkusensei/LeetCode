using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ShortestSubarray(int[] nums, int k)
    {
        int n = nums.Length;
        int res = 1 + n;
        PriorityQueue<(int i, long sum), long> pq = new();
        long curr = 0;
        for (int i = 0; i < n; i++)
        {
            curr += nums[i];
            if (curr >= k) { res = int.Min(res, 1 + i); }
            while (pq.TryPeek(out _, out long prio) && curr - prio >= k)
            {
                (int prev, _) = pq.Dequeue();
                res = int.Min(res, i - prev);
            }
            pq.Enqueue((i, curr), curr);
        }
        return res > n ? -1 : res;
    }

    public int WithDeque(int[] nums, int k)
    {
        int n = nums.Length;
        int res = 1 + n;
        LinkedList<(int i, long sum)> queue = new();
        long curr = 0;
        for (int i = 0; i < n; i++)
        {
            curr += nums[i];
            if (curr >= k) { res = int.Min(res, 1 + i); }
            while (queue.Last is not null && queue.Last.Value.sum >= curr)
            {
                queue.RemoveLast();
            }
            while (queue.First is not null && curr - queue.First.Value.sum >= k)
            {
                int prev = queue.First.Value.i;
                res = int.Min(res, i - prev);
                queue.RemoveFirst();
            }
            queue.AddLast((i, curr));
        }
        return res > n ? -1 : res;
    }
}