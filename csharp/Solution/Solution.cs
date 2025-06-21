using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumDeletions(string word, int k)
    {
        Span<int> _freq = stackalloc int[26];
        foreach (var item in word)
        {
            _freq[item - 'a'] += 1;
        }
        List<int> freq = [];
        for (int i = 0; i < 26; i++)
        {
            if (_freq[i] > 0) { freq.Add(_freq[i]); }
        }
        freq.Sort();
        int res = 0; // resulting length
        for (int i = 0; i < freq.Count; i++)
        {
            int curr = 0;
            foreach (var f in freq[i..])
            {
                curr += Math.Min(freq[i] + k, f);
            }
            res = Math.Max(res, curr);
        }
        return word.Length - res;
    }
}
