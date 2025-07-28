using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string RemoveDuplicateLetters(string s)
    {
        Span<int> last_idx = stackalloc int[26];
        last_idx.Fill(-1);
        for (int i = 0; i < s.Length; i++)
        {
            last_idx[s[i] - 'a'] = i;
        }
        Stack<char> st = [];
        int mask = 0;
        for (int i = 0; i < s.Length; i++)
        {
            int ci = s[i] - 'a';
            if (((mask >> ci) & 1) == 1) { continue; }
            while (st.TryPeek(out var top) && top > s[i] & last_idx[top - 'a'] > i)
            {
                st.Pop();
                mask ^= 1 << (top - 'a');
            }
            st.Push(s[i]);
            mask |= 1 << ci;
        }
        return string.Concat(st.Reverse());
    }
}
