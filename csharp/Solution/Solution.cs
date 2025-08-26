using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> FindAllConcatenatedWordsInADict(string[] words)
    {
        Array.Sort(words, (a, b) => a.Length.CompareTo(b.Length));
        List<string> res = [];
        Trie tr = new();
        foreach (var s in words)
        {
            if (tr.Find(s)) { res.Add(s); }
            else { tr.Insert(s); }
        }
        return res;
    }
}

class Trie
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
        foreach (var c in s)
        {
            int idx = c - 'a';
            curr = curr.Nodes[idx] ??= new();
        }
        curr.IsEnd = true;
    }

    public bool Find(ReadOnlySpan<char> s)
    {
        var curr = this;
        for (int i = 0; i < s.Length; i++)
        {
            if (curr is null) { return false; }
            if (curr.IsEnd)
            {
                if (this.Find(s[i..])) { return true; }
            }
            curr = curr.Nodes[s[i] - 'a'];
        }
        bool res = curr is not null && curr.IsEnd;
        return res;
    }
}