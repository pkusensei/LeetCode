using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string RemoveKdigits(string num, int k)
    {
        Stack<char> st = [];
        foreach (var c in num)
        {
            while (k > 0 && st.TryPeek(out var top) && top > c)
            {
                k -= 1;
                st.Pop();
            }
            st.Push(c);
        }
        for (int _ = 0; _ < k; _++)
        {
            st.Pop();
        }
        Queue<char> res = [];
        foreach (var c in st.Reverse())
        {
            res.Enqueue(c);
        }
        while (res.TryPeek(out var c) && c == '0')
        {
            res.Dequeue();
        }
        return res.Count == 0 ? "0" : new([.. res]);
    }
}
