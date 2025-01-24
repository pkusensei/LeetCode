using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<int> EventualSafeNodes(int[][] graph)
    {
        var n = graph.Length;
        var outdegs = new int[n];
        var prevs = new List<int>[n];
        for (int i = 0; i < n; i++)
        {
            // This is annoying
            // Array.Fill() would copy the same ref to all indices
            prevs[i] = [];
        }
        var queue = new Queue<int>();
        foreach (var (i, item) in graph.Select((v, i) => (i, v)))
        {
            outdegs[i] = item.Length;
            foreach (var target in item) { prevs[target].Add(i); }
            if (item.Length == 0) { queue.Enqueue(i); }
        }
        List<int> res = [];
        while (queue.TryDequeue(out var node))
        {
            res.Add(node);
            foreach (var prev in prevs[node])
            {
                outdegs[prev] -= 1;
                if (outdegs[prev] == 0) { queue.Enqueue(prev); }
            }
        }
        res.Sort();
        return res;
    }
}
