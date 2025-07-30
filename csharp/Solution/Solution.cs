using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> FindItinerary(IList<IList<string>> tickets)
    {
        Dictionary<string, List<string>> adj = [];
        foreach (var t in tickets)
        {
            if (!adj.TryAdd(t[0], [t[1]])) { adj[t[0]].Add(t[1]); }
        }
        foreach (var v in adj.Values)
        {
            v.Sort((a, b) => b.CompareTo(a));
        }
        var lookup = adj.GetAlternateLookup<ReadOnlySpan<char>>();
        List<string> res = [];
        Dfs("JFK");
        res.Reverse();
        return res;

        void Dfs(ReadOnlySpan<char> curr)
        {
            while (lookup.TryGetValue(curr, out var tos) && tos.Count > 0)
            {
                var next = tos[^1];
                tos.RemoveAt(tos.Count - 1);
                Dfs(next);
            }
            res.Add(new(curr));
        }
    }
}
