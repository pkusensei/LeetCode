using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NextGreaterElement(int n)
    {
        var s = n.ToString().ToCharArray();
        if (NextPerm(s) && int.TryParse(s, out var v)) { return v; }
        return -1;

        static bool NextPerm(Span<char> s)
        {
            int n = s.Length;
            int left = n - 2;
            bool found = false;
            for (; left >= 0; left -= 1)
            {
                if (s[left] < s[1 + left])
                {
                    found = true;
                    break;
                }
            }
            if (!found) { return false; }
            int right = n - 1;
            for (; left < right && s[left] >= s[right]; right -= 1) { }
            (s[left], s[right]) = (s[right], s[left]);
            s[(1 + left)..].Reverse();
            return true;
        }
    }
}
