using System.Text;
using System.Text.RegularExpressions;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;


public class Solution
{
    public int MaximumGain(string s, int x, int y)
    {
        (char left, char right) = x > y ? ('a', 'b') : ('b', 'a');
        Stack<char> st = [];
        int res = 0;
        foreach (var ch in s)
        {
            if (ch == right && st.TryPeek(out var top) && top == left)
            {
                st.Pop();
                res += int.Max(x, y);
            }
            else { st.Push(ch); }
        }
        (left, right) = (right, left);
        Stack<char> st2 = [];
        foreach (var ch in st.Reverse())
        {
            if (ch == right && st2.TryPeek(out var top) && top == left)
            {
                st2.Pop();
                res += int.Min(x, y);
            }
            else { st2.Push(ch); }
        }
        return res;
    }

    public int WithCounting(string s, int x, int y)
    {
        if (x < y)
        {
            (x, y) = (y, x); // always starts with "ab"
            s = new([.. s.Reverse()]);
        }
        int a_count = 0;
        int b_count = 0;
        int res = 0;
        foreach (var ch in s)
        {
            switch (ch)
            {
                case 'a':
                    a_count += 1;
                    break;
                case 'b' when a_count > 0:
                    a_count -= 1;
                    res += x;
                    break;
                case 'b':
                    b_count += 1;
                    break;
                default: // compute all "ba"
                    res += int.Min(a_count, b_count) * y;
                    a_count = 0;
                    b_count = 0;
                    break;
            }
        }
        res += int.Min(a_count, b_count) * y;
        return res;
    }
}