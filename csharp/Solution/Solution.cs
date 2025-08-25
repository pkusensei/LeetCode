using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindSubstringInWraproundString(string s)
    {
        if (s.Length < 2) { return s.Length; }
        Span<int> lens = stackalloc int[26];
        int left = 0;
        for (int right = 1; right < s.Length; right++)
        {
            int curr_start = s[left] - 'a';
            int a = s[right - 1] - 'a';
            int b = s[right] - 'a';
            if ((1 + a) % 26 == b)
            {
                lens[curr_start] = int.Max(lens[curr_start], right - left + 1);
            }
            else
            {
                while (left < right)
                {
                    lens[curr_start] = int.Max(lens[curr_start], right - left);
                    left += 1;
                    curr_start = s[left] - 'a';
                }
                left = right;
            }
        }
        while (left < s.Length)
        {
            lens[s[left] - 'a'] = int.Max(lens[s[left] - 'a'], s.Length - left);
            left += 1;
        }
        int res = 0;
        foreach (var v in lens)
        {
            res += v;
        }
        return res;
    }
}