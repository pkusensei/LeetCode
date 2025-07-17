using System.Text;
using Solution.LList;
// using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> WordBreak(string s, IList<string> wordDict)
    {
        Trie tr = new();
        foreach (var item in wordDict)
        {
            tr.Insert(item);
        }
        List<List<int>> res = [];
        tr.Dfs(s, 0, [], res);
        return res.Select(v =>
        {
            int prev = 0;
            StringBuilder sb = new();
            foreach (var end in v)
            {
                sb.Append(s[prev..end]);
                sb.Append(' ');
                prev = end;
            }
            sb.Remove(sb.Length - 1, 1);
            return sb.ToString();
        }).ToList();
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

    public void Dfs(ReadOnlySpan<char> s, int idx, List<int> indices, List<List<int>> res)
    {
        if (idx == s.Length)
        {
            res.Add([.. indices]);
            return;
        }
        var curr = this;
        for (int i = idx; i < s.Length; i++)
        {
            curr = curr.Nodes[s[i] - 'a'];
            if (curr is null) { break; }
            if (curr.IsEnd)
            {
                indices.Add(1 + i);
                Dfs(s, 1 + i, indices, res);
                indices.RemoveAt(indices.Count - 1);
            }
        }
    }
}