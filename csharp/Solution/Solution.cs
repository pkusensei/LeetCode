using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LargestPathValue(string colors, int[][] edges)
    {
        int n = colors.Length;
        int[] degs = new int[n];
        List<int>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        foreach (var e in edges)
        {
            (int from, int to) = (e[0], e[1]);
            adj[to].Add(from);
            degs[from] += 1;
        }
        Queue<int> queue = [];
        int[,] dp = new int[n, 26];
        for (int i = 0; i < n; i++)
        {
            if (degs[i] == 0)
            {
                dp[i, colors[i] - 'a'] += 1;
                queue.Enqueue(i);
            }
        }
        if (queue.Count == 0) { return -1; }
        int res = 1;
        while (queue.TryDequeue(out var node))
        {
            foreach (var next in adj[node])
            {
                for (int c = 0; c < 26; c++)
                {
                    dp[next, c] = Math.Max(dp[next, c], dp[node, c]);
                }
                degs[next] -= 1;
                if (degs[next] == 0)
                {
                    int c = colors[next] - 'a';
                    dp[next, c] += 1;
                    res = Math.Max(res, dp[next, c]);
                    queue.Enqueue(next);
                }
            }
        }
        if (degs.Any(v => v > 0)) { return -1; }
        return res;
    }
}
