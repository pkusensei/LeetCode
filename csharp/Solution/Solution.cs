using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
public int[] SortItems(int n, int m, int[] group, IList<IList<int>> beforeItems)
{
    int[] indegs = new int[n];
    List<int>[] adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
    for (int i = 0; i < n; i++)
    {
        indegs[i] += beforeItems[i].Count;
        foreach (var item in beforeItems[i])
        {
            adj[item].Add(i);
        }
    }
    List<int> sorted_nodes = TopoSort(adj, indegs);
    if (sorted_nodes.Count < n) { return []; }
    List<List<int>> group_list = [.. Enumerable.Range(0, m).Select(_ => new List<int>())];
    foreach (var item in sorted_nodes)
    {
        if (group[item] >= 0) { group_list[group[item]].Add(item); }
        else
        {
            group[item] = group_list.Count;
            group_list.Add([item]);
        }
    }
    List<int>[] gadj = [.. Enumerable.Range(0, group_list.Count).Select(_ => new List<int>())];
    int[] gindegs = new int[group_list.Count];
    for (int i = 0; i < n; i++)
    {
        int to = group[i];
        foreach (var item in beforeItems[i])
        {
            int from = group[item];
            if (from != to)
            {
                gindegs[to] += 1;
                gadj[from].Add(to);
            }
        }
    }
    List<int> sorted_groups = TopoSort(gadj, gindegs);
    if (sorted_groups.Count < gindegs.Length) { return []; }
    return [.. sorted_groups.SelectMany(i => group_list[i])];

    static List<int> TopoSort(List<int>[] adj, int[] indegs)
    {
        int n = indegs.Length;
        List<int> res = new(n);
        Queue<int> queue = new();
        for (int i = 0; i < n; i++)
        {
            if (indegs[i] == 0) { queue.Enqueue(i); }
        }
        while (queue.TryDequeue(out int node))
        {
            res.Add(node);
            foreach (var item in adj[node])
            {
                indegs[item] -= 1;
                if (indegs[item] == 0) { queue.Enqueue(item); }
            }
        }
        return res;
    }
}
}