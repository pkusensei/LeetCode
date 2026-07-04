using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool ParseBoolExpr(string expression)
    {
        Stack<char> st = [];
        foreach (var item in expression)
        {
            switch (item)
            {
                case ')':
                    List<bool> vals = [];
                    while (st.TryPeek(out var top) && char.IsAsciiLetterLower(top))
                    {
                        st.Pop();
                        vals.Add(top == 't');
                    }
                    switch (st.Pop())
                    {
                        case '!':
                            st.Push(vals[0] ? 'f' : 't');
                            break;
                        case '&':
                            st.Push(vals.All(v => v) ? 't' : 'f');
                            break;
                        case '|':
                            st.Push(vals.Any(v => v) ? 't' : 'f');
                            break;
                        default:
                            break;
                    }
                    break;
                case ',':
                case '(':
                    break; // do nothing
                default:
                    st.Push(item);
                    break;
            }
        }
        return st.Pop() == 't';
    }
}
