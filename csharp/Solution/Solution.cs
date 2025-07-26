using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsAdditiveNumber(string num)
    {
        if (num.Length < 3) { return false; }
        int max_len = (num.Length + 1) / 2;
        return Backtrack(num, -1, -1, 0);

        bool Backtrack(ReadOnlySpan<char> s, long v1, long v2, int count)
        {
            if (s.IsEmpty) { return count >= 3; }
            for (int i = 0; i < int.Min(max_len, s.Length); i++)
            {
                if (i > 0 && s[0] == '0') { break; }
                long curr = long.Parse(s[..(1 + i)]);
                if (v1 >= 0 && v2 >= 0)
                {
                    if (curr == v1 + v2 && Backtrack(s[(1 + i)..], v2, curr, 1 + count))
                    {
                        return true;
                    }
                }
                else if (Backtrack(s[(1 + i)..], v2, curr, 1 + count)) { return true; }
            }
            return false;
        }
    }
}