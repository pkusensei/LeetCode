using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double[] CalcEquation(IList<IList<string>> equations, double[] values, IList<IList<string>> queries)
    {
        Dictionary<string, int> str_ids = [];
        foreach (var s in equations.SelectMany(e => e))
        {
            str_ids.TryAdd(s, str_ids.Count);
        }
        int n = str_ids.Count;
        List<(int node, double w)>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<(int node, double w)>())];
        foreach (var (eq, v) in equations.Zip(values))
        {
            int n1 = str_ids[eq[0]];
            int n2 = str_ids[eq[1]];
            adj[n1].Add((n2, v));
            adj[n2].Add((n1, 1 / v));
        }
        List<double> res = new(queries.Count);
        foreach (var q in queries)
        {
            if (!str_ids.TryGetValue(q[0], out int n1) || !str_ids.TryGetValue(q[1], out int n2))
            {
                res.Add(-1.0);
                continue;
            }
            res.Add(Bfs(n1, n2));
        }
        return [.. res];

        double Bfs(int n1, int n2)
        {
            Queue<(int, double)> queue = [];
            queue.Enqueue((n1, 1.0));
            Span<bool> seen = stackalloc bool[n];
            seen[n1] = true;
            while (queue.TryDequeue(out var item))
            {
                (int node, double w) = item;
                if (node == n2) { return w; }
                foreach (var (next, w1) in adj[node])
                {
                    if (!seen[next])
                    {
                        seen[next] = true;
                        queue.Enqueue((next, w * w1));
                    }
                }
            }
            return -1.0;
        }
    }
}
