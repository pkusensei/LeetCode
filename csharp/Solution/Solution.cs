using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CheckValidString(string s)
    {
        Stack<int> opens = [];
        Stack<int> stars = [];
        for (int i = 0; i < s.Length; i++)
        {
            switch (s[i])
            {
                case '(':
                    opens.Push(i);
                    break;
                case '*':
                    stars.Push(i);
                    break;
                default:
                    if (!opens.TryPop(out _) && !stars.TryPop(out _))
                    {
                        return false;
                    }
                    break;
            }
        }
        while (opens.TryPop(out var open))
        {
            if (!stars.TryPop(out var close) || close < open) { return false; }
        }
        return opens.Count == 0;
    }
}