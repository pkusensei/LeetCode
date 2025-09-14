using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string[] Spellchecker(string[] wordlist, string[] queries)
    {
        HashSet<string> exact = [];
        Dictionary<string, string> capital = [];
        Dictionary<string, string> vowel = [];
        foreach (var item in wordlist)
        {
            exact.Add(item);
            capital.TryAdd(item.ToLower(), item);
            vowel.TryAdd(Process(item), item);
        }
        List<string> res = [];
        foreach (var q in queries)
        {
            if (exact.Contains(q)) { res.Add(q); }
            else if (capital.TryGetValue(q.ToLower(), out var v)) { res.Add(v); }
            else if (vowel.TryGetValue(Process(q), out v)) { res.Add(v); }
            else { res.Add(""); }
        }
        return [.. res];

        static string Process(ReadOnlySpan<char> s)
        {
            StringBuilder sb = new();
            foreach (var item in s)
            {
                char ch = char.ToLower(item);
                if ("aeiou".Contains(ch)) { sb.Append('_'); }
                else { sb.Append(ch); }
            }
            return sb.ToString();
        }
    }
}