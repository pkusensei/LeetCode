using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindMaximizedCapital(int k, int w, int[] profits, int[] capital)
    {
        int n = profits.Length;
        (int pro, int cap)[] vals = [.. profits.Zip(capital)];
        Array.Sort(vals, (a, b) => a.cap.CompareTo(b.cap));
        PriorityQueue<int, int> pq = new();
        int idx = 0;
        for (int _ = 0; _ < k; _++)
        {
            for (; idx < n && vals[idx].cap <= w; idx += 1)
            {
                pq.Enqueue(vals[idx].pro, -vals[idx].pro);
            }
            if (pq.TryDequeue(out var p, out var _p))
            {
                w += p;
            }
        }
        return w;
    }
}