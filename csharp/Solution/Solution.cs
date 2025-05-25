using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestPalindrome(string[] words)
    {
        Dictionary<char, int> same_dict = [];
        Dictionary<(char, char), (int, int)> diff_dict = [];
        foreach (var w in words)
        {
            if (w[0] == w[1] && !same_dict.TryAdd(w[0], 1)) { same_dict[w[0]] += 1; }
            else if (diff_dict.TryGetValue((w[1], w[0]), out var v)) { diff_dict[(w[1], w[0])] = (v.Item1, 1 + v.Item2); }
            else if (!diff_dict.TryAdd((w[0], w[1]), (1, 0)))
            {
                var val = diff_dict[(w[0], w[1])];
                diff_dict[(w[0], w[1])] = (1 + val.Item1, val.Item2);
            }
        }
        int res = diff_dict.Values.Select(v => 2 * Math.Min(v.Item1, v.Item2)).Sum();
        res += same_dict.Values.Sum();
        int odd = same_dict.Values.Where(v => (v & 1) == 1).Count();
        if (odd > 0) { res -= odd - 1; }
        return 2 * res;
    }
}
