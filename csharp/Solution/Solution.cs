using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FindMinHeightTrees(int n, int[][] edges)
    {
        if (n <= 2) { return [.. Enumerable.Range(0, n)]; }
        int[] degs = new int[n];
        List<int>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        foreach (var e in edges)
        {
            adj[e[0]].Add(e[1]);
            adj[e[1]].Add(e[0]);
            degs[e[0]] += 1;
            degs[e[1]] += 1;
        }
        Queue<int> queue = [];
        for (int i = 0; i < n; i++)
        {
            if (degs[i] == 1) { queue.Enqueue(i); }
        }
        List<int> res = [];
        while (queue.Count > 0)
        {
            res.Clear();
            int count = queue.Count;
            for (int _ = 0; _ < count; _++)
            {
                int node = queue.Dequeue();
                res.Add(node);
                foreach (var next in adj[node])
                {
                    degs[next] -= 1;
                    if (degs[next] == 1) { queue.Enqueue(next); }
                }
            }
        }
        return res;
    }
}