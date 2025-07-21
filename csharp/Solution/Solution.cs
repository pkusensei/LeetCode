using System.ComponentModel.Design;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class WordDictionary
{
    public WordDictionary()
    {
        Tr = new();
    }

    public Trie Tr { get; }

    public void AddWord(string word) => Tr.Add(word);

    public bool Search(string word) => Tr.Check(word);
}

public class Trie
{
    public Trie()
    {
        Nodes = new Trie[26];
        IsEnd = false;
    }

    public Trie[] Nodes { get; }
    public bool IsEnd { get; private set; }

    public void Add(ReadOnlySpan<char> s)
    {
        var curr = this;
        foreach (var ch in s)
        {
            curr = curr.Nodes[ch - 'a'] ??= new();
        }
        curr.IsEnd = true;
    }

    public bool Check(ReadOnlySpan<char> s)
    {
        if (s.IsEmpty) { return IsEnd; }
        if (s[0] == '.')
        {
            foreach (var node in Nodes)
            {
                if (node is not null && node.Check(s[1..])) { return true; }
            }
        }
        else
        {
            var node = Nodes[s[0] - 'a'];
            if (node is not null) { return node.Check(s[1..]); }
        }
        return false;
    }
}
