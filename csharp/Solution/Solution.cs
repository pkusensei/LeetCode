using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> ReplaceNonCoprimes(int[] nums)
    {
        Stack<int> st = [];
        foreach (var num in nums)
        {
            int curr = num;
            while (st.TryPeek(out var top) && GCD(top, curr) > 1)
            {
                st.Pop();
                curr = LCM(top, curr);
            }
            st.Push(curr);
        }
        return [.. st.Reverse()];

        static int GCD(int a, int b) => a == 0 ? b : GCD(b % a, a);
        static int LCM(int a, int b) => a / GCD(a, b) * b;
    }
}