using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int ShortestSubarray(int[] nums, int k)
    {
        var prefix = new long[1 + nums.Length];
        foreach (var (idx, num) in nums.Select((n, i) => (i, n)))
        {
            prefix[1 + idx] = prefix[idx] + num;
        }
        List<(int idx, long num)> queue = [];
        var res = 1 + nums.Length;
        foreach (var (idx, sum) in prefix.Select((n, i) => (i, n)))
        {
            while (queue.Count > 0 && queue.Last().num >= sum)
            {
                queue.RemoveAt(queue.Count - 1);
            }
            while (queue.Count > 0 && queue.First().num + k <= sum)
            {
                res = Math.Min(res, idx - queue.First().idx);
                queue.RemoveAt(0);
            }
            queue.Add((idx, sum));
        }
        return res < 1 + nums.Length ? res : -1;
    }

    public int WithPriorityQueue(int[] nums, int k)
    {
        var res = 1 + nums.Length;
        // element => int idx
        // priority => long sum 
        PriorityQueue<int, long> pq = new();
        long sum = 0;
        foreach (var (idx, num) in nums.Select((n, i) => (i, n)))
        {
            sum += num;
            if (sum >= k) { res = Math.Min(res, 1 + idx); }

            while (pq.TryPeek(out int i, out long peek) && sum - peek >= k)
            {
                pq.Dequeue();
                res = Math.Min(res, idx - i);
            }
            pq.Enqueue(idx, sum);
        }
        return res < 1 + nums.Length ? res : -1;
    }

    public int WithMonotonicStack(int[] nums, int k)
    {
        List<(long num, int idx)> stack = [(0, -1)];
        long sum = 0;
        var res = int.MaxValue;
        foreach (var (idx, num) in nums.Select((n, i) => (i, n)))
        {
            sum += num;
            while (stack.Count > 0 && stack.Last().num >= sum)
            {
                stack.RemoveAt(stack.Count - 1);
            }
            stack.Add((sum, idx));
            var candidate = BinarySearch(sum - k);
            if (candidate >= 0)
            {
                res = Math.Min(res, idx - stack[candidate].idx);
            }
        }
        return res < 1 + nums.Length ? res : -1;

        int BinarySearch(long target)
        {
            int left = 0;
            int right = stack.Count - 1;
            while (left <= right)
            {
                var mid = left + (right - left) / 2;
                if (stack[mid].num <= target)
                {
                    left = 1 + mid;
                }
                else
                {
                    right = mid - 1;
                }
            }
            return right;
        }
    }
}
