using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestValidParentheses(string s)
    {
        bool[] bits = new bool[s.Length];
        Stack<int> st = [];
        for (int i = 0; i < s.Length; i++)
        {
            if (st.Count == 0 || s[i] == '(') { st.Push(i); }
            else if (st.TryPeek(out var top) && s[top] == '(')
            {
                st.Pop();
                bits[i] = true;
                bits[top] = true;
            }
            else { st.Push(i); }
        }
        int curr = 0;
        int res = 0;
        foreach (var item in bits)
        {
            if (item)
            {
                curr += 1;
                res = Math.Max(res, curr);
            }
            else { curr = 0; }
        }
        return res;
    }
}
