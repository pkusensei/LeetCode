using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string ReplaceWords(IList<string> dictionary, string sentence)
    {
        Trie tr = new();
        foreach (var w in dictionary)
        {
            tr.Insert(w);
        }
        return string.Join(' ', sentence.Split(' ').Select(s => tr.Find(s)));
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
            foreach (var ch in s)
            {
                curr = curr.Nodes[ch - 'a'] ??= new();
            }
            curr.IsEnd = true;
        }

        public string Find(ReadOnlySpan<char> s)
        {
            var curr = this;
            for (int i = 0; i < s.Length; i++)
            {
                if (curr is null) { break; }
                if (curr.IsEnd) { return new(s[..i]); }
                curr = curr.Nodes[s[i] - 'a'];
            }
            return new(s);
        }
    }
}