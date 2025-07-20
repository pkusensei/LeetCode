using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<string>> DeleteDuplicateFolder(IList<IList<string>> paths)
    {
        Trie tr = new("");
        foreach (var p in paths)
        {
            tr.Add(p);
        }
        Dictionary<string, Trie> seen = [];
        tr.Mark(seen);
        List<IList<string>> res = [];
        foreach (var v in tr.Nodes.Values)
        {
            v.Build([], res);
        }
        return res;
    }
}

internal class Trie
{
    public string Name { get; }
    public SortedList<string, Trie> Nodes { get; }
    public bool Del { get; set; }

    public Trie(string name)
    {
        Name = name;
        Nodes = [];
        Del = false;
    }

    public void Add(IList<string> paths)
    {
        var curr = this;
        foreach (var p in paths)
        {
            if (!curr.Nodes.TryGetValue(p, out var node))
            {
                node = new(p);
                curr.Nodes.Add(p, node);
            }
            curr = node;
        }
    }

    public string Mark(IDictionary<string, Trie> seen)
    {
        StringBuilder sb = new();
        foreach (var v in Nodes.Values)
        {
            sb.Append(v.Mark(seen));
        }
        var s = sb.ToString();
        if (s.Length > 0)
        {
            if (seen.TryGetValue(s, out var node))
            {
                node.Del = true;
                Del = true;
            }
            else
            {
                seen.Add(s, this);
            }
        }
        return $"({Name}{s})";
    }

    public void Build(List<string> curr, List<IList<string>> res)
    {
        if (Del) { return; }
        curr.Add(Name);
        res.Add([.. curr]);
        foreach (var v in Nodes.Values)
        {
            v.Build(curr, res);
        }
        curr.RemoveAt(curr.Count - 1);
    }
}