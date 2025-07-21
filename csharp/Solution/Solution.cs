using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string MakeFancyString(string s)
    {
        StringBuilder sb = new();
        foreach (var ch in s)
        {
            if (sb.Length >= 2 && sb[^1] == ch && sb[^2] == ch) { continue; }
            else { sb.Append(ch); }
        }
        return sb.ToString();
    }
}
