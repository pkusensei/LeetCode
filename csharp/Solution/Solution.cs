using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string SortVowels(string s)
    {
        char[] V = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        char[] vows = [.. s.Where(c => V.Contains(c)).Order()];
        int i = 0;
        StringBuilder sb = new();
        foreach (var c in s)
        {
            if (V.Contains(c))
            {
                sb.Append(vows[i]);
                i += 1;
            }
            else { sb.Append(c); }
        }
        return sb.ToString();
    }
}
