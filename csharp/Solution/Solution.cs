using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindMaxPathScore(int[][] edges, bool[] online, long k)
    {
        int n = online.Length;
        List<(int next, int cost)>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<(int, int)>())];
        int right = 0;
        foreach (var item in edges)
        {
            if (online[item[0]] && online[item[1]])
            {
                adj[item[0]].Add((item[1], item[2]));
                right = int.Max(right, item[2]);
            }
        }
        int left = -1;
        while (left < right)
        {
            int mid = left + (1 + right - left) / 2;
            if (Check(mid)) { left = mid; }
            else { right = mid - 1; }
        }
        return left;

        bool Check(int mid)
        {
            PriorityQueue<int, long> pq = new();
            pq.Enqueue(0, 0);
            long[] dists = [.. Enumerable.Range(0, n).Select(_ => long.MaxValue >> 1)];
            dists[0] = 0;
            while (pq.TryDequeue(out var node, out long dist))
            {
                if (node == n - 1) { break; }
                if (dist > dists[node]) { continue; }
                foreach (var (next, cost) in adj[node])
                {
                    long ncost = cost + dist;
                    if (cost >= mid && ncost < dists[next])
                    {
                        dists[next] = ncost;
                        pq.Enqueue(next, ncost);
                    }
                }
            }
            return dists[^1] <= k;
        }
    }
}
