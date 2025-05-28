using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxRemoval(int[] nums, int[][] queries)
    {
        Array.Sort(queries, (a, b) =>
        {
            if (a[0] == b[0]) { return a[1].CompareTo(b[1]); }
            return a[0].CompareTo(b[0]);
        });
        int qidx = 0;
        PriorityQueue<int, int> candids = new();
        PriorityQueue<int, int> curr = new();
        for (int i = 0; i < nums.Length; i += 1)
        {
            while (qidx < queries.Length && queries[qidx][0] <= i)
            {
                int end = queries[qidx][1];
                candids.Enqueue(end, -end);
                qidx += 1;
            }
            while (curr.TryPeek(out int top, out _) && top < i)
            {
                curr.Dequeue();
            }
            while (curr.Count < nums[i])
            {
                if (!candids.TryDequeue(out int top, out _) || top < i) { return -1; }
                curr.Enqueue(top, top);
            }
        }
        return candids.Count;
    }
}
