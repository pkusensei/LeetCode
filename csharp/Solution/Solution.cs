using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool CanMakeSubsequence(string str1, string str2)
    {
        int i = 0;
        foreach (var c in str1)
        {
            if (c == str2[i] || c + 1 == str2[i] || (c == 'z' && str2[i] == 'a'))
            {
                i += 1;
            }
            if (i == str2.Length) { break; }
        }
        return i == str2.Length;
    }
}
