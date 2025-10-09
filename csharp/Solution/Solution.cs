using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool ValidPalindrome(string s)
    {
        int left = 0;
        int right = s.Length - 1;
        while (left < right)
        {
            if (s[left] == s[right])
            {
                left += 1;
                right -= 1;
            }
            else
            {
                return Check(s.AsSpan()[left..right])
                    || Check(s.AsSpan()[(1 + left)..(1 + right)]);
            }
        }
        return true;

        static bool Check(ReadOnlySpan<char> s)
        {
            int n = s.Length;
            if (n <= 1) { return true; }
            for (int i = 0; i < n / 2; i++)
            {
                if (s[i] != s[n - i - 1]) { return false; }
            }
            return true;
        }
    }
}