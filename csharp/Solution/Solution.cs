using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LengthLongestPath(string input)
    {
        Stack<(int tabs, int len)> st = [];
        int res = 0;
        foreach (var s in input.Split('\n'))
        {
            int tabs = s.TakeWhile(c => c == '\t').Count();
            while (st.TryPeek(out var top) && top.tabs >= tabs)
            {
                st.Pop();
            }
            st.Push((tabs, s.Length - tabs));
            if (s.Contains('.'))
            {
                int curr = st.Sum(v => v.len) + st.Count - 1;
                res = int.Max(res, curr);
            }
        }
        return res;
    }
}