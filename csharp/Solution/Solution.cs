using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class MapSum
{
    public MapSum()
    {
        Root = new();
    }

    Trie Root { get; }

    public void Insert(string key, int val) => Root.Insert(key, val);

    public int Sum(string prefix) => Root.Sum(prefix);
}

internal sealed class Trie
{
    public Trie()
    {
        Nodes = new Trie[26];
        Val = 0;
    }

    Trie[] Nodes { get; }
    int Val { get; set; }

    public void Insert(ReadOnlySpan<char> s, int val)
    {
        var curr = this;
        foreach (var ch in s)
        {
            int idx = ch - 'a';
            curr = curr.Nodes[idx] ??= new();
        }
        curr.Val = val;
    }

    public int Sum(ReadOnlySpan<char> s)
    {
        if (s.IsEmpty)
        {
            int res = Val;
            foreach (var node in Nodes)
            {
                if (node is not null) { res += node.Sum([]); }
            }
            return res;
        }
        else
        {
            int idx = s[0] - 'a';
            return Nodes[idx] is Trie node ? node.Sum(s[1..]) : 0;
        }
    }
}