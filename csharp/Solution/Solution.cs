using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsValidSerialization(string preorder)
    {
        string[] s = preorder.Split(',');
        return Preorder(s, out var v) && v.IsEmpty;

        bool Preorder(ReadOnlySpan<string> s, out ReadOnlySpan<string> next)
        {
            if (s.IsEmpty)
            {
                next = [];
                return false;
            }
            if (s[0] == "#")
            {
                next = s[1..];
                return true;
            }
            var res = Preorder(s[1..], out var right);
            res = Preorder(right, out next);
            return res;
        }
    }
}
