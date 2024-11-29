using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool ParseBoolExpr(string expression)
    {
        Stack<char> st = [];
        foreach (var ch in expression)
        {
            switch (ch)
            {
                case '&':
                case '|':
                case '!':
                case 't':
                case 'f':
                    st.Push(ch);
                    break;
                case ')':
                    List<bool> vals = [];
                    while (st.TryPeek(out var v) && char.IsLower(v))
                    {
                        vals.Add(st.Pop() == 't');
                    }
                    switch (st.Pop())
                    {
                        case '!':
                            st.Push(vals.Single() ? 'f' : 't');
                            break;
                        case '&':
                            st.Push(vals.All(b => b) ? 't' : 'f');
                            break;
                        case '|':
                            st.Push(vals.Any(b => b) ? 't' : 'f');
                            break;
                        default: break;
                    }
                    break;
                default:
                    break;
            }
        }
        return st.Single() == 't';
    }
}
