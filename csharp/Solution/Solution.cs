using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class MagicDictionary
{
    public MagicDictionary() => Root = new();

    Trie Root { get; }

    public void BuildDict(string[] dictionary)
    {
        foreach (var s in dictionary)
        {
            Root.Insert(s);
        }
    }

    public bool Search(string searchWord) => Root.Find(searchWord, false);
}

internal sealed class Trie
{
    public Trie()
    {
        Nodes = new Trie[26];
        IsEnd = false;
    }

    Trie[] Nodes { get; }
    bool IsEnd { get; set; }

    public void Insert(ReadOnlySpan<char> s)
    {
        var curr = this;
        foreach (var ch in s)
        {
            int idx = ch - 'a';
            curr = curr.Nodes[idx] ??= new();
        }
        curr.IsEnd = true;
    }

    public bool Find(ReadOnlySpan<char> s, bool alter)
    {
        if (s.IsEmpty) { return IsEnd && alter; }
        int idx = s[0] - 'a';
        bool res = false;
        for (int i = 0; i < 26; i++)
        {
            if (Nodes[i] is Trie node)
            {
                if (idx == i) { res |= node.Find(s[1..], alter); }
                else if (!alter) { res |= node.Find(s[1..], true); }
                if (res) { break; }
            }
        }
        return res;
    }
}