using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string LongestWord(string[] words)
    {
        Array.Sort(words, (a, b) =>
        {
            if (a.Length == b.Length) { return a.CompareTo(b); }
            return a.Length.CompareTo(b.Length);
        });
        string res = "";
        Trie trie = new();
        foreach (var s in words)
        {
            if (trie.Insert(s) && s.Length > res.Length) { res = s; }
        }
        return res;
    }
}

internal sealed class Trie
{
    public Trie()
    {
        Nodes = new Trie[26];
    }

    Trie[] Nodes { get; }

    public bool Insert(ReadOnlySpan<char> s)
    {
        var curr = this;
        for (int i = 0; i < s.Length; i++)
        {
            int idx = s[i] - 'a';
            if (curr.Nodes[idx] is not null) { curr = curr.Nodes[idx]; }
            else if (i == s.Length - 1) { curr = curr.Nodes[idx] = new(); }
            else { return false; }
        }
        return true;
    }
}