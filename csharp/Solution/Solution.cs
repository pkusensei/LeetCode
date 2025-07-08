using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FindSubstring(string s, string[] words)
    {
        int word_len = words[0].Length;
        int window_len = words.Length * word_len;
        if (s.Length < window_len) { return []; }
        Dictionary<string, int> _freq = [];
        var freq = _freq.GetAlternateLookup<ReadOnlySpan<char>>();
        foreach (var item in words)
        {
            if (!freq.TryAdd(item, 1)) { freq[item] += 1; }
        }
        List<int> res = [];
        for (int i = 0; i < word_len; i++)
        {
            Dictionary<string, int> _window = [];
            var window = _window.GetAlternateLookup<ReadOnlySpan<char>>();
            int left = i;
            int count = 0;
            for (int right = i; right + word_len <= s.Length; right += word_len)
            {
                var word = s.AsSpan(right, word_len);
                if (freq.ContainsKey(word))
                {
                    if (!window.TryAdd(word, 1)) { window[word] += 1; }
                    count += 1;
                    while (window[word] > freq[word])
                    {
                        var left_span = s.AsSpan(left, word_len);
                        window[left_span] -= 1;
                        left += word_len;
                        count -= 1;
                    }
                    if (count == words.Length) { res.Add(left); }
                }
                else
                {
                    _window.Clear();
                    count = 0;
                    left = right + word_len;
                }
            }
        }
        return res;
    }
}
