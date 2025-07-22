using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Calculate(string s)
    {
        Stack<(int num, int sign)> st = [];
        int num = 0;
        int sign = 1;
        for (int i = 0; i < s.Length; i++)
        {
            switch (s[i])
            {
                case '-':
                    sign *= -1;
                    break;
                case '(':
                    st.Push((num, sign));
                    num = 0;
                    sign = 1;
                    break;
                case ')':
                    var item = st.Pop();
                    num = item.num + item.sign * num;
                    break;
                case char c when char.IsAsciiDigit(c):
                    int curr = c - '0';
                    while (1 + i < s.Length && char.IsAsciiDigit(s[1 + i]))
                    {
                        i += 1;
                        curr = curr * 10 + (s[i] - '0');
                    }
                    num += curr * sign;
                    sign = 1;
                    break;
                case '+':
                default: break;
            }
        }
        return num;
    }
}