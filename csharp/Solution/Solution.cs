using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CanFinish(int n, int[][] prerequisites)
    {
        List<int>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        int[] degs = new int[n];
        foreach (var p in prerequisites)
        {
            degs[p[0]] += 1;
            adj[p[1]].Add(p[0]);
        }
        Queue<int> queue = [];
        for (int i = 0; i < n; i++)
        {
            if (degs[i] == 0) { queue.Enqueue(i); }
        }
        while (queue.TryDequeue(out var node))
        {
            foreach (var next in adj[node])
            {
                degs[next] -= 1;
                if (degs[next] == 0) { queue.Enqueue(next); }
            }
        }
        return degs.All(v => v == 0);
    }
}
