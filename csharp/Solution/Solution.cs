using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int TakeCharacters(string s, int k)
    {
        var count = new int[3];
        foreach (var c in s) { count[c - 'a'] += 1; }
        if (count.Any(v => v < k)) { return -1; }
        var best = new int[3];
        for (int i = 0; i < 3; i++) { best[i] = count[i] - k; }
        var window = new int[3];
        int left = 0;
        int res = 0;
        foreach (var (right, ch) in s.Select((c, i) => (i, c)))
        {
            window[ch - 'a'] += 1;
            if (window.Zip(best).All(p => p.First <= p.Second))
            {
                res = Math.Max(res, right + 1 - left);
            }
            while (window.Zip(best).Any(p => p.First > p.Second))
            {
                window[s[left] - 'a'] -= 1;
                left += 1;
            }
        }
        return s.Length - res;
    }
}
