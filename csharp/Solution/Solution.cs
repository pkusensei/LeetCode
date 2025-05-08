using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumDiameterAfterMerge(int[][] edges1, int[][] edges2)
    {
        var tree1 = Build(edges1);
        var tree2 = Build(edges2);
        int d1 = TopoSort(tree1);
        int d2 = TopoSort(tree2);
        int half1 = d1 / 2 + (d1 & 1);
        int half2 = d2 / 2 + (d2 & 1);
        return Math.Max(1 + half1 + half2, Math.Max(d1, d2));

        static int TopoSort(List<List<int>> tree)
        {
            int nodes = tree.Count;
            int[] degs = new int[nodes];
            Queue<int> queue = [];
            for (int i = 0; i < nodes; i++)
            {
                degs[i] = tree[i].Count;
                if (degs[i] == 1) { queue.Enqueue(i); }
            }
            int layers = 0;
            while (nodes > 2)
            {
                int sz = queue.Count;
                nodes -= sz;
                layers += 1;
                for (int _ = 0; _ < sz; _++)
                {
                    int front = queue.Dequeue();
                    foreach (var item in tree[front])
                    {
                        degs[item] -= 1;
                        if (degs[item] == 1) { queue.Enqueue(item); }
                    }
                }
            }
            return 2 * layers + nodes / 2;
        }

        static List<List<int>> Build(int[][] edges)
        {
            int n = 1 + edges.Length;
            List<List<int>> res = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
            foreach (var e in edges)
            {
                res[e[0]].Add(e[1]);
                res[e[1]].Add(e[0]);
            }
            return res;
        }
    }
}
