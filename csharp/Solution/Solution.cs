using System.Text;
using Solution.LList;
// using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool WordBreak(string s, IList<string> wordDict)
    {
        Trie tr = new();
        foreach (var item in wordDict)
        {
            tr.Insert(item);
        }
        byte[] memo = new byte[s.Length];
        return tr.Dfs(s, 0, memo);
    }
}

class Trie
{
    public Trie()
    {
        Nodes = new Trie[26];
        IsEnd = false;
    }

    public Trie[] Nodes { get; }
    public bool IsEnd { get; private set; }

    public void Insert(ReadOnlySpan<char> s)
    {
        var curr = this;
        for (int i = 0; i < s.Length; i++)
        {
            curr = curr.Nodes[s[i] - 'a'] ??= new();
        }
        curr.IsEnd = true;
    }

    public bool Dfs(ReadOnlySpan<char> s, int idx, Span<byte> memo)
    {
        if (idx == s.Length) { return true; }
        if (memo[idx] > 0) { return memo[idx] > 1; }
        var curr = this;
        bool res = false;
        for (int i = idx; i < s.Length; i++)
        {
            curr = curr.Nodes[s[i] - 'a'];
            if (curr is null) { break; }
            if (curr.IsEnd) { res |= Dfs(s, 1 + i, memo); }
        }
        memo[idx] = (byte)(res ? 2 : 1);
        return res;
    }
}