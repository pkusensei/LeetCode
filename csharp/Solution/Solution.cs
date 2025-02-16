using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<IList<string>> DeleteDuplicateFolder(IList<IList<string>> paths)
    {
        Trie root = new("");
        foreach (var item in paths) { root.Insert(item); }
        Dictionary<string, Trie> seen = [];
        root.Dedup(seen);
        List<string> path = [];
        List<IList<string>> res = [];
        foreach (var v in root.Nodes.Values) { v.Build(path, res); }
        return res;
    }
}

public class Trie
{
    public string Name { get; }
    public SortedDictionary<string, Trie> Nodes { get; }
    public bool Del { get; set; }

    public Trie(string name)
    {
        Name = name;
        Nodes = [];
        Del = false;
    }

    public void Insert(IList<string> paths)
    {
        var curr = this;
        foreach (var name in paths)
        {
            curr.Nodes.TryAdd(name, new(name));
            curr = curr.Nodes[name];
        }
    }

    public string Dedup(IDictionary<string, Trie> seen)
    {
        StringBuilder sb = new();
        foreach (var v in Nodes.Values) { sb.Append(v.Dedup(seen)); }
        var key = sb.ToString();
        if (sb.Length > 0)
        {
            if (seen.TryGetValue(key, out var node))
            {
                node.Del = true;
                Del = true;
            }
            else
            {
                seen.Add(key, this);
            }
        }
        return $"({Name}{key})";
    }

    public void Build(List<string> path, List<IList<string>> res)
    {
        if (Del) { return; }
        path.Add(Name);
        res.Add(path[..]);
        foreach (var v in Nodes.Values) { v.Build(path, res); }
        path.RemoveAt(path.Count - 1);
    }
}
