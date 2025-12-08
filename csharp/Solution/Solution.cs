using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> EventualSafeNodes(int[][] graph)
    {
        int n = graph.Length;
        int[] outdegs = new int[n];
        List<int>[] incoming = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        for (int i = 0; i < n; i++)
        {
            outdegs[i] = graph[i].Length;
            foreach (var node in graph[i])
            {
                incoming[node].Add(i);
            }
        }
        Queue<int> queue = [];
        List<int> res = [];
        for (int i = 0; i < n; i++)
        {
            if (outdegs[i] == 0)
            {
                res.Add(i);
                queue.Enqueue(i);
            }
        }
        while (queue.TryDequeue(out int node))
        {
            foreach (var item in incoming[node])
            {
                outdegs[item] -= 1;
                if (outdegs[item] == 0)
                {
                    res.Add(item);
                    queue.Enqueue(item);
                }
            }
        }
        res.Sort();
        return res;
    }
}
