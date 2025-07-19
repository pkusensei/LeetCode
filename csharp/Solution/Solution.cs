using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> RemoveSubfolders(string[] folder)
    {
        Array.Sort(folder, (a, b) => a.Length.CompareTo(b.Length));
        List<string> res = [];
        Trie tr = new();
        foreach (var s in folder)
        {
            if (tr.Insert(s)) { res.Add(s); }
        }
        return res;
    }
}

internal class Trie
{
    public Trie()
    {
        Nodes = [];
        IsEnd = false;
    }

    public Dictionary<string, Trie> Nodes { get; }
    public bool IsEnd { get; private set; }

    public bool Insert(string s)
    {
        var curr = this;
        foreach (var seg in s.Split('/', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
        {
            if (curr.Nodes.TryGetValue(seg, out var node))
            {
                if (node.IsEnd) { return false; }
            }
            else
            {
                node = new();
                curr.Nodes.Add(seg, node);
            }
            curr = node;
        }
        curr.IsEnd = true;
        return true;
    }
}