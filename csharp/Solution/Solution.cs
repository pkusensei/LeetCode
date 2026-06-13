using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class StreamChecker
{
    readonly Trie trie;
    readonly List<char> storage;

    public StreamChecker(string[] words)
    {
        storage = new();
        trie = new();
        foreach (var s in words)
        {
            trie.Insert(s);
        }
    }

    public bool Query(char letter)
    {
        storage.Add(letter);
        return trie.Find(storage.AsEnumerable().Reverse());
    }
}

internal sealed class Trie
{
    public Trie()
    {
        Nodes = new Trie[26];
        IsEnd = false;
    }

    public Trie[] Nodes { get; private init; }
    public bool IsEnd { get; private set; }

    public void Insert(string s)
    {
        var node = this;
        foreach (char c in s.Reverse())
        {
            int i = c - 'a';
            if (node.Nodes[i] is null) { node.Nodes[i] = new(); }
            node = node.Nodes[i];
        }
        node.IsEnd = true;
    }

    public bool Find(IEnumerable<char> s)
    {
        var node = this;
        foreach (char c in s)
        {
            int i = c - 'a';
            if (node.Nodes[i] is null) { return false; }
            node = node.Nodes[i];
            if (node.IsEnd) { return true; }
        }
        return node.IsEnd;
    }
}
