using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int CountPaths(int n, int[][] roads)
    {
        List<(int node, long cost)>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<(int, long)>())];
        foreach (var item in roads)
        {
            int a = item[0];
            int b = item[1];
            int time = item[2];
            adj[a].Add((b, time));
            adj[b].Add((a, time));
        }
        Span<int> dp = stackalloc int[n];
        dp[0] = 1;
        Span<long> costs = stackalloc long[n];
        costs.Fill(long.MaxValue);
        PriorityQueue<int, long> pq = new();
        pq.Enqueue(0, 0);
        while (pq.TryDequeue(out var curr, out var cost))
        {
            if (cost > costs[curr]) { continue; }
            foreach (var next in adj[curr])
            {
                long nc = cost + next.cost;
                if (nc < costs[next.node])
                {
                    dp[next.node] = dp[curr];
                    costs[next.node] = nc;
                    pq.Enqueue(next.node, nc);
                }
                else if (nc == costs[next.node])
                {
                    dp[next.node] += dp[curr];
                    dp[next.node] %= 1_000_000_007;
                }
            }
        }
        return dp[n - 1];
    }
}
