using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int FindTheLongestSubstring(string s)
    {
        Dictionary<int, int> dict = new() { { 0, 0 } };
        var mask = 0;
        var res = 0;
        foreach (var (idx, ch) in s.Select((v, i) => (i + 1, v)))
        {
            switch (ch)
            {
                case 'a': mask ^= 1 << 0; break;
                case 'e': mask ^= 1 << 1; break;
                case 'i': mask ^= 1 << 2; break;
                case 'o': mask ^= 1 << 3; break;
                case 'u': mask ^= 1 << 4; break;
                default: break;
            }
            if (dict.TryGetValue(mask, out var left))
            {
                res = Math.Max(res, idx - left);
            }
            else
            {
                dict.Add(mask, idx);
            }
        }
        return res;
    }
}