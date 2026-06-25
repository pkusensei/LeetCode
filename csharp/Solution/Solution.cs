using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string SmallestSubsequence(string s)
    {
        Span<int> freq = stackalloc int[26];
        foreach (var item in s)
        {
            freq[item - 'a'] += 1;
        }
        Span<bool> seen = stackalloc bool[26];
        Stack<char> st = [];
        foreach (var c in s)
        {
            freq[c - 'a'] -= 1;
            if (seen[c - 'a']) { continue; }
            while (st.TryPeek(out var top) && top > c && freq[top - 'a'] > 0)
            {
                st.Pop();
                seen[top - 'a'] = false;
            }
            st.Push(c);
            seen[c - 'a'] = true;
        }
        return new([.. st.Reverse()]);
    }
}
