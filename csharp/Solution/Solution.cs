using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool ValidateStackSequences(int[] pushed, int[] popped)
    {
        int n = pushed.Length;
        Stack<int> st = [];
        int i = 0;
        foreach (var item in pushed)
        {
            st.Push(item);
            while (st.TryPeek(out int top) && top == popped[i])
            {
                st.Pop();
                i += 1;
            }
        }
        return st.Count == 0 && i == n;
    }
}