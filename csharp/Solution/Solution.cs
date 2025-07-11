using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string MinWindow(string s, string t)
    {
        Span<int> freq = stackalloc int[52];
        foreach (var c in t)
        {
            Count(freq, c, -1);
        }
        ReadOnlySpan<char> res = [];
        int left = 0;
        for (int right = 0; right < s.Length; right++)
        {
            Count(freq, s[right], 1);
            while (Check(freq))
            {
                if (res.IsEmpty || res.Length > right + 1 - left) { res = s.AsSpan(left, right + 1 - left); }
                Count(freq, s[left], -1);
                left += 1;
            }
        }
        return new(res);

        static void Count(Span<int> freq, char c, int delta)
        {
            if (char.IsAsciiLetterUpper(c)) { freq[c - 'A'] += delta; }
            else { freq[c - 'a' + 26] += delta; }
        }

        static bool Check(Span<int> freq)
        {
            for (int i = 0; i < 52; i++)
            {
                if (freq[i] < 0) { return false; }
            }
            return true;
        }
    }
}