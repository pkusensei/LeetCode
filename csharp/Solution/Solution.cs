using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;


public class Solution
{
    public int Calculate(string s)
    {
        Stack<int> st = [];
        int sign = 1;
        char? op = null;
        for (int i = 0; i < s.Length; i++)
        {
            switch (s[i])
            {
                case >= '0' and <= '9':
                    int curr = s[i] - '0';
                    while (i + 1 < s.Length && char.IsAsciiDigit(s[1 + i]))
                    {
                        i += 1;
                        curr = 10 * curr + s[i] - '0';
                    }
                    curr *= sign;
                    sign = 1;
                    if (op.HasValue)
                    {
                        int top = st.Pop();
                        if (op.Value == '*') { curr *= top; }
                        else { curr = top / curr; }
                        op = null;
                    }
                    st.Push(curr);
                    break;
                case '*':
                case '/':
                    op = s[i];
                    break;
                case '-':
                    sign *= -1;
                    break;
                case '+':
                default: break;
            }
        }
        return st.Sum();
    }
}