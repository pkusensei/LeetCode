using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string ReverseParentheses(string s)
    {
        Stack<int> st = [];
        List<char> res = [];
        foreach (var c in s)
        {
            switch (c)
            {
                case '(':
                    st.Push(res.Count); break;
                case ')':
                    int top = st.Pop();
                    res.Reverse(top, res.Count - top);
                    break;
                default:
                    res.Add(c); break;
            }
        }
        return string.Concat(res);
    }
}